import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { info } from '@tauri-apps/plugin-log';
import "../styles/ConfigForm.css";

function ConfigForm({ onSave }) {
  const [apiKey, setApiKey] = useState("");
  const [defaultCity, setDefaultCity] = useState("");

  async function handleSave() {
    try {
      info("Calling the save_config function from JS");
      await invoke("save_config", { apiKey, defaultCity });
      info("Config was saved successfully and returned control to JS");
      onSave();
    } catch (error) {
      console.error("Failed to save config:", error);
    }
  }

  return (
    <div className="config-form">
      <h2>Enter API Key and Default City</h2>
      <div className="input-line">
        <input
          type="text"
          placeholder="Open Weather API Key"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
        />
      </div>
      <div className="input-line">
        <input
          type="text"
          placeholder="Default City"
          value={defaultCity}
          onChange={(e) => setDefaultCity(e.target.value)}
        />
      </div>

      <button className="submit-button" onClick={handleSave}>Save</button>
    </div>
  );
}

export default ConfigForm;
