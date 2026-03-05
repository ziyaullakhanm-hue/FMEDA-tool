// src/App.tsx
// import Sidebar from "./components/Sidebar"
// import { Outlet } from "react-router-dom"

// export default function App() {
//   return (
//     <div className="flex h-screen w-screen bg-gray-100">
//       <Sidebar />

//       <div className="flex-1 flex flex-col">
//         {/* Optionally, a top header bar */}
//         <header className="h-16 bg-white border-b border-gray-200 px-4 flex items-center">
//           <h1 className="text-lg font-semibold">Dashboard</h1>
//         </header>

//         <main className="flex-1 overflow-y-auto p-4">
//           <Outlet />
//         </main>
//       </div>
//     </div>
//   )
// }

import Sidebar from "./components/Sidebar";
import Header from "./components/Header";
import { Outlet } from "react-router-dom";
import { useState } from "react";

export default function App() {
  const [collapsed, setCollapsed] = useState(false);

  return (
    <div className="flex flex-col h-screen bg-slate-50">
      <Header /> {/* Full width header */}
      <div className="flex flex-1 overflow-hidden">
        <Sidebar collapsed={collapsed} setCollapsed={setCollapsed} />
        <main className="flex-1 overflow-y-auto p-6">
           <div className="max-w-7xl mx-auto">
            <Outlet />
          </div>
        </main>
      </div>
    </div>
  );
}
