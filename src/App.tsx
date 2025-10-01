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

// src/App.tsx
import Sidebar from "./components/Sidebar";
import { Outlet } from "react-router-dom";

export default function App() {
  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 p-6 bg-gray-100 overflow-y-auto">
        <Outlet />
      </main>
    </div>
  );
}

