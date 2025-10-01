// src/pages/FTA.tsx
import React from "react"

export default function FTA() {
  return (
    <div className="w-full h-screen bg-gray-100 p-4">
      <h1 className="text-2xl font-bold mb-4">Fault Tree Analysis (FTA)</h1>

      <iframe
        src="https://www.yworks.com/yed-live/"
        title="yEd Live Editor"
        width="100%"
        height="90%"
        style={{ border: "1px solid #ccc", borderRadius: "6px" }}
      />
    </div>
  )
}
