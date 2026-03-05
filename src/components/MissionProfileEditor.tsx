import { useState, useEffect } from 'react';
import { Plus, Edit2, Trash2, Save, X } from 'lucide-react';

interface TemperatureSegment {
  temperature: number;
  tau: number;
}

interface MissionProfile {
  id: string;
  name: string;
  description?: string;
  segments: TemperatureSegment[];
}

export default function MissionProfileEditor() {
  const [profiles, setProfiles] = useState<MissionProfile[]>([]);
  const [selectedProfile, setSelectedProfile] = useState<MissionProfile | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [loading, setLoading] = useState(false);
  const [formData, setFormData] = useState({
    name: '',
    description: '',
    segments: [{ temperature: 25, tau: 0.5 }],
  });

  useEffect(() => {
    fetchProfiles();
  }, []);

  const fetchProfiles = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/mission-profiles');
      const data = await response.json();
      setProfiles(data || []);
    } catch (error) {
      console.error('Failed to fetch mission profiles:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleAddSegment = () => {
    setFormData({
      ...formData,
      segments: [...formData.segments, { temperature: 25, tau: 0.5 }],
    });
  };

  const handleRemoveSegment = (index: number) => {
    const newSegments = formData.segments.filter((_, i) => i !== index);
    setFormData({ ...formData, segments: newSegments });
  };

  const handleSegmentChange = (index: number, field: 'temperature' | 'tau', value: number) => {
    const newSegments = [...formData.segments];
    newSegments[index] = { ...newSegments[index], [field]: value };
    setFormData({ ...formData, segments: newSegments });
  };

  const handleSaveProfile = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch('/api/mission-profiles', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: formData.name,
          description: formData.description,
          temp_tau_profile: {
            temperature: formData.segments.map((s) => s.temperature),
            tau: formData.segments.map((s) => s.tau),
          },
        }),
      });

      if (response.ok) {
        fetchProfiles();
        setShowForm(false);
        setFormData({
          name: '',
          description: '',
          segments: [{ temperature: 25, tau: 0.5 }],
        });
      }
    } catch (error) {
      console.error('Failed to save profile:', error);
    }
  };

  const handleDeleteProfile = async (id: string) => {
    if (!window.confirm('Delete this mission profile?')) return;
    try {
      await fetch(`/api/mission-profiles/${id}`, { method: 'DELETE' });
      fetchProfiles();
      setSelectedProfile(null);
    } catch (error) {
      console.error('Failed to delete profile:', error);
    }
  };

  const totalTau = formData.segments.reduce((sum, s) => sum + s.tau, 0);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Mission Profile Editor</h1>
          <p className="text-gray-600 mt-1">Define temperature and time profiles for reliability calculations</p>
        </div>
        <button
          onClick={() => setShowForm(!showForm)}
          className="flex items-center gap-2 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition"
        >
          <Plus size={20} /> Create Profile
        </button>
      </div>

      {/* Create/Edit Form */}
      {showForm && (
        <div className="bg-white rounded-lg shadow-md p-6">
          <h2 className="text-xl font-semibold mb-4">Create New Mission Profile</h2>
          <form onSubmit={handleSaveProfile} className="space-y-4">
            {/* Basic Info */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-1">Profile Name *</label>
                <input
                  type="text"
                  required
                  placeholder="e.g., High Temperature Mission"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  className="w-full border border-gray-300 rounded px-3 py-2"
                />
              </div>
              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-1">Description</label>
                <input
                  type="text"
                  placeholder="Optional description"
                  value={formData.description}
                  onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                  className="w-full border border-gray-300 rounded px-3 py-2"
                />
              </div>
            </div>

            {/* Temperature/Tau Segments */}
            <div>
              <div className="flex justify-between items-center mb-3">
                <label className="block text-sm font-semibold text-gray-700">Temperature Segments *</label>
                <button
                  type="button"
                  onClick={handleAddSegment}
                  className="text-blue-600 hover:text-blue-800 font-medium text-sm flex items-center gap-1"
                >
                  <Plus size={16} /> Add Segment
                </button>
              </div>

              <div className="space-y-3 bg-gray-50 p-4 rounded-lg border border-gray-200">
                {formData.segments.map((segment, index) => (
                  <div key={index} className="flex gap-3 items-end">
                    <div className="flex-1">
                      <label className="text-xs text-gray-600 font-semibold">Temperature (°C)</label>
                      <input
                        type="number"
                        step="0.1"
                        value={segment.temperature}
                        onChange={(e) => handleSegmentChange(index, 'temperature', parseFloat(e.target.value))}
                        className="w-full border border-gray-300 rounded px-3 py-2 mt-1"
                      />
                    </div>
                    <div className="flex-1">
                      <label className="text-xs text-gray-600 font-semibold">Time Fraction (τ)</label>
                      <input
                        type="number"
                        step="0.01"
                        min="0"
                        max="1"
                        value={segment.tau}
                        onChange={(e) => handleSegmentChange(index, 'tau', parseFloat(e.target.value))}
                        className="w-full border border-gray-300 rounded px-3 py-2 mt-1"
                      />
                    </div>
                    <button
                      type="button"
                      onClick={() => handleRemoveSegment(index)}
                      className="text-red-600 hover:text-red-800 transition p-2"
                      title="Remove segment"
                    >
                      <Trash2 size={18} />
                    </button>
                  </div>
                ))}

                {/* Total Tau Display */}
                <div className="bg-white p-3 rounded border border-gray-300 mt-3">
                  <p className="text-sm text-gray-700">
                    <span className="font-semibold">Total Time Fraction (Σ τ):</span>{' '}
                    <span className={totalTau === 1 ? 'text-green-600' : 'text-red-600'} className="font-bold">
                      {totalTau.toFixed(4)}
                    </span>
                    {totalTau !== 1 && <span className="text-red-600 text-xs ml-2">(Should equal 1.0)</span>}
                  </p>
                </div>
              </div>
            </div>

            {/* Action Buttons */}
            <div className="flex gap-3 justify-end mt-6">
              <button
                type="button"
                onClick={() => {
                  setShowForm(false);
                  setFormData({
                    name: '',
                    description: '',
                    segments: [{ temperature: 25, tau: 0.5 }],
                  });
                }}
                className="flex items-center gap-2 bg-gray-400 text-white px-4 py-2 rounded-lg hover:bg-gray-500 transition"
              >
                <X size={18} /> Cancel
              </button>
              <button
                type="submit"
                className="flex items-center gap-2 bg-green-600 text-white px-4 py-2 rounded-lg hover:bg-green-700 transition disabled:opacity-50"
                disabled={!formData.name || totalTau !== 1}
              >
                <Save size={18} /> Create Profile
              </button>
            </div>
          </form>
        </div>
      )}

      {/* Profiles List */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Profiles Sidebar */}
        <div className="bg-white rounded-lg shadow-md overflow-hidden">
          <div className="px-6 py-4 border-b border-gray-200 bg-gray-50">
            <h2 className="text-lg font-semibold text-gray-900">Profiles</h2>
          </div>
          <div className="divide-y max-h-96 overflow-y-auto">
            {loading ? (
              <p className="px-6 py-4 text-gray-500 text-center">Loading...</p>
            ) : profiles.length === 0 ? (
              <p className="px-6 py-4 text-gray-500 text-center">No profiles. Create one!</p>
            ) : (
              profiles.map((profile) => (
                <div
                  key={profile.id}
                  onClick={() => setSelectedProfile(profile)}
                  className={`px-6 py-4 cursor-pointer transition ${
                    selectedProfile?.id === profile.id
                      ? 'bg-blue-50 border-l-4 border-blue-600'
                      : 'hover:bg-gray-50'
                  }`}
                >
                  <p className="font-semibold text-gray-900">{profile.name}</p>
                  <p className="text-xs text-gray-600 mt-1">
                    {profile.description || 'No description'}
                  </p>
                </div>
              ))
            )}
          </div>
        </div>

        {/* Profile Details */}
        {selectedProfile && (
          <div className="lg:col-span-2 bg-white rounded-lg shadow-md p-6">
            <div className="space-y-4">
              <div>
                <h2 className="text-2xl font-bold text-gray-900">{selectedProfile.name}</h2>
                <p className="text-gray-600 mt-1">{selectedProfile.description || 'No description'}</p>
              </div>

              {/* Segments Table */}
              <div>
                <h3 className="text-lg font-semibold text-gray-900 mb-3">Temperature Segments</h3>
                <div className="overflow-x-auto">
                  <table className="w-full">
                    <thead className="bg-gray-100 border-b">
                      <tr>
                        <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">Temperature (°C)</th>
                        <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">Time Fraction (τ)</th>
                        <th className="px-4 py-3 text-left text-sm font-semibold text-gray-700">Percentage</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y">
                      {selectedProfile.segments.map((segment, idx) => (
                        <tr key={idx} className="hover:bg-gray-50">
                          <td className="px-4 py-3 text-sm text-gray-900">{segment.temperature}°C</td>
                          <td className="px-4 py-3 text-sm text-gray-900">{segment.tau.toFixed(4)}</td>
                          <td className="px-4 py-3 text-sm text-gray-600">
                            {((segment.tau / selectedProfile.segments.reduce((s, seg) => s + seg.tau, 0)) * 100).toFixed(1)}%
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>

              {/* Delete Button */}
              <button
                onClick={() => handleDeleteProfile(selectedProfile.id)}
                className="flex items-center gap-2 bg-red-600 text-white px-4 py-2 rounded-lg hover:bg-red-700 transition mt-4"
              >
                <Trash2 size={18} /> Delete Profile
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
