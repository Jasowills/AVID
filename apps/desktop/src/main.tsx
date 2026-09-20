import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { HashRouter } from "react-router-dom";
import { App } from "./App";
import "./index.css";

// HashRouter: file:// + Tauri webview safe (no server rewrites needed).
const root = document.getElementById("root");
if (!root) throw new Error("AVID: #root element missing");
createRoot(root).render(
  <StrictMode>
    <HashRouter>
      <App />
    </HashRouter>
  </StrictMode>,
);
