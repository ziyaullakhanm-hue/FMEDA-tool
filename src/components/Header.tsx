import React from 'react';
import { Search, Bell, Settings, User, ShieldCheck } from 'lucide-react';

export default function Header() {
  return (
    <header className="h-16 bg-white border-b border-slate-200 flex items-center justify-between px-4 z-10">
      {/* Left: Logo & Title */}
      <div className="flex items-center gap-3 w-64">
        <div className="bg-blue-600 p-1.5 rounded-lg">
          <ShieldCheck className="text-white w-6 h-6" />
        </div>
        <span className="text-xl font-bold text-slate-900 tracking-tight">SafeCrate</span>
      </div>

      {/* Center: Search */}
      <div className="flex-1 max-w-2xl px-8">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 w-4 h-4" />
          <input 
            type="text" 
            placeholder="Search components, projects, failure modes..." 
            className="w-full pl-10 pr-4 py-2 bg-slate-100 border-none rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:bg-white transition-all"
          />
        </div>
      </div>

      {/* Right: Actions */}
      <div className="flex items-center gap-4">
        <button className="p-2 text-slate-500 hover:bg-slate-100 rounded-full relative">
          <Bell className="w-5 h-5" />
          <span className="absolute top-1.5 right-1.5 w-2 h-2 bg-red-500 rounded-full border-2 border-white"></span>
        </button>
        <button className="p-2 text-slate-500 hover:bg-slate-100 rounded-full">
          <Settings className="w-5 h-5" />
        </button>
        <div className="h-8 w-px bg-slate-200 mx-1"></div>
        <button className="flex items-center gap-2 p-1 pr-3 hover:bg-slate-50 rounded-full border border-transparent hover:border-slate-200 transition-all">
          <div className="w-8 h-8 bg-slate-900 rounded-full flex items-center justify-center text-white text-xs font-medium">
            JD
          </div>
          <div className="text-left hidden sm:block">
            <p className="text-xs font-semibold text-slate-700">John Doe</p>
            <p className="text-[10px] text-slate-500">Safety Engineer</p>
          </div>
        </button>
      </div>
    </header>
  );
}
