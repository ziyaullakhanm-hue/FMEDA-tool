import { PieChart, Pie, Cell, Tooltip, Legend } from "recharts"

const data = [
  { name: "Project A", value: 3 },
  { name: "Project B", value: 5 },
  { name: "Project C", value: 2 },
]

const COLORS = ["#0088FE", "#00C49F", "#FFBB28"]

export default function ProjectPieChart() {
  return (
    <PieChart width={250} height={250}>
      <Pie
        data={data}
        cx="50%"
        cy="50%"
        labelLine={false}
        outerRadius={90}
        fill="#8884d8"
        dataKey="value"
        label
      >
        {data.map((_, index) => (
          <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
        ))}
      </Pie>
      <Tooltip />
      <Legend />
    </PieChart>
  )
}
