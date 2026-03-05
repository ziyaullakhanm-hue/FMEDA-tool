import React from "react"
import ReactDOM from "react-dom/client"
import { createBrowserRouter, RouterProvider } from "react-router-dom"
import "./index.css"

import App from "./App"

// Pages & Components
import Dashboard from "./components/Dashboard"
import ProjectsPage from "./pages/ProjectsPage"
import NewProject from "./pages/NewProject"
import ComponentsList from "./components/ComponentsList"
import FITDashboard from "./components/FITDashboard"
import MissionProfileEditor from "./components/MissionProfileEditor"
import ComplianceChecker from "./components/ComplianceChecker"
import Reports from "./components/Reports"
import Settings from "./components/Settings"
import ComponentsTable from "./components/ComponentsTable"

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    children: [
      { index: true, element: <Dashboard /> },
      
      // Projects
      { path: "projects", element: <ProjectsPage /> },
      { path: "projects/new", element: <NewProject /> },
      
      // FMEDA
      { path: "fmeda", element: <ComponentsTable /> }, // Placeholder for FMEDA Table
      { path: "fmeda/failure-modes", element: <ComponentsList /> }, // Placeholder
      { path: "fmeda/diagnostics", element: <div className="p-4">Diagnostics Placeholder</div> },
      
      // Reliability
      { path: "reliability/components", element: <ComponentsList /> },
      { path: "reliability/fit", element: <FITDashboard /> },
      { path: "reliability/mission-profiles", element: <MissionProfileEditor /> },
      
      // Analysis
      { path: "analysis/metrics", element: <ComplianceChecker /> },
      { path: "analysis/safety-metrics", element: <div className="p-4">SPFM/LFM Placeholder</div> },
      { path: "analysis/pmhf", element: <div className="p-4">PMHF Placeholder</div> },
      
      // Reports
      { path: "reports/export", element: <Reports /> },
      { path: "reports/safety-case", element: <div className="p-4">Safety Case Placeholder</div> },
      { path: "reports/compliance", element: <div className="p-4">Compliance Report Placeholder</div> },
      
      // Admin
      { path: "admin/users", element: <div className="p-4">User Management Placeholder</div> },
      { path: "admin/db", element: <Settings /> },
    ],
  },
])

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
)
