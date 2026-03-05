import { useState, useEffect } from 'react';
import { BarChart, Bar, LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer, PieChart, Pie, Cell } from 'recharts';
import { TrendingUp, AlertTriangle, CheckCircle } from 'lucide-react';

interface FITData {
  component: string;
  fit: number;
  type: string;
  status: 'good' | 'warning' | 'critical';
}

interface MissionProfile {
  id: string;
  name: string;
  description?: string;
}

export default function FITCalculationDashboard() {
  const [fitResults, setFitResults] = useState<FITData[]>([]);
  const [missionProfiles, setMissionProfiles] = useState<MissionProfile[]>([]);
  const [selectedProfile, setSelectedProfile] = useState<string>('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    fetchMissionProfiles();
  }, []);

  useEffect(() => {
    if (selectedProfile) {
      fetchFITResults();
    }
  }, [selectedProfile]);

  const fetchMissionProfiles = async () => {
    try {
      const response = await fetch('/api/mission-profiles');
      const data = await response.json();
      setMissionProfiles(data || []);
      if (data && data.length > 0) {
        setSelectedProfile(data[0].id);
      }
    } catch (error) {
      console.error('Failed to fetch mission profiles:', error);
    }
  };

  const fetchFITResults = async () => {
    setLoading(true);
    try {
      const response = await fetch(`/api/fit-calculations?profile_id=${selectedProfile}`);
      const data = await response.json();
      setFitResults(data || []);
    } catch (error) {
      console.error('Failed to fetch FIT results:', error);
    } finally {
      setLoading(false);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'good':
        return '#10b981';
      case 'warning':
        return '#f59e0b';
      case 'critical':
        return '#ef4444';
      default:
        return '#6b7280';
    }
  };

  const totalFIT = fitResults.reduce((sum, item) => sum + item.fit, 0);
  const avgFIT = fitResults.length > 0 ? totalFIT / fitResults.length : 0;
  const criticalCount = fitResults.filter((item) => item.status === 'critical').length;
  const warningCount = fitResults.filter((item) => item.status === 'warning').length;

  const typeDistribution = fitResults.reduce((acc, item) => {
    const existing = acc.find((d) => d.name === item.type);
    if (existing) {
      existing.value++;
    } else {
      acc.push({ name: item.type, value: 1 });
    }
    return acc;
  }, [] as { name: string; value: number }[]);

  const COLORS = ['#3b82f6', '#ef4444', '#10b981', '#f59e0b', '#8b5cf6', '#ec4899'];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900">FIT Calculation Dashboard</h1>
        <p className="text-gray-600 mt-1">Real-time reliability analysis (Failures per 10⁹ hours)</p>
      </div>

      {/* Mission Profile Selector */}
      <div className="bg-white rounded-lg shadow-md p-4">
        <label className="block text-sm font-semibold text-gray-700 mb-2">Select Mission Profile</label>
        <select
          value={selectedProfile}
          onChange={(e) => setSelectedProfile(e.target.value)}
          className="w-full md:w-64 border border-gray-300 rounded px-3 py-2"
        >
          <option value="">-- Choose Profile --</option>
          {missionProfiles.map((profile) => (
            <option key={profile.id} value={profile.id}>
              {profile.name}
            </option>
          ))}
        </select>
      </div>

      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="bg-white rounded-lg shadow p-5">
          <div className="flex items-start justify-between">
            <div>
              <p className="text-gray-600 text-sm">Total System FIT</p>
              <p className="text-3xl font-bold text-gray-900 mt-2">{totalFIT.toFixed(2)}</p>
            </div>
            <TrendingUp className="text-blue-600" size={28} />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-5">
          <div className="flex items-start justify-between">
            <div>
              <p className="text-gray-600 text-sm">Average Component FIT</p>
              <p className="text-3xl font-bold text-gray-900 mt-2">{avgFIT.toFixed(4)}</p>
            </div>
            <CheckCircle className="text-green-600" size={28} />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-5">
          <div className="flex items-start justify-between">
            <div>
              <p className="text-gray-600 text-sm">Critical Items</p>
              <p className="text-3xl font-bold text-red-600 mt-2">{criticalCount}</p>
            </div>
            <AlertTriangle className="text-red-600" size={28} />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-5">
          <div className="flex items-start justify-between">
            <div>
              <p className="text-gray-600 text-sm">Warnings</p>
              <p className="text-3xl font-bold text-yellow-600 mt-2">{warningCount}</p>
            </div>
            <AlertTriangle className="text-yellow-600" size={28} />
          </div>
        </div>
      </div>

      {/* Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* FIT by Component */}
        <div className="bg-white rounded-lg shadow-md p-6">
          <h2 className="text-xl font-semibold text-gray-900 mb-4">FIT by Component</h2>
          {loading ? (
            <p className="text-gray-500">Loading...</p>
          ) : fitResults.length === 0 ? (
            <p className="text-gray-500">No data available</p>
          ) : (
            <ResponsiveContainer width="100%" height={300}>
              <BarChart data={fitResults}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="component" angle={-45} textAnchor="end" height={80} />
                <YAxis />
                <Tooltip />
                <Bar dataKey="fit" fill="#3b82f6" />
              </BarChart>
            </ResponsiveContainer>
          )}
        </div>

        {/* Component Type Distribution */}
        <div className="bg-white rounded-lg shadow-md p-6">
          <h2 className="text-xl font-semibold text-gray-900 mb-4">Component Type Distribution</h2>
          {typeDistribution.length === 0 ? (
            <p className="text-gray-500">No data available</p>
          ) : (
            <ResponsiveContainer width="100%" height={300}>
              <PieChart>
                <Pie
                  data={typeDistribution}
                  cx="50%"
                  cy="50%"
                  labelLine={false}
                  label={({ name, value }) => `${name}: ${value}`}
                  outerRadius={80}
                  fill="#8884d8"
                  dataKey="value"
                >
                  {typeDistribution.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                  ))}
                </Pie>
                <Tooltip />
              </PieChart>
            </ResponsiveContainer>
          )}
        </div>
      </div>

      {/* Detailed Results Table */}
      <div className="bg-white rounded-lg shadow-md overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h2 className="text-xl font-semibold text-gray-900">Detailed FIT Results</h2>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead className="bg-gray-100 border-b">
              <tr>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-700">Component</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-700">Type</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-700">FIT Value</th>
                <th className="px-6 py-3 text-left text-sm font-semibold text-gray-700">Status</th>
              </tr>
            </thead>
            <tbody className="divide-y">
              {loading ? (
                <tr>
                  <td colSpan={4} className="px-6 py-4 text-center text-gray-500">
                    Loading...
                  </td>
                </tr>
              ) : fitResults.length === 0 ? (
                <tr>
                  <td colSpan={4} className="px-6 py-4 text-center text-gray-500">
                    No results available
                  </td>
                </tr>
              ) : (
                fitResults.map((result, idx) => (
                  <tr key={idx} className="hover:bg-gray-50 transition">
                    <td className="px-6 py-4 text-sm font-medium text-gray-900">{result.component}</td>
                    <td className="px-6 py-4 text-sm">
                      <span className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-xs font-semibold">
                        {result.type}
                      </span>
                    </td>
                    <td className="px-6 py-4 text-sm font-semibold text-orange-600">{result.fit.toFixed(6)}</td>
                    <td className="px-6 py-4 text-sm">
                      <span
                        className={`px-3 py-1 rounded-full text-xs font-semibold text-white ${
                          result.status === 'good'
                            ? 'bg-green-600'
                            : result.status === 'warning'
                              ? 'bg-yellow-600'
                              : 'bg-red-600'
                        }`}
                      >
                        {result.status.toUpperCase()}
                      </span>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
