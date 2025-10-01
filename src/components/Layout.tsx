// src/components/Layout.tsx
import React, { useState } from "react";

interface LayoutProps {
  children: React.ReactNode;
}

export default function Layout({ children }: LayoutProps) {
  const [isOpen, setIsOpen] = useState(true);

  return (
    <div className="flex h-screen">
      {/* Sidebar */}
      <div
        className={`bg-gray-800 text-white transition-all duration-300 ${
          isOpen ? "w-64" : "w-16"
        }`}
      >
        <div className="flex items-center justify-between p-4 border-b border-gray-700">
          {isOpen && <span className="text-xl font-bold">FMEDA Tool</span>}
          <button
            onClick={() => setIsOpen(!isOpen)}
            className="text-white focus:outline-none"
          >
            {isOpen ? "⬅️" : "➡️"}
          </button>
        </div>

        <ul className="mt-4">
          <li className="px-4 py-2 hover:bg-gray-700">Dashboard</li>
          <li className="px-4 py-2 hover:bg-gray-700">Projects</li>
          <li className="px-4 py-2 hover:bg-gray-700">Analysis</li>
        </ul>
      </div>

      {/* Main Content */}
      <main className="flex-1 bg-gray-100 p-6 overflow-auto">{children}</main>
    </div>
  );
}
