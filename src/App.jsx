import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [forecast, setCityForecast] = useState("");
  const [cityName, setCityName] = useState("");

  async function city_select() {
    setCityForecast(await invoke("load_forecast", { cityName }));
  }

  return (
    <main className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          city_select();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setCityName(e.currentTarget.value)}
          placeholder="City name..."
        />
        <button type="submit">Select</button>
      </form>
      <p>{forecast}</p>
    </main>
  );
}

export default App;
