
#[tauri::command]
fn encode_base64(text: &str) -> String {
    format!("{}", text)
}

#[tauri::command]
fn decode_base64(text: &str) -> String {
    format!("{}", text)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![encode_base64, decode_base64])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
