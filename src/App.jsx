import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [weather, setWeather] = useState("");
  const [city, setCityName] = useState("Kraków");

  async function fetchWeather() {
    try {
      const result = await invoke("get_weather", { city });
      setWeather(result);
    } catch (error) {
      console.error("Error fetching weather:", error);
      setWeather("Failed to fetch weather.");
    }
  }

  return (
    <main className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          fetchWeather();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setCityName(e.currentTarget.value)}
          placeholder="City name..."
        />
        <button type="submit">Select</button>
      </form>
      <div>
        {weather.split(" | ").map((line, index) => (
          <p key={index}>{line}</p>
        ))}
      </div>
    </main>
  );
}

export default App;
