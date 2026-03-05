import React from 'react';
import StatsCard from "./StatsCard"
import { 
  FolderOpen, 
  Cpu, 
  AlertTriangle, 
  Activity, 
  ShieldCheck, 
  ShieldAlert 
} from "lucide-react";

export default function Dashboard() {
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold tracking-tight">Dashboard</h1>
        <div className="flex items-center gap-2">
          <span className="text-sm text-muted-foreground">Last updated: Just now</span>
        </div>
      </div>

      {/* Top Metrics Row 1: Counts */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
        <StatsCard title="Active Projects" value="12" icon={FolderOpen} color="primary" />
        <StatsCard title="Total Components" value="324" icon={Cpu} color="primary" />
        <StatsCard title="Failure Modes" value="1,480" icon={AlertTriangle} color="warning" />
      </div>

      {/* Top Metrics Row 2: Safety Metrics */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
        <StatsCard title="Total FIT" value="32.5" subtitle="Failures in Time" icon={Activity} color="destructive" />
        <StatsCard title="SPFM" value="97.2%" subtitle="Single Point Fault Metric" icon={ShieldCheck} color="success" />
        <StatsCard title="LFM" value="91.4%" subtitle="Latent Fault Metric" icon={ShieldAlert} color="success" />
      </div>

      {/* Charts Section */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white p-6 rounded-xl border shadow-sm">
          <h3 className="text-lg font-semibold mb-4">FIT Distribution by Component</h3>
          <div className="h-64 flex items-end justify-around gap-2 px-4">
             {/* Simple CSS bar chart visualization for the example */}
             <div className="w-12 bg-blue-500 rounded-t" style={{ height: '40%' }} title="Resistor"></div>
             <div className="w-12 bg-blue-500 rounded-t" style={{ height: '60%' }} title="Capacitor"></div>
             <div className="w-12 bg-blue-500 rounded-t" style={{ height: '80%' }} title="IC"></div>
             <div className="w-12 bg-blue-500 rounded-t" style={{ height: '30%' }} title="Diode"></div>
             <div className="w-12 bg-blue-500 rounded-t" style={{ height: '50%' }} title="Connector"></div>
          </div>
          <div className="flex justify-around mt-2 text-xs text-gray-500">
            <span>Res</span><span>Cap</span><span>IC</span><span>Dio</span><span>Con</span>
          </div>
        </div>

        <div className="bg-white p-6 rounded-xl border shadow-sm">
          <h3 className="text-lg font-semibold mb-4">Failure Mode Classification</h3>
          <div className="h-64 flex items-center justify-center">
             {/* Placeholder for Pie Chart */}
             <div className="relative w-48 h-48 rounded-full border-8 border-slate-100 flex items-center justify-center">
                <div className="text-center">
                  <div className="text-2xl font-bold">1,480</div>
                  <div className="text-xs text-gray-500">Modes</div>
                </div>
                {/* CSS Conic Gradient for Pie */}
                <div className="absolute inset-0 rounded-full" style={{ background: 'conic-gradient(#3b82f6 0% 60%, #ef4444 60% 85%, #eab308 85% 100%)', opacity: 0.2 }}></div>
             </div>
          </div>
          <div className="flex justify-center gap-4 mt-4 text-sm">
            <div className="flex items-center gap-1"><div className="w-3 h-3 bg-blue-500 rounded-full"></div> Safe (60%)</div>
            <div className="flex items-center gap-1"><div className="w-3 h-3 bg-red-500 rounded-full"></div> SPF (25%)</div>
            <div className="flex items-center gap-1"><div className="w-3 h-3 bg-yellow-500 rounded-full"></div> Latent (15%)</div>
          </div>
        </div>
      </div>
    </div>
  )
}
