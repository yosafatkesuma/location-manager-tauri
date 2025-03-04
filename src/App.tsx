import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type CoordinateType = { longitude: number | null; latitude: number | null };

function App() {
  const [coordinate, setCoordinate] = useState<CoordinateType | null>(null);

  useEffect(() => {
    (async () => {
      try {
        await invoke("init_location");
      } catch (error) {
        console.error(error);
      }
    })();
  }, []);

  async function requestPermission() {
    try {
      await invoke("request_location_permission");
    } catch (error) {
      console.error(error);
    }
  }

  async function getCoordination() {
    try {
      const checkPermission = await invoke("check_location_permission");
      if (checkPermission) {
        const locationCoordinate = (await invoke(
          "location_coor"
        )) as CoordinateType | null;
        setCoordinate(locationCoordinate);
      } else {
        await requestPermission();
      }
    } catch (error) {
      console.error(error);
    }
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <button type="button" onClick={requestPermission}>
        Request Permission
      </button>
      <button type="button" onClick={getCoordination}>
        Get Coordination
      </button>

      {coordinate ? (
        <div>
          <h1>Location</h1>
          <p>Longitude: {coordinate.longitude}</p>
          <p>Latitude: {coordinate.latitude}</p>
        </div>
      ) : (
        <p>Location Null</p>
      )}
    </main>
  );
}

export default App;
