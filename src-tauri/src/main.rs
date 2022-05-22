#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use goblin_core::api::GoblinCore;
use goblin_core::session::Session;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
pub(crate) fn create_session(sessionname: String, addr: String, username: String, password: String) {
    let session = Session{
        name: sessionname,
        addr,
        username,
        password
    };
    GoblinCore::create_session(session);
}
