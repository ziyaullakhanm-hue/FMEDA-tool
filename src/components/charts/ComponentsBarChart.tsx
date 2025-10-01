import { BarChart, Bar, XAxis, YAxis, Tooltip, Legend } from "recharts"

const data = [
  { project: "Project A", resistors: 20, capacitors: 10, ics: 5 },
  { project: "Project B", resistors: 15, capacitors: 12, ics: 8 },
  { project: "Project C", resistors: 10, capacitors: 7, ics: 4 },
]

export default function ComponentsBarChart() {
  return (
    <BarChart width={300} height={250} data={data}>
      <XAxis dataKey="project" />
      <YAxis />
      <Tooltip />
      <Legend />
      <Bar dataKey="resistors" stackId="a" fill="#8884d8" />
      <Bar dataKey="capacitors" stackId="a" fill="#82ca9d" />
      <Bar dataKey="ics" stackId="a" fill="#ffc658" />
    </BarChart>
  )
}
