// src/pages/NewProject.tsx
import React, { useState } from "react"

const asilTargets: Record<string, { PFHM: string; LFM: string; SPFM: string }> = {
  QM: { PFHM: "Not applicable", LFM: "Not applicable", SPFM: "Not applicable" },
  A: { PFHM: "< 10^-5 /h", LFM: "≥ 60%", SPFM: "≥ 90%" },
  B: { PFHM: "< 10^-6 /h", LFM: "≥ 60%", SPFM: "≥ 90%" },
  C: { PFHM: "< 10^-7 /h", LFM: "≥ 90%", SPFM: "≥ 97%" },
  D: { PFHM: "< 10^-8 /h", LFM: "≥ 99%", SPFM: "≥ 99%" },
}

export default function NewProject() {
  const [project, setProject] = useState({
    name: "",
    number: "",
    asil: "QM",
    startDate: "",
    endDate: "",
    teamMembers: "",
  })

  const metrics = asilTargets[project.asil]

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    setProject({ ...project, [e.target.name]: e.target.value })
  }

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    console.log("New Project:", { ...project, metrics })
    alert("Project created successfully!")
  }

  return (
    <div className="max-w-2xl mx-auto bg-white shadow-md rounded-lg p-6">
      <h1 className="text-xl font-bold mb-4">New Project</h1>
      <form onSubmit={handleSubmit} className="space-y-4">
        {/* Program Name */}
        <input
          type="text"
          name="name"
          placeholder="Program Name"
          value={project.name}
          onChange={handleChange}
          className="w-full border p-2 rounded"
          required
        />

        {/* Program Number */}
        <input
          type="text"
          name="number"
          placeholder="Program Number"
          value={project.number}
          onChange={handleChange}
          className="w-full border p-2 rounded"
          required
        />

        {/* ASIL */}
        <select
          name="asil"
          value={project.asil}
          onChange={handleChange}
          className="w-full border p-2 rounded"
        >
          <option value="QM">QM</option>
          <option value="A">ASIL A</option>
          <option value="B">ASIL B</option>
          <option value="C">ASIL C</option>
          <option value="D">ASIL D</option>
        </select>

        {/* Dates */}
        <div className="flex gap-4">
          <input
            type="date"
            name="startDate"
            value={project.startDate}
            onChange={handleChange}
            className="flex-1 border p-2 rounded"
          />
          <input
            type="date"
            name="endDate"
            value={project.endDate}
            onChange={handleChange}
            className="flex-1 border p-2 rounded"
          />
        </div>

        {/* Team Members */}
        <input
          type="text"
          name="teamMembers"
          placeholder="Team Members (comma separated)"
          value={project.teamMembers}
          onChange={handleChange}
          className="w-full border p-2 rounded"
        />

        {/* Auto-Configured Metrics */}
        <div className="bg-gray-100 p-3 rounded text-sm">
          <p><b>PFHM:</b> {metrics.PFHM}</p>
          <p><b>LFM:</b> {metrics.LFM}</p>
          <p><b>SPFM:</b> {metrics.SPFM}</p>
        </div>

        <button
          type="submit"
          className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
        >
          Create Project
        </button>
      </form>
    </div>
  )
}
