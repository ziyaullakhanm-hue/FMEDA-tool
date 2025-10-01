import ProjectPieChart from "./charts/ProjectPieChart"
import MetricsMatrix from "./charts/MetricsMatrix"
import ComponentsBarChart from "./charts/ComponentsBarChart"

export default function Dashboard() {
  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
      {/* Pie Chart */}
      <div className="bg-white p-4 rounded-2xl shadow">
        <h2 className="text-lg font-semibold mb-2">Projects Overview</h2>
        <ProjectPieChart />
      </div>

      {/* Matrix Chart */}
      <div className="bg-white p-4 rounded-2xl shadow">
        <h2 className="text-lg font-semibold mb-2">Safety Metrics</h2>
        <MetricsMatrix />
      </div>

      {/* Stacked Bar Chart */}
      <div className="bg-white p-4 rounded-2xl shadow">
        <h2 className="text-lg font-semibold mb-2">Components by Project</h2>
        <ComponentsBarChart />
      </div>
    </div>
  )
}
