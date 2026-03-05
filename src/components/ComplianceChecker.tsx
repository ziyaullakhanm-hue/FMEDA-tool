import React from 'react';
import { Check, AlertCircle, X, FileDown } from 'lucide-react';

interface ComponentRow {
  id: number;
  name: string;
  type: string;
  status: 'pass' | 'warning' | 'fail';
}

const ComplianceChecker: React.FC = () => {
  const components: ComponentRow[] = [
    { id: 1, name: 'RES-001', type: 'Resistor', status: 'pass' },
    { id: 2, name: 'CAP-002', type: 'Capacitor', status: 'pass' },
    { id: 3, name: 'IC-003', type: 'Logic IC', status: 'warning' },
    { id: 4, name: 'DIO-004', type: 'Diode', status: 'fail' },
  ];

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'pass':
        return <Check className="w-5 h-5 text-green-600" />;
      case 'warning':
        return <AlertCircle className="w-5 h-5 text-yellow-600" />;
      case 'fail':
        return <X className="w-5 h-5 text-red-600" />;
      default:
        return null;
    }
  };

  return (
    <div className="bg-white p-6 rounded-lg shadow">
      <h1 className="text-3xl font-bold mb-6">Compliance Checker</h1>

      <div className="flex gap-3 mb-6 flex-wrap">
        <button className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">
          Check Single Component
        </button>
        <button className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">
          Batch Check
        </button>
        <button className="ml-auto bg-white border border-gray-300 text-gray-700 px-4 py-2 rounded hover:bg-gray-50 flex items-center gap-2">
          <FileDown className="w-4 h-4" />
          Export Report
        </button>
      </div>

      <div className="overflow-x-auto border border-gray-200 rounded">
        <table className="w-full">
          <thead className="bg-gray-50 border-b">
            <tr>
              <th className="px-6 py-3 text-left text-sm font-semibold">Component</th>
              <th className="px-6 py-3 text-left text-sm font-semibold">Type</th>
              <th className="px-6 py-3 text-left text-sm font-semibold">Status</th>
            </tr>
          </thead>
          <tbody>
            {components.map((comp) => (
              <tr key={comp.id} className="border-b hover:bg-gray-50">
                <td className="px-6 py-3">{comp.name}</td>
                <td className="px-6 py-3">{comp.type}</td>
                <td className="px-6 py-3 flex items-center gap-2">
                  {getStatusIcon(comp.status)}
                  <span className="capitalize">{comp.status}</span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default ComplianceChecker;
