import React, { useState } from 'react';
import { 
  LayoutDashboard, 
  FolderOpen, 
  Table2, 
  Activity, 
  BarChart4, 
  FileText, 
  Settings, 
  ChevronDown, 
  ChevronRight,
  Menu
} from "lucide-react";
import { Link, useLocation } from "react-router-dom";

interface SidebarProps {
  collapsed: boolean;
  setCollapsed: React.Dispatch<React.SetStateAction<boolean>>;
}

export default function Sidebar({ collapsed, setCollapsed }: SidebarProps) {
  const location = useLocation();
  const [expandedMenus, setExpandedMenus] = useState<Record<string, boolean>>({
    Projects: true,
    FMEDA: true,
    Reliability: false,
    Analysis: false,
    Reports: false,
    Admin: false
  });

  const toggleMenu = (name: string) => {
    setExpandedMenus(prev => ({ ...prev, [name]: !prev[name] }));
  };

  const navItems = [
    { name: "Dashboard", path: "/", icon: <LayoutDashboard size={20} /> },
    {
      name: "Projects",
      icon: <FolderOpen size={20} />,
      children: [
        { name: "Active Projects", path: "/projects" },
        { name: "Create Project", path: "/projects/new" },
      ],
    },
    {
      name: "FMEDA",
      icon: <Table2 size={20} />,
      children: [
        { name: "FMEDA Table", path: "/fmeda" },
        { name: "Failure Modes", path: "/fmeda/failure-modes" },
        { name: "Diagnostics", path: "/fmeda/diagnostics" },
      ],
    },
    {
      name: "Reliability",
      icon: <Activity size={20} />,
      children: [
        { name: "Component Database", path: "/reliability/components" },
        { name: "FIT Calculator", path: "/reliability/fit" },
        { name: "Mission Profiles", path: "/reliability/mission-profiles" },
      ],
    },
    {
      name: "Analysis",
      icon: <BarChart4 size={20} />,
      children: [
        { name: "Hardware Metrics", path: "/analysis/metrics" },
        { name: "SPFM / LFM", path: "/analysis/safety-metrics" },
        { name: "PMHF", path: "/analysis/pmhf" },
      ],
    },
    {
      name: "Reports",
      icon: <FileText size={20} />,
      children: [
        { name: "FMEDA Export", path: "/reports/export" },
        { name: "Safety Case", path: "/reports/safety-case" },
        { name: "Compliance Report", path: "/reports/compliance" },
      ],
    },
    {
      name: "Admin",
      icon: <Settings size={20} />,
      children: [
        { name: "User Management", path: "/admin/users" },
        { name: "Database Config", path: "/admin/db" },
      ],
    },
  ];

  return (
    <div
      className={`bg-slate-900 text-slate-300 flex flex-col transition-all duration-300 border-r border-slate-800 ${
        collapsed ? "w-16" : "w-64"
      }`}
    >
      {/* Toggle Area */}
      <div className="h-12 flex items-center justify-end px-4 border-b border-slate-800">
        <button
          onClick={() => setCollapsed(!collapsed)}
          className="p-1 hover:bg-slate-800 rounded text-slate-400 hover:text-white"
        >
          <Menu size={20} />
        </button>
      </div>

      {/* Navigation Links */}
      <nav className="flex-1 overflow-y-auto py-4">
        <ul className="space-y-1 px-2">
        {navItems.map((item) => {
            const hasChildren = item.children && item.children.length > 0;
            const isExpanded = expandedMenus[item.name];
          const isActive =
            location.pathname === item.path ||
                             (hasChildren && item.children?.some(c => c.path === location.pathname));

          return (
              <li key={item.name}>
                {hasChildren ? (
                  <div>
                <button
                      onClick={() => !collapsed && toggleMenu(item.name)}
                      className={`w-full flex items-center justify-between px-3 py-2 rounded-md transition-colors ${
                        isActive ? "text-white bg-slate-800" : "hover:text-white hover:bg-slate-800/50"
                  }`}
                      title={collapsed ? item.name : undefined}
                >
                  <div className="flex items-center gap-3">
                    {item.icon}
                        {!collapsed && <span className="text-sm font-medium">{item.name}</span>}
                  </div>
                      {!collapsed && (
                        isExpanded ? <ChevronDown size={14} /> : <ChevronRight size={14} />
                      )}
                </button>
                    
                    {!collapsed && isExpanded && (
                      <ul className="mt-1 ml-4 space-y-1 border-l border-slate-700 pl-2">
                        {item.children?.map((child) => (
                          <li key={child.path}>
                            <Link
                              to={child.path}
                              className={`block px-3 py-1.5 text-sm rounded-md transition-colors ${
                                location.pathname === child.path
                                  ? "text-blue-400 bg-blue-500/10"
                                  : "text-slate-400 hover:text-white hover:bg-slate-800/50"
                              }`}
                            >
                              {child.name}
                            </Link>
                          </li>
                        ))}
                      </ul>
                    )}
                  </div>
                ) : (
                <Link
                  to={item.path!}
                    className={`flex items-center gap-3 px-3 py-2 rounded-md transition-colors ${
                      location.pathname === item.path
                        ? "text-white bg-blue-600"
                        : "hover:text-white hover:bg-slate-800/50"
                  }`}
                    title={collapsed ? item.name : undefined}
                >
                  {item.icon}
                    {!collapsed && <span className="text-sm font-medium">{item.name}</span>}
                </Link>
              )}
              </li>
            );
        })}
        </ul>
      </nav>
    </div>
  );
}
