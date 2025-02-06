import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import ConfigForm from "./components/ConfigForm";

function App() {
  const [weather, setWeather] = useState("");
  const [city, setCityName] = useState("Kraków");
  const [isConfigured, setIsConfigured] = useState(false);

  useEffect(() => {
    async function checkConfig() {
      try {
        await invoke("load_config");
        setIsConfigured(true);
      } catch (error) {
        setIsConfigured(false);
      }
    }
    checkConfig();
  }, []);

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
    <div>
        {isConfigured ? (
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
                        value={city}
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
        ) : (
            <ConfigForm onSave={() => setIsConfigured(true)} />
        )}
    </div>
);
  
}

export default App;
