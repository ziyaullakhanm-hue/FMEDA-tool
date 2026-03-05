import React from 'react';
import { Check, AlertCircle, X } from 'lucide-react';

interface Component {
  id: number;
  type: string;
  subtype: string;
  mpn: string;
  fit: number;
  compliance: 'Pass' | 'Warning' | 'Fail';
}

const ComponentsTable: React.FC = () => {
  const rows: Component[] = [
    { id: 1, type: 'Resistor', subtype: 'Thin Film', mpn: 'RES-001-TF', fit: 0.35, compliance: 'Pass' },
    { id: 2, type: 'Capacitor', subtype: 'Ceramic', mpn: 'CAP-002-C', fit: 4.34, compliance: 'Pass' },
    { id: 3, type: 'IC', subtype: 'Logic', mpn: 'IC-003-L', fit: 12.1, compliance: 'Warning' },
    { id: 4, type: 'Diode', subtype: 'Zener', mpn: 'DIO-004-Z', fit: 2.5, compliance: 'Fail' },
  ];

  const getComplianceIcon = (compliance: string) => {
    switch (compliance) {
      case 'Pass':
        return <Check className="w-5 h-5 text-green-600" />;
      case 'Warning':
        return <AlertCircle className="w-5 h-5 text-yellow-600" />;
      case 'Fail':
        return <X className="w-5 h-5 text-red-600" />;
      default:
        return null;
    }
  };

  return (
    <div className="bg-white p-6 rounded-lg shadow">
      <h1 className="text-3xl font-bold mb-6">Component List</h1>
      
      <div className="overflow-x-auto border border-gray-200 rounded">
        <table className="w-full">
          <thead className="bg-gray-50 border-b">
            <tr>
              <th className="px-4 py-3 text-left text-sm font-semibold">Component Type</th>
              <th className="px-4 py-3 text-left text-sm font-semibold">Subtype</th>
              <th className="px-4 py-3 text-left text-sm font-semibold">MPN</th>
              <th className="px-4 py-3 text-left text-sm font-semibold">FIT</th>
              <th className="px-4 py-3 text-left text-sm font-semibold">Compliance</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={row.id} className="border-b hover:bg-gray-50">
                <td className="px-4 py-3">{row.type}</td>
                <td className="px-4 py-3">{row.subtype}</td>
                <td className="px-4 py-3 font-mono text-sm">{row.mpn}</td>
                <td className="px-4 py-3">{row.fit}</td>
                <td className="px-4 py-3 flex items-center gap-2">
                  {getComplianceIcon(row.compliance)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default ComponentsTable;
