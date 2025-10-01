// src/main.tsx
import React from "react"
import ReactDOM from "react-dom/client"
import { createBrowserRouter, RouterProvider } from "react-router-dom"
import "./index.css"

// Layout
import App from "./App"   // App includes the sidebar

// Pages
import Home from "./pages/Home"
import Project from "./pages/Project"
import FTA from "./pages/FTA"
import NewProject from "./pages/NewProject"   // <-- Import New Project

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />, // Layout with sidebar
    children: [
      { index: true, element: <Home /> },                // renders at "/"
      { path: "project/:id", element: <Project /> },     // renders at "/project/:id"
      { path: "fta", element: <FTA /> },                 // renders at "/fta"
      { path: "new-project", element: <NewProject /> },  // <-- renders at "/new-project"
    ],
  },
])

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
)
