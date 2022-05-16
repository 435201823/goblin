#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn save_session(name:String,addr:String) {
  println!("I was invoked from JS:{}",name);
  println!("I was invoked from JS:{}",addr);
}

//conn_name:String,addr:String,username:String,psw:String