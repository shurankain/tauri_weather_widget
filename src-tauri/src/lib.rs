use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{LogicalSize, Manager};
use tauri_plugin_log::{Target, TargetKind};

#[derive(Debug, Deserialize)]
struct Weather {
    main: Main,
    weather: Vec<WeatherDescription>,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
}

#[derive(Debug, Deserialize)]
struct WeatherDescription {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    api_key: String,
    default_city: String,
}

#[tauri::command]
async fn save_config(api_key: String, default_city: String) -> Result<(), String> {
    let config = Config {
        api_key,
        default_city,
    };
    let config_json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write("config.json", config_json).map_err(|e| e.to_string())?;
    Ok(())
}

fn load_config() -> Result<Config, String> {
    let config_data = fs::read_to_string("config.json").map_err(|_| "Config not found")?;
    serde_json::from_str(&config_data).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_weather(city: String) -> Result<String, String> {
    let api_key = load_config()?.api_key;
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        city, api_key
    );

    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let weather: Weather = response.json().await.map_err(|e| e.to_string())?;

    let result = format!(
        "The weather in {}: | Temperature: {:.0}°C | Feels like {:.0} | With {}",
        city, weather.main.temp, weather.main.feels_like, weather.weather[0].description
    );

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_weather])
        .invoke_handler(tauri::generate_handler![save_config])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window
                .set_size(tauri::Size::Logical(LogicalSize::new(400., 300.)))
                .unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
