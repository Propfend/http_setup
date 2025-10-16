import React from "react";
import App from "./components/App";
import { createRoot } from "react-dom/client";
// import registerServiceWorker from "./registerServiceWorker";

const rootNodeElement = document.getElementById("root");

if (!rootNodeElement) {
  throw new Error("Root node not found");
}

// void registerServiceWorker();

const root = createRoot(rootNodeElement);
  
root.render(<App />);
