#[tauri::command]
fn load_forecast(city_name: &str) -> String {
    format!("Weather in {} is sunny now!", city_name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_forecast])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
