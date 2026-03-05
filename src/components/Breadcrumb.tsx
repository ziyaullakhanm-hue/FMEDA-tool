import React from 'react';
import { ChevronRight } from 'lucide-react';
import { useLocation } from 'react-router-dom';

interface BreadcrumbItem {
  label: string;
  path?: string;
}

const Breadcrumb: React.FC = () => {
  const location = useLocation();

  // Define breadcrumb mappings
  const breadcrumbMap: { [key: string]: BreadcrumbItem[] } = {
    '/': [{ label: 'Home' }],
    '/projects': [{ label: 'Home', path: '/' }, { label: 'Projects' }],
    '/components': [{ label: 'Home', path: '/' }, { label: 'Components' }],
    '/fit-dashboard': [{ label: 'Home', path: '/' }, { label: 'FIT Dashboard' }],
    '/mission-profiles': [{ label: 'Home', path: '/' }, { label: 'Mission Profiles' }],
    '/fta': [{ label: 'Home', path: '/' }, { label: 'FTA' }],
    '/new-project': [{ label: 'Home', path: '/' }, { label: 'New Project' }],
    '/langgraph-dashboard': [{ label: 'Home', path: '/' }, { label: 'Tools', path: '#' }, { label: 'Dashboard' }],
    '/bom-upload': [{ label: 'Home', path: '/' }, { label: 'Tools', path: '#' }, { label: 'BOM Upload' }],
    '/fit-calculation': [{ label: 'Home', path: '/' }, { label: 'Tools', path: '#' }, { label: 'FIT Calculation' }],
    '/compliance-checker': [{ label: 'Home', path: '/' }, { label: 'Tools', path: '#' }, { label: 'Compliance Checker' }],
    '/components-table': [{ label: 'Home', path: '/' }, { label: 'Tools', path: '#' }, { label: 'Components Table' }],
    '/reports': [{ label: 'Home', path: '/' }, { label: 'Reports' }],
    '/settings': [{ label: 'Home', path: '/' }, { label: 'Settings' }],
  };

  const breadcrumbs = breadcrumbMap[location.pathname] || [{ label: 'Page' }];

  return (
    <nav className="flex items-center gap-2 text-sm text-gray-600 mb-6">
      {breadcrumbs.map((item, index) => (
        <div key={index} className="flex items-center gap-2">
          {item.path ? (
            <a href={item.path} className="text-blue-600 hover:text-blue-700 transition">
              {item.label}
            </a>
          ) : (
            <span className={index === breadcrumbs.length - 1 ? 'text-gray-900 font-semibold' : ''}>
              {item.label}
            </span>
          )}
          {index < breadcrumbs.length - 1 && <ChevronRight className="w-4 h-4" />}
        </div>
      ))}
    </nav>
  );
};

export default Breadcrumb;
