use tauri::{LogicalSize, Manager};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Weather {
    main: Main,
    weather: Vec<WeatherDescription>,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
}

#[derive(Debug, Deserialize)]
struct WeatherDescription {
    description: String,
}

#[tauri::command]
async fn get_weather(city: String) -> Result<String, String> {
    let api_key = "your_api_key"; // Replace with your actual API key
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        city, api_key
    );

    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let weather: Weather = response.json().await.map_err(|e| e.to_string())?;

    let result = format!(
        "The weather in {} is {:.0}°C With {}",
        city, weather.main.temp, weather.weather[0].description
    );

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_weather])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_size(tauri::Size::Logical(LogicalSize::new(400., 250.))).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
