'use client';

import React from 'react';
import { Github, Twitter, Globe } from 'lucide-react';

export const Footer: React.FC = () => {
  return (
    <footer className="py-12 px-6 border-t border-gray-800">
      <div className="container mx-auto">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
          {/* Logo and Description */}
          <div className="md:col-span-2">
            <div className="flex items-center space-x-2 mb-4">
              <div className="w-8 h-8 echo-gradient rounded-lg"></div>
              <span className="text-xl font-bold text-white">EchoLayer</span>
            </div>
            <p className="text-gray-400 max-w-md leading-relaxed">
              Reconstructing the decentralized attention ecosystem through signal-aware technology 
              that tracks attention propagation across content, platforms and networks.
            </p>
          </div>

          {/* Links */}
          <div>
            <h4 className="text-white font-semibold mb-4">Product</h4>
            <ul className="space-y-2">
              <li><a href="#features" className="text-gray-400 hover:text-white transition-colors">Features</a></li>
              <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Documentation</a></li>
              <li><a href="#" className="text-gray-400 hover:text-white transition-colors">API</a></li>
              <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Whitepaper</a></li>
            </ul>
          </div>

          {/* Community */}
          <div>
            <h4 className="text-white font-semibold mb-4">Community</h4>
            <ul className="space-y-2">
              <li><a href="https://x.com/EchoLayer_" className="text-gray-400 hover:text-white transition-colors">Twitter</a></li>
              <li><a href="https://github.com/EchoLayerS/EchoLayer" className="text-gray-400 hover:text-white transition-colors">GitHub</a></li>
              <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Blog</a></li>
              <li><a href="#" className="text-gray-400 hover:text-white transition-colors">FAQ</a></li>
            </ul>
          </div>
        </div>

        {/* Bottom Bar */}
        <div className="border-t border-gray-800 mt-8 pt-8 flex flex-col md:flex-row justify-between items-center">
          <p className="text-gray-400 text-sm">
            © 2024 EchoLayer. All rights reserved.
          </p>
          
          {/* Social Links */}
          <div className="flex items-center space-x-4 mt-4 md:mt-0">
            <a 
              href="https://www.echolayers.xyz" 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white transition-colors"
            >
              <Globe size={20} />
            </a>
            <a 
              href="https://x.com/EchoLayer_" 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white transition-colors"
            >
              <Twitter size={20} />
            </a>
            <a 
              href="https://github.com/EchoLayerS/EchoLayer" 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white transition-colors"
            >
              <Github size={20} />
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
}; 