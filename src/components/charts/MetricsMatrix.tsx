export default function MetricsMatrix() {
  const metrics = [
    { name: "SPFM", value: "95%" },
    { name: "LFM", value: "90%" },
    { name: "PMHF", value: "1e-8" },
  ]

  return (
    <div className="grid grid-cols-3 gap-4 text-center">
      {metrics.map((m) => (
        <div key={m.name} className="p-3 bg-gray-100 rounded-lg shadow">
          <h3 className="font-semibold">{m.name}</h3>
          <p className="text-blue-600 text-lg">{m.value}</p>
        </div>
      ))}
    </div>
  )
}
