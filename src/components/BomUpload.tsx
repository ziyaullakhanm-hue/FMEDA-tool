import React, { useState } from 'react';

const BomUpload: React.FC = () => {
  const [files, setFiles] = useState<File[]>([]);

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.currentTarget.classList.add('border-blue-500', 'bg-blue-50');
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.currentTarget.classList.remove('border-blue-500', 'bg-blue-50');
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.currentTarget.classList.remove('border-blue-500', 'bg-blue-50');
    setFiles(Array.from(e.dataTransfer.files));
  };

  return (
    <div className="bg-white p-6 rounded-lg shadow">
      <h2 className="text-2xl font-bold mb-6">BOM Upload</h2>
      
      <div
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        className="border-2 border-dashed border-gray-300 rounded-lg p-12 text-center cursor-pointer hover:bg-gray-50 transition mb-6"
      >
        <p className="text-gray-600 mb-2">Drag & drop file here</p>
        <p className="text-gray-500 mb-4">or</p>
        <button className="bg-blue-600 text-white px-6 py-2 rounded hover:bg-blue-700">
          Select File
        </button>
      </div>

      {files.length > 0 && (
        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3">Selected Files</h3>
          <ul className="space-y-2">
            {files.map((file, idx) => (
              <li key={idx} className="text-sm text-gray-600 bg-gray-50 p-2 rounded">
                {file.name} ({(file.size / 1024).toFixed(2)} KB)
              </li>
            ))}
          </ul>
        </div>
      )}

      <div className="mb-6">
        <h3 className="text-lg font-semibold mb-3">File Preview (First 10 rows)</h3>
        <div className="bg-gray-50 border border-gray-200 rounded p-4 text-gray-500">
          Table preview will appear here
        </div>
      </div>

      <div>
        <h3 className="text-lg font-semibold mb-3">Validation Messages</h3>
        <div className="bg-red-50 border border-red-200 rounded p-4 text-red-600">
          Validation messages will appear here
        </div>
      </div>
    </div>
  );
};

export default BomUpload;
