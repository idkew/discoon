# Discoon
Stealer with webhook protection.

### Educational purposes only
Only use this on your own machine and do not use it maliciously. 

### License
This project is licensed under the terms of the MIT license.

### How it works
This is a malware written in Rust that steals the information the attacker chooses to steal and sends that through a server to the attacker's webhook.

### Features
- Grab IP Address (`backend grabs the IP address`)
- Anti analysis (`detects some malware analysis environments`)
- Webhook protection (`sends to webhook through a backend making your webhook protected`)
- Trace token (`sends new user token when they change user data also steals credit cards and login information`)
- Steal discord tokens (`steal and decrypt discord tokens`)
- Steal browser passwords (`steals browser passwords`)
- Steal browser cookies (`steals browser cookies`)
- Steal browsing history (`steals browsing history`)
- Steal credit cards (`steals browser credit cards`)
- Take screenshot (`takes a screenshot`)
- Take webcam image (`takes a webcam image`)

### How to use
1. Make sure you have rust installed
2. Open it in VS Code or your preferred IDE
3. Goto `assets/upload.php` and set the `$webhook` to yours
4. Get a webhost you can get a free one from [here](https://www.000webhost.com/)
5. Get the `assets/upload.php` and upload it to your webhost
6. Change the `BACKEND` in `src/constants.rs` to yours
7. Set the options you want in `constants.rs`
8. Run (x64) `cargo build --release` or (x86) `cargo build --release --target=i686-pc-windows-msvc`

### Contributing
1. Fork it
2. Create your branch (`git checkout -b my-change`)
3. Commit your changes (`git commit -am 'changed something'`)
4. Push to the branch (`git push origin my-change`)
5. Create new pull request
