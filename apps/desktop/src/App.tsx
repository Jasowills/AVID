import { Route, Routes } from "react-router-dom";
import { Toaster } from "./components/Toaster";
import { Home } from "./pages/Home";
import { NewProject } from "./pages/NewProject";
import { Editor } from "./pages/Editor";
import { Settings } from "./pages/Settings";

/** Route map (Phase 1.3): home / new-project / editor / settings. */
export function App() {
  return (
    <div className="flex h-full flex-col bg-avid-base text-avid-primary">
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/projects/new" element={<NewProject />} />
        <Route path="/editor/:id" element={<Editor />} />
        <Route path="/settings" element={<Settings />} />
        <Route path="*" element={<Home />} />
      </Routes>
      <Toaster />
    </div>
  );
}
