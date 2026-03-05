import React from 'react';
import { Github, Heart } from 'lucide-react';

const Footer: React.FC = () => {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="bg-gray-900 text-gray-300 py-8 mt-12">
      <div className="max-w-7xl mx-auto px-4 grid grid-cols-1 md:grid-cols-3 gap-8">
        {/* Left Section */}
        <div>
          <h3 className="text-white font-semibold mb-2">SafeCrate</h3>
          <p className="text-sm text-gray-400">
            FMEDA and reliability analysis tool for electronic systems
          </p>
          <p className="text-xs text-gray-500 mt-4">Version 1.0.0</p>
        </div>

        {/* Middle Section */}
        <div className="flex justify-center">
          <div>
            <h4 className="text-white font-semibold mb-3">Quick Links</h4>
            <ul className="space-y-2 text-sm">
              <li><a href="#" className="hover:text-white transition">Documentation</a></li>
              <li><a href="#" className="hover:text-white transition">API Reference</a></li>
              <li><a href="#" className="hover:text-white transition">Support</a></li>
              <li><a href="#" className="hover:text-white transition">Privacy Policy</a></li>
            </ul>
          </div>
        </div>

        {/* Right Section */}
        <div className="flex justify-end">
          <div>
            <h4 className="text-white font-semibold mb-3">Contact & Support</h4>
            <ul className="space-y-2 text-sm">
              <li><a href="mailto:support@safecrate.com" className="hover:text-white transition">support@safecrate.com</a></li>
              <li><a href="#" className="hover:text-white transition flex items-center gap-2">
                <Github className="w-4 h-4" />
                GitHub
              </a></li>
              <li className="pt-2 text-xs text-gray-500">Made with <Heart className="w-3 h-3 inline text-red-500" /> by SafeCrate Team</li>
            </ul>
          </div>
        </div>
      </div>

      {/* Bottom Bar */}
      <div className="border-t border-gray-800 mt-8 pt-6 text-center text-sm text-gray-500">
        <p>&copy; {currentYear} SafeCrate. All rights reserved.</p>
      </div>
    </footer>
  );
};

export default Footer;
