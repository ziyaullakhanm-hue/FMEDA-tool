import React, { useState } from 'react';
import { BarChart3, PieChart as PieChartIcon, Download, Filter } from 'lucide-react';

const Reports: React.FC = () => {
  const [filterType, setFilterType] = useState('all');
  const [filterStatus, setFilterStatus] = useState('all');

  const complianceSummary = {
    pass: 45,
    warning: 23,
    fail: 12,
  };

  const fitSummary = {
    low: 20,
    medium: 35,
    high: 25,
    critical: 20,
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">Reports</h1>
        <button className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 flex items-center gap-2">
          <Download className="w-4 h-4" />
          Export to PDF
        </button>
      </div>

      {/* Filters */}
      <div className="bg-white p-4 rounded-lg shadow flex gap-4 items-center flex-wrap">
        <Filter className="w-5 h-5 text-gray-600" />
        
        <div>
          <label className="text-sm font-semibold text-gray-700 block mb-1">Component Type</label>
          <select
            value={filterType}
            onChange={(e) => setFilterType(e.target.value)}
            className="border border-gray-300 rounded px-3 py-1 text-sm focus:outline-none focus:border-blue-500"
          >
            <option value="all">All Types</option>
            <option value="resistor">Resistor</option>
            <option value="capacitor">Capacitor</option>
            <option value="ic">Integrated Circuit</option>
            <option value="diode">Diode</option>
          </select>
        </div>

        <div>
          <label className="text-sm font-semibold text-gray-700 block mb-1">Status</label>
          <select
            value={filterStatus}
            onChange={(e) => setFilterStatus(e.target.value)}
            className="border border-gray-300 rounded px-3 py-1 text-sm focus:outline-none focus:border-blue-500"
          >
            <option value="all">All Status</option>
            <option value="pass">Pass</option>
            <option value="warning">Warning</option>
            <option value="fail">Fail</option>
          </select>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Compliance Summary Chart */}
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="flex items-center gap-2 mb-6">
            <PieChartIcon className="w-5 h-5 text-blue-600" />
            <h2 className="text-xl font-semibold">Compliance Summary</h2>
          </div>
          
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 bg-green-500 rounded-full"></div>
                <span className="text-gray-700">Pass</span>
              </div>
              <span className="font-semibold text-green-600">{complianceSummary.pass}</span>
            </div>
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 bg-yellow-500 rounded-full"></div>
                <span className="text-gray-700">Warning</span>
              </div>
              <span className="font-semibold text-yellow-600">{complianceSummary.warning}</span>
            </div>
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 bg-red-500 rounded-full"></div>
                <span className="text-gray-700">Fail</span>
              </div>
              <span className="font-semibold text-red-600">{complianceSummary.fail}</span>
            </div>
            <div className="h-2 bg-gray-200 rounded-full overflow-hidden mt-4">
              <div className="h-full flex">
                <div 
                  className="bg-green-500" 
                  style={{ width: `${(complianceSummary.pass / (complianceSummary.pass + complianceSummary.warning + complianceSummary.fail)) * 100}%` }}
                ></div>
                <div 
                  className="bg-yellow-500" 
                  style={{ width: `${(complianceSummary.warning / (complianceSummary.pass + complianceSummary.warning + complianceSummary.fail)) * 100}%` }}
                ></div>
                <div className="bg-red-500 flex-1"></div>
              </div>
            </div>
          </div>
        </div>

        {/* FIT Range Distribution */}
        <div className="bg-white p-6 rounded-lg shadow">
          <div className="flex items-center gap-2 mb-6">
            <BarChart3 className="w-5 h-5 text-blue-600" />
            <h2 className="text-xl font-semibold">FIT Range Distribution</h2>
          </div>
          
          <div className="space-y-4">
            {[
              { label: 'Low (< 10)', value: fitSummary.low, color: 'bg-green-500' },
              { label: 'Medium (10-50)', value: fitSummary.medium, color: 'bg-yellow-500' },
              { label: 'High (50-100)', value: fitSummary.high, color: 'bg-orange-500' },
              { label: 'Critical (> 100)', value: fitSummary.critical, color: 'bg-red-500' },
            ].map((item, idx) => (
              <div key={idx}>
                <div className="flex justify-between mb-1">
                  <span className="text-sm font-medium text-gray-700">{item.label}</span>
                  <span className="text-sm font-semibold text-gray-900">{item.value}</span>
                </div>
                <div className="w-full bg-gray-200 rounded-full h-2">
                  <div 
                    className={`${item.color} h-2 rounded-full`} 
                    style={{ width: `${(item.value / 100) * 100}%` }}
                  ></div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Detailed Components Table */}
      <div className="bg-white p-6 rounded-lg shadow">
        <h2 className="text-xl font-semibold mb-4">Component Compliance Details</h2>
        <div className="overflow-x-auto border border-gray-200 rounded">
          <table className="w-full">
            <thead className="bg-gray-50 border-b">
              <tr>
                <th className="px-4 py-3 text-left text-sm font-semibold">Component Type</th>
                <th className="px-4 py-3 text-left text-sm font-semibold">Subtype</th>
                <th className="px-4 py-3 text-left text-sm font-semibold">Count</th>
                <th className="px-4 py-3 text-left text-sm font-semibold">Pass Rate</th>
                <th className="px-4 py-3 text-left text-sm font-semibold">Avg FIT</th>
              </tr>
            </thead>
            <tbody>
              {[
                { type: 'Resistor', subtype: 'Thin Film', count: 25, pass: 96, fit: 0.45 },
                { type: 'Capacitor', subtype: 'Ceramic', count: 18, pass: 94, fit: 2.3 },
                { type: 'IC', subtype: 'Logic', count: 12, pass: 82, fit: 8.5 },
                { type: 'Diode', subtype: 'Zener', count: 8, pass: 75, fit: 5.2 },
                { type: 'Inductor', subtype: 'SMD', count: 10, pass: 90, fit: 1.8 },
              ].map((item, idx) => (
                <tr key={idx} className="border-b hover:bg-gray-50">
                  <td className="px-4 py-3">{item.type}</td>
                  <td className="px-4 py-3">{item.subtype}</td>
                  <td className="px-4 py-3">{item.count}</td>
                  <td className="px-4 py-3">
                    <div className="flex items-center gap-2">
                      <div className="w-12 h-2 bg-gray-200 rounded-full overflow-hidden">
                        <div 
                          className="bg-green-500 h-full" 
                          style={{ width: `${item.pass}%` }}
                        ></div>
                      </div>
                      <span className="text-sm font-medium">{item.pass}%</span>
                    </div>
                  </td>
                  <td className="px-4 py-3 font-mono">{item.fit}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

export default Reports;
