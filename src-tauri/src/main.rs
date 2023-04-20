#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn encode_base64(text: &str) -> String {
    format!("{}", text)
}

#[tauri::command]
fn decode_base64(text: &str) -> String {
    format!("{}", text)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![encode_base64, decode_base64])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
