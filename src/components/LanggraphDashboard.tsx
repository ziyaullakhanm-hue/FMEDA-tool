import React from 'react';
import { Link } from 'react-router-dom';

const LanggraphDashboard: React.FC = () => {
  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold">Dashboard</h1>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Quick Links */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">Quick Links</h2>
          <div className="flex gap-3">
            <Link
              to="/bom-upload"
              className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
            >
              Upload BOM
            </Link>
            <Link
              to="/fit-calculation"
              className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
            >
              Run Calculation
            </Link>
          </div>
        </div>

        {/* Compliance Summary */}
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">Compliance Summary</h2>
          <div className="bg-gray-50 border border-gray-200 rounded p-8 h-40 flex items-center justify-center text-gray-400">
            Chart Area
          </div>
        </div>
      </div>

      {/* Recent BOMs */}
      <div className="bg-white p-6 rounded-lg shadow">
        <h2 className="text-xl font-semibold mb-4">Recent BOMs</h2>
        <div className="bg-gray-50 border border-gray-200 rounded p-4 text-gray-500">
          Table or list of recent BOMs placeholder
        </div>
      </div>
    </div>
  );
};

export default LanggraphDashboard;
