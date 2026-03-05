import React, { useState } from 'react';

const Settings: React.FC = () => {
  const [darkMode, setDarkMode] = useState(false);
  const [emailNotifications, setEmailNotifications] = useState(true);
  const [inAppNotifications, setInAppNotifications] = useState(true);
  const [exportFormat, setExportFormat] = useState('excel');

  return (
    <div className="bg-white p-6 rounded-lg shadow max-w-2xl">
      <h1 className="text-3xl font-bold mb-8">Settings</h1>

      {/* Theme Section */}
      <div className="mb-8 pb-8 border-b">
        <h2 className="text-xl font-semibold mb-4">Theme</h2>
        <label className="flex items-center gap-3 cursor-pointer">
          <input
            type="checkbox"
            checked={darkMode}
            onChange={(e) => setDarkMode(e.target.checked)}
            className="w-5 h-5"
          />
          <span>Dark Mode</span>
        </label>
      </div>

      {/* Notification Preferences Section */}
      <div className="mb-8 pb-8 border-b">
        <h2 className="text-xl font-semibold mb-4">Notification Preferences</h2>
        <div className="space-y-3">
          <label className="flex items-center gap-3 cursor-pointer">
            <input
              type="checkbox"
              checked={emailNotifications}
              onChange={(e) => setEmailNotifications(e.target.checked)}
              className="w-5 h-5"
            />
            <span>Email Notifications</span>
          </label>
          <label className="flex items-center gap-3 cursor-pointer">
            <input
              type="checkbox"
              checked={inAppNotifications}
              onChange={(e) => setInAppNotifications(e.target.checked)}
              className="w-5 h-5"
            />
            <span>In-App Notifications</span>
          </label>
        </div>
      </div>

      {/* Export Format Section */}
      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">Default Export Format</h2>
        <select
          value={exportFormat}
          onChange={(e) => setExportFormat(e.target.value)}
          className="border border-gray-300 rounded px-4 py-2 focus:outline-none focus:border-blue-500"
        >
          <option value="excel">Excel</option>
          <option value="pdf">PDF</option>
          <option value="csv">CSV</option>
        </select>
      </div>

      {/* Save Button */}
      <button className="bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700">
        Save Settings
      </button>
    </div>
  );
};

export default Settings;
