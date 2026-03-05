import { useNavigate } from 'react-router-dom';
import { BarChart3, Folder, Zap, Settings, ArrowRight } from 'lucide-react';
import Dashboard from '../components/Dashboard';

export default function Home() {
  const navigate = useNavigate();

  const features = [
    {
      icon: <Folder className="w-8 h-8" />,
      title: 'Project Management',
      description: 'Create and manage your FMEDA projects',
      path: '/projects',
      color: 'bg-blue-50 border-blue-200',
      iconColor: 'text-blue-600',
    },
    {
      icon: <BarChart3 className="w-8 h-8" />,
      title: 'Component Analysis',
      description: 'Add and analyze component FIT values',
      path: '/components',
      color: 'bg-green-50 border-green-200',
      iconColor: 'text-green-600',
    },
    {
      icon: <Zap className="w-8 h-8" />,
      title: 'FIT Dashboard',
      description: 'Real-time reliability metrics and trends',
      path: '/fit-dashboard',
      color: 'bg-orange-50 border-orange-200',
      iconColor: 'text-orange-600',
    },
    {
      icon: <Settings className="w-8 h-8" />,
      title: 'Mission Profiles',
      description: 'Define temperature and time profiles',
      path: '/mission-profiles',
      color: 'bg-purple-50 border-purple-200',
      iconColor: 'text-purple-600',
    },
  ];

  return (
    <div className="space-y-8">
      {/* Hero Section */}
      <div className="bg-gradient-to-r from-blue-600 to-blue-800 rounded-lg shadow-lg p-8 text-white">
        <h1 className="text-4xl font-bold mb-2">SafeCrate FMEDA Tool</h1>
        <p className="text-lg text-blue-100 mb-4">
          Advanced reliability analysis using SN29500 standards for electronic components
        </p>
        <p className="text-blue-100">
          Analyze component reliability with mission profile temperature/tau distributions
        </p>
      </div>

      {/* Quick Access Features */}
      <div>
        <h2 className="text-2xl font-bold text-gray-900 mb-4">Quick Access</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {features.map((feature) => (
            <button
              key={feature.path}
              onClick={() => navigate(feature.path)}
              className={`${feature.color} border-2 rounded-lg p-6 text-left hover:shadow-lg transition-all hover:scale-105 cursor-pointer`}
            >
              <div className={`${feature.iconColor} mb-3`}>{feature.icon}</div>
              <h3 className="font-bold text-gray-900 mb-2">{feature.title}</h3>
              <p className="text-sm text-gray-600 mb-3">{feature.description}</p>
              <div className="flex items-center text-blue-600 font-medium text-sm">
                Open <ArrowRight className="w-4 h-4 ml-2" />
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Dashboard Charts */}
      <div>
        <h2 className="text-2xl font-bold text-gray-900 mb-4">Dashboard Overview</h2>
        <Dashboard />
      </div>

      {/* Info Cards */}
      <div>
        <h2 className="text-2xl font-bold text-gray-900 mb-4">Key Features</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="bg-white rounded-lg shadow p-6 border-l-4 border-blue-600">
            <h3 className="font-bold text-lg text-gray-900 mb-2">SN29500 Calculations</h3>
            <p className="text-gray-600 text-sm">
              Accurate FIT (Failures In Time) calculations based on SN29500 standard for comprehensive reliability analysis of electronic components.
            </p>
          </div>

          <div className="bg-white rounded-lg shadow p-6 border-l-4 border-green-600">
            <h3 className="font-bold text-lg text-gray-900 mb-2">Mission Profile Support</h3>
            <p className="text-gray-600 text-sm">
              Define complex temperature and time profiles to accurately model real-world operating conditions for your components.
            </p>
          </div>

          <div className="bg-white rounded-lg shadow p-6 border-l-4 border-orange-600">
            <h3 className="font-bold text-lg text-gray-900 mb-2">Component Library</h3>
            <p className="text-gray-600 text-sm">
              Comprehensive database of component variants with thermal activation energies, quality factors, and reliability parameters.
            </p>
          </div>

          <div className="bg-white rounded-lg shadow p-6 border-l-4 border-purple-600">
            <h3 className="font-bold text-lg text-gray-900 mb-2">Real-time Analytics</h3>
            <p className="text-gray-600 text-sm">
              Interactive dashboards with charts, metrics, and detailed reports for analyzing reliability trends across your projects.
            </p>
          </div>
        </div>
      </div>

      {/* Getting Started */}
      <div className="bg-gray-50 rounded-lg p-8 border border-gray-200">
        <h2 className="text-2xl font-bold text-gray-900 mb-4">Getting Started</h2>
        <ol className="space-y-3 text-gray-700 list-decimal list-inside">
          <li>
            <strong>Create a Project:</strong> Go to <button onClick={() => navigate('/projects')} className="text-blue-600 hover:underline">Projects</button> and create a new FMEDA project.
          </li>
          <li>
            <strong>Define Mission Profile:</strong> Set up temperature and time segments in <button onClick={() => navigate('/mission-profiles')} className="text-blue-600 hover:underline">Mission Profiles</button>.
          </li>
          <li>
            <strong>Add Components:</strong> Manage components in <button onClick={() => navigate('/components')} className="text-blue-600 hover:underline">Components</button> section.
          </li>
          <li>
            <strong>Analyze FIT:</strong> View reliability metrics on the <button onClick={() => navigate('/fit-dashboard')} className="text-blue-600 hover:underline">FIT Dashboard</button>.
          </li>
        </ol>
      </div>
    </div>
  );
}
