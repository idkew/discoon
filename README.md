# Discoon
FUD malware with a backend written in Rust

## Educational purposes only
This demonstrates how malware can be created in Rust. Making malware in a language where it's not usual means lower detection rates since the signature is different from lot of popular malware. **Only use this on your own machine and do not use it on other people maliciously**. 

# Terms
- [x] You're free to use this code if you credit the original repository
- [x] I'm not responsible for anything you do with this

### Features
- Grab IP Address (`backend grabs the IP address`)
- Anti analysis (`detects some malware analysis environments`)
- Webhook protection (`sends to webhook through a backend making your webhook protected`)
- Trace token (`sends new user token when they change user data also steals credit cards and login information`)
- Steal discord tokens (`steal and decrypt discord tokens`)
- Steal browser passwords (`steals browser passwords`)
- Steal browser cookies (`steals browser cookies`)
- Steal browsing history (`steals browsing history`)
- Take screenshot (`takes a screenshot`)
- Take webcam image (`takes a webcam image`)
- Fully undetectable (`this is fully undetectable by anti viruses for now`)

### How to use
1. Open it in VS Code or your preferred IDE
2. Goto `constants.rs` and find the `WEBHOOK` field
3. Set the webhook to your webhook
4. Make sure to change the encryption key and iv in `encryption-macro/src/lib.rs` and the `upload.php` on your backend
5. Make sure to change the backend since the default one will probably get banned at some point
6. Set the options in `constants.rs`
7. Run (x64) `cargo build --release` or (x86) `cargo build --release --target=i686-pc-windows-msvc`

### Contributing
1. Fork it
2. Create your branch (`git checkout -b my-change`)
3. Commit your changes (`git commit -am 'changed something'`)
4. Push to the branch (`git push origin my-change`)
5. Create new pull request
