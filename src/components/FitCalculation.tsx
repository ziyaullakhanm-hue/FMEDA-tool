import React, { useState } from 'react';

const steps = ['Select Profile', 'Set Temperature/Tau', 'Review Constants', 'Run Calculation'];

const FitCalculation: React.FC = () => {
  const [activeStep, setActiveStep] = useState(0);

  const handleNext = () => {
    if (activeStep < steps.length - 1) {
      setActiveStep(activeStep + 1);
    }
  };

  const handleBack = () => {
    if (activeStep > 0) {
      setActiveStep(activeStep - 1);
    }
  };

  return (
    <div className="bg-white p-6 rounded-lg shadow">
      <h1 className="text-3xl font-bold mb-6">FIT Calculation</h1>

      {/* Stepper */}
      <div className="mb-8">
        <div className="flex items-center">
          {steps.map((label, idx) => (
            <div key={idx} className="flex items-center flex-1">
              <div
                className={`w-10 h-10 rounded-full flex items-center justify-center font-bold ${
                  idx <= activeStep
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-300 text-gray-600'
                }`}
              >
                {idx + 1}
              </div>
              <div className="text-sm font-medium mx-2">{label}</div>
              {idx < steps.length - 1 && (
                <div
                  className={`flex-1 h-1 mx-2 ${
                    idx < activeStep ? 'bg-blue-600' : 'bg-gray-300'
                  }`}
                ></div>
              )}
            </div>
          ))}
        </div>
      </div>

      {/* Step Content */}
      <div className="border border-gray-300 rounded p-6 mb-6 min-h-40 bg-gray-50">
        <p className="text-lg font-semibold mb-4">Step {activeStep + 1}: {steps[activeStep]}</p>
        <p className="text-gray-600">Placeholder for step content</p>
      </div>

      {/* Navigation Buttons */}
      <div className="flex items-center justify-between">
        <button
          onClick={handleBack}
          disabled={activeStep === 0}
          className="px-6 py-2 rounded bg-gray-300 text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-400"
        >
          Back
        </button>
        <button
          onClick={handleNext}
          className="px-6 py-2 rounded bg-blue-600 text-white hover:bg-blue-700"
        >
          {activeStep === steps.length - 1 ? 'Run Calculation' : 'Next'}
        </button>
      </div>
    </div>
  );
};

export default FitCalculation;
