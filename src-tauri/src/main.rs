// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub use vrchatapi::apis;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn see_online_players(username: &str) -> Result<String, ()> {
    let config = apis::configuration::Configuration::default();

    let online = apis::system_api::get_current_online_users(&config).unwrap();
    Ok(format!("Hello {}, There are {} current players online on VRChat!", username, online))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![see_online_players])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
