'use client';

import React, { useState } from 'react';
import { Menu, X } from 'lucide-react';

export const Header: React.FC = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <header className="fixed top-0 w-full z-50 glass-effect">
      <div className="container mx-auto px-6 py-4">
        <div className="flex items-center justify-between">
          {/* Logo */}
          <div className="flex items-center space-x-2">
            <div className="w-8 h-8 echo-gradient rounded-lg"></div>
            <span className="text-xl font-bold text-white">EchoLayer</span>
          </div>

          {/* Desktop Navigation */}
          <nav className="hidden md:flex items-center space-x-8">
            <a href="#home" className="text-gray-300 hover:text-white transition-colors">
              Home
            </a>
            <a href="#features" className="text-gray-300 hover:text-white transition-colors">
              Features
            </a>
            <a href="#docs" className="text-gray-300 hover:text-white transition-colors">
              Docs
            </a>
            <a href="#community" className="text-gray-300 hover:text-white transition-colors">
              Community
            </a>
            <button className="bg-echo-purple text-white px-6 py-2 rounded-lg hover:bg-opacity-80 transition-colors">
              Launch App
            </button>
          </nav>

          {/* Mobile Menu Button */}
          <button
            className="md:hidden text-white"
            onClick={() => setIsMenuOpen(!isMenuOpen)}
          >
            {isMenuOpen ? <X size={24} /> : <Menu size={24} />}
          </button>
        </div>

        {/* Mobile Navigation */}
        {isMenuOpen && (
          <nav className="md:hidden mt-4 pb-4 border-t border-gray-700">
            <div className="flex flex-col space-y-4 pt-4">
              <a href="#home" className="text-gray-300 hover:text-white transition-colors">
                Home
              </a>
              <a href="#features" className="text-gray-300 hover:text-white transition-colors">
                Features
              </a>
              <a href="#docs" className="text-gray-300 hover:text-white transition-colors">
                Docs
              </a>
              <a href="#community" className="text-gray-300 hover:text-white transition-colors">
                Community
              </a>
              <button className="bg-echo-purple text-white px-6 py-2 rounded-lg hover:bg-opacity-80 transition-colors w-full">
                Launch App
              </button>
            </div>
          </nav>
        )}
      </div>
    </header>
  );
}; 