import Dashboard from "../components/Dashboard"
import { Link } from "react-router-dom"

export default function Home() {
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-4">FMEDA Tool Dashboard</h1>

      {/* Add New Project Button */}
      <Link to="/new-project">
        <button className="mb-6 px-4 py-2 bg-blue-600 text-white rounded-lg shadow hover:bg-blue-700">
          + Add New Project
        </button>
      </Link>

      {/* Dashboard with project list or charts */}
      <Dashboard />
    </div>
  )
}
