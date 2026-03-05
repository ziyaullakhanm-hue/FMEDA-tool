import { useState, useEffect } from 'react';
import { Plus, Edit2, Trash2, BarChart3, Upload } from 'lucide-react';

interface Component {
  id: string;
  manufacturer_part_number: string;
  manufacturer?: string;
  component_type: string;
  reference_designator?: string;
  quantity: number;
  fit?: number;
}

export default function ComponentManagement() {
  const [components, setComponents] = useState<Component[]>([]);
  const [loading, setLoading] = useState(false);
  const [showForm, setShowForm] = useState(false);
  const [formData, setFormData] = useState({
    mpn: '',
    manufacturer: '',
    type: 'Resistor',
    designator: '',
    quantity: 1,
  });

  useEffect(() => {
    fetchComponents();
  }, []);

  const fetchComponents = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/components');
      const data = await response.json();
      setComponents(data || []);
    } catch (error) {
      console.error('Failed to fetch components:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleAddComponent = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch('/api/components', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          manufacturer_part_number: formData.mpn,
          manufacturer: formData.manufacturer,
          component_type: formData.type,
          reference_designator: formData.designator,
          quantity: formData.quantity,
        }),
      });

      if (response.ok) {
        fetchComponents();
        setShowForm(false);
        setFormData({
          mpn: '',
          manufacturer: '',
          type: 'Resistor',
          designator: '',
          quantity: 1,
        });
      }
    } catch (error) {
      console.error('Failed to add component:', error);
    }
  };

  const handleDelete = async (id: string) => {
    if (!window.confirm('Delete this component?')) return;
    try {
      await fetch(`/api/components/${id}`, { method: 'DELETE' });
      fetchComponents();
    } catch (error) {
      console.error('Failed to delete component:', error);
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Component Management</h1>
          <p className="text-gray-600 mt-1">Manage and calculate FIT for components</p>
        </div>
        <button
          onClick={() => setShowForm(!showForm)}
          className="flex items-center gap-2 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition"
        >
          <Plus size={20} /> Add Component
        </button>
      </div>

      {/* Add Component Form */}
      {showForm && (
        <div className="bg-white rounded-lg shadow-md p-6">
          <h2 className="text-xl font-semibold mb-4">Add New Component</h2>
          <form onSubmit={handleAddComponent} className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <input
              type="text"
              placeholder="Part Number (MPN)"
              required
              value={formData.mpn}
              onChange={(e) => setFormData({ ...formData, mpn: e.target.value })}
              className="border border-gray-300 rounded px-3 py-2"
            />
            <input
              type="text"
              placeholder="Manufacturer"
              value={formData.manufacturer}
              onChange={(e) => setFormData({ ...formData, manufacturer: e.target.value })}
              className="border border-gray-300 rounded px-3 py-2"
            />
            <select
              value={formData.type}
              onChange={(e) => setFormData({ ...formData, type: e.target.value })}
              className="border border-gray-300 rounded px-3 py-2"
            >
              <option value="Resistor">Resistor</option>
              <option value="Capacitor">Capacitor</option>
              <option value="IC">IC</option>
              <option value="Diode">Diode</option>
            </select>
            <input
              type="text"
              placeholder="Reference Designator (e.g., R1, C1)"
              value={formData.designator}
              onChange={(e) => setFormData({ ...formData, designator: e.target.value })}
              className="border border-gray-300 rounded px-3 py-2"
            />
            <input
              type="number"
              placeholder="Quantity"
              min="1"
              value={formData.quantity}
              onChange={(e) => setFormData({ ...formData, quantity: parseInt(e.target.value) })}
              className="border border-gray-300 rounded px-3 py-2"
            />
            <div className="flex gap-2">
              <button
                type="submit"
                className="flex-1 bg-green-600 text-white rounded px-4 py-2 hover:bg-green-700 transition font-medium"
              >
                Add
              </button>
              <button
                type="button"
                onClick={() => setShowForm(false)}
                className="flex-1 bg-gray-400 text-white rounded px-4 py-2 hover:bg-gray-500 transition font-medium"
              >
                Cancel
              </button>
            </div>
          </form>
        </div>
      )}

      {/* Components Table */}
      <div className="bg-white rounded-lg shadow-md overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead className="bg-gray-100 border-b">
              <tr>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-700">MPN</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-700">Designator</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-700">Type</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-700">Manufacturer</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-700">Qty</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-700">FIT</th>
                <th className="px-6 py-3 text-center text-sm font-semibold text-gray-700">Actions</th>
              </tr>
            </thead>
            <tbody className="divide-y">
              {loading ? (
                <tr>
                  <td colSpan={7} className="px-6 py-4 text-center text-gray-500">
                    Loading...
                  </td>
                </tr>
              ) : components.length === 0 ? (
                <tr>
                  <td colSpan={7} className="px-6 py-4 text-center text-gray-500">
                    No components found. Add one to get started!
                  </td>
                </tr>
              ) : (
                components.map((comp) => (
                  <tr key={comp.id} className="hover:bg-gray-50 transition">
                    <td className="px-6 py-4 text-sm font-medium text-gray-900">{comp.manufacturer_part_number}</td>
                    <td className="px-6 py-4 text-sm text-gray-600">{comp.reference_designator || '-'}</td>
                    <td className="px-6 py-4 text-sm">
                      <span className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-xs font-semibold">
                        {comp.component_type}
                      </span>
                    </td>
                    <td className="px-6 py-4 text-sm text-gray-600">{comp.manufacturer || '-'}</td>
                    <td className="px-6 py-4 text-sm text-gray-600">{comp.quantity}</td>
                    <td className="px-6 py-4 text-sm font-semibold text-orange-600">
                      {comp.fit ? comp.fit.toFixed(4) : '-'}
                    </td>
                    <td className="px-6 py-4 text-center space-x-2">
                      <button
                        className="text-blue-600 hover:text-blue-800 transition"
                        title="Calculate FIT"
                      >
                        <BarChart3 size={18} />
                      </button>
                      <button
                        className="text-orange-600 hover:text-orange-800 transition"
                        title="Edit"
                      >
                        <Edit2 size={18} />
                      </button>
                      <button
                        onClick={() => handleDelete(comp.id)}
                        className="text-red-600 hover:text-red-800 transition"
                        title="Delete"
                      >
                        <Trash2 size={18} />
                      </button>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="bg-white rounded-lg shadow p-4">
          <p className="text-gray-600 text-sm">Total Components</p>
          <p className="text-3xl font-bold text-gray-900">{components.length}</p>
        </div>
        <div className="bg-white rounded-lg shadow p-4">
          <p className="text-gray-600 text-sm">Resistors</p>
          <p className="text-3xl font-bold text-gray-900">
            {components.filter((c) => c.component_type === 'Resistor').length}
          </p>
        </div>
        <div className="bg-white rounded-lg shadow p-4">
          <p className="text-gray-600 text-sm">Capacitors</p>
          <p className="text-3xl font-bold text-gray-900">
            {components.filter((c) => c.component_type === 'Capacitor').length}
          </p>
        </div>
        <div className="bg-white rounded-lg shadow p-4">
          <p className="text-gray-600 text-sm">Other</p>
          <p className="text-3xl font-bold text-gray-900">
            {components.filter((c) => !['Resistor', 'Capacitor'].includes(c.component_type)).length}
          </p>
        </div>
      </div>
    </div>
  );
}
