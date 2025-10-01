// src/components/Sidebar.tsx
import React from 'react'
import { Home, PlusCircle, FileText, Settings, ChevronDown, ChevronUp } from "lucide-react";
import { Link, useLocation } from "react-router-dom";

interface SidebarProps {
  collapsed: boolean;
  setCollapsed: React.Dispatch<React.SetStateAction<boolean>>;
}

export default function Sidebar({ collapsed, setCollapsed }: SidebarProps) {
  const location = useLocation();
  const [ftaOpen, setFtaOpen] = React.useState(false);

  const navItems = [
    { name: "Home", path: "/", icon: <Home size={20} /> },
    { name: "New Project", path: "/new-project", icon: <PlusCircle size={20} /> },
    {
      name: "FTA",
      path: "/fta",
      icon: <FileText size={20} />,
      children: [
        { name: "View FTA", path: "/fta/view" },
        { name: "Create FTA", path: "/fta/create" },
      ],
    },
  ];

  return (
    <div
      className={`fixed top-0 left-0 h-screen bg-gray-900 text-white flex flex-col transition-all duration-300 z-50 shadow-lg ${
        collapsed ? "w-20" : "w-64"
      }`}
    >
      {/* Logo + Collapse Button */}
      <div className="flex items-center justify-between p-4 border-b border-gray-700">
        {!collapsed && <span className="text-lg font-bold">SafeCrate</span>}
        <button
          onClick={() => setCollapsed(!collapsed)}
          className="text-gray-400 hover:text-white focus:outline-none"
        >
          {collapsed ? "»" : "«"}
        </button>
      </div>

      {/* Navigation Links */}
      <nav className="flex-1 overflow-y-auto mt-2">
        {navItems.map((item) => {
          const isActive =
            location.pathname === item.path ||
            (item.children && item.children.some((c) => c.path === location.pathname));

          return (
            <div key={item.path}>
              <button
                onClick={() => item.children && setFtaOpen(!ftaOpen)}
                className={`flex items-center justify-between gap-3 w-full px-4 py-2 rounded-lg mb-2 transition-colors ${
                  isActive
                    ? "bg-gray-700 text-white"
                    : "text-gray-400 hover:bg-gray-800 hover:text-white"
                }`}
              >
                <div className="flex items-center gap-3">
                  {item.icon}
                  {!collapsed && <span>{item.name}</span>}
                </div>
                {!collapsed && item.children && (ftaOpen ? <ChevronUp size={16} /> : <ChevronDown size={16} />)}
              </button>

              {/* Sub-menu */}
              {item.children && ftaOpen && !collapsed && (
                <div className="pl-12">
                  {item.children.map((child) => (
                    <Link
                      key={child.path}
                      to={child.path}
                      className={`block px-4 py-2 rounded-lg mb-1 text-sm transition-colors ${
                        location.pathname === child.path
                          ? "bg-gray-700 text-white"
                          : "text-gray-400 hover:bg-gray-800 hover:text-white"
                      }`}
                    >
                      {child.name}
                    </Link>
                  ))}
                </div>
              )}
            </div>
          );
        })}
      </nav>

      {/* Bottom Section */}
      <div className="p-4 border-t border-gray-700">
        <Link
          to="/settings"
          className={`flex items-center gap-3 px-4 py-2 rounded-lg transition-colors ${
            location.pathname === "/settings"
              ? "bg-gray-700 text-white"
              : "text-gray-400 hover:bg-gray-800 hover:text-white"
          }`}
        >
          <Settings size={20} />
          {!collapsed && <span>Settings</span>}
        </Link>
      </div>
    </div>
  );
}
