import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { HashRouter } from "react-router-dom";
import { App } from "./App";
import { isTauri, setInvoker } from "./lib/ipc";
import "./index.css";

// Wire the Tauri invoker when inside the webview; the browser keeps the
// explicit NOT_IN_TAURI behavior (see lib/ipc.ts).
if (isTauri()) {
  import("@tauri-apps/api/core")
    .then(({ invoke }) => setInvoker(invoke))
    .catch((error: unknown) => {
      console.error("AVID: failed to load Tauri API", error);
    });
}

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
