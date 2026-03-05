import { useState, useEffect } from 'react';
import { Folder, Plus, BarChart3, Settings } from 'lucide-react';

interface Project {
  id: string;
  name: string;
  description?: string;
  component_count: number;
  created_at: string;
}

export default function ProjectOverview() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(false);
  const [showForm, setShowForm] = useState(false);
  const [formData, setFormData] = useState({
    name: '',
    description: '',
  });

  useEffect(() => {
    fetchProjects();
  }, []);

  const fetchProjects = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/projects');
      const data = await response.json();
      setProjects(data || []);
    } catch (error) {
      console.error('Failed to fetch projects:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateProject = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch('/api/projects', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData),
      });

      if (response.ok) {
        fetchProjects();
        setShowForm(false);
        setFormData({ name: '', description: '' });
      }
    } catch (error) {
      console.error('Failed to create project:', error);
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Projects</h1>
          <p className="text-gray-600 mt-1">Manage your FMEDA projects and analyze component reliability</p>
        </div>
        <button
          onClick={() => setShowForm(!showForm)}
          className="flex items-center gap-2 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition"
        >
          <Plus size={20} /> New Project
        </button>
      </div>

      {/* Create Project Form */}
      {showForm && (
        <div className="bg-white rounded-lg shadow-md p-6">
          <h2 className="text-xl font-semibold mb-4">Create New Project</h2>
          <form onSubmit={handleCreateProject} className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <input
                type="text"
                placeholder="Project Name"
                required
                value={formData.name}
                onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                className="border border-gray-300 rounded px-3 py-2"
              />
              <input
                type="text"
                placeholder="Description"
                value={formData.description}
                onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                className="border border-gray-300 rounded px-3 py-2"
              />
            </div>
            <div className="flex gap-2">
              <button
                type="submit"
                className="flex-1 bg-green-600 text-white rounded px-4 py-2 hover:bg-green-700 transition font-medium"
              >
                Create Project
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

      {/* Projects Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {loading ? (
          <p className="col-span-3 text-gray-500 text-center">Loading...</p>
        ) : projects.length === 0 ? (
          <div className="col-span-3 bg-white rounded-lg shadow p-8 text-center">
            <Folder size={48} className="mx-auto text-gray-300 mb-4" />
            <p className="text-gray-500 mb-4">No projects yet. Create one to get started!</p>
          </div>
        ) : (
          projects.map((project) => (
            <div
              key={project.id}
              className="bg-white rounded-lg shadow-md hover:shadow-lg transition p-6 cursor-pointer group"
            >
              <div className="flex items-start justify-between mb-4">
                <Folder className="text-blue-600 group-hover:text-blue-700" size={32} />
                <BarChart3 className="text-gray-400 group-hover:text-gray-600" size={24} />
              </div>

              <h3 className="text-xl font-bold text-gray-900 mb-2">{project.name}</h3>
              <p className="text-gray-600 text-sm mb-4">{project.description || 'No description'}</p>

              <div className="border-t border-gray-200 pt-4">
                <div className="flex justify-between items-center">
                  <div>
                    <p className="text-xs text-gray-600">Components</p>
                    <p className="text-2xl font-bold text-gray-900">{project.component_count || 0}</p>
                  </div>
                  <button className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition text-sm font-medium">
                    View
                  </button>
                </div>
              </div>

              <p className="text-xs text-gray-500 mt-4">
                Created: {new Date(project.created_at).toLocaleDateString()}
              </p>
            </div>
          ))
        )}
      </div>

      {/* Summary Stats */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="bg-blue-50 rounded-lg p-4 border border-blue-200">
          <p className="text-blue-700 font-semibold">Total Projects</p>
          <p className="text-3xl font-bold text-blue-900 mt-2">{projects.length}</p>
        </div>
        <div className="bg-green-50 rounded-lg p-4 border border-green-200">
          <p className="text-green-700 font-semibold">Total Components</p>
          <p className="text-3xl font-bold text-green-900 mt-2">
            {projects.reduce((sum, p) => sum + (p.component_count || 0), 0)}
          </p>
        </div>
        <div className="bg-orange-50 rounded-lg p-4 border border-orange-200">
          <p className="text-orange-700 font-semibold">Avg Components/Project</p>
          <p className="text-3xl font-bold text-orange-900 mt-2">
            {projects.length > 0
              ? (projects.reduce((sum, p) => sum + (p.component_count || 0), 0) / projects.length).toFixed(1)
              : '0'}
          </p>
        </div>
      </div>
    </div>
  );
}
