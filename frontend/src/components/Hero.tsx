'use client';

import React from 'react';
import { ArrowRight, Zap, TrendingUp } from 'lucide-react';

export const Hero: React.FC = () => {
  return (
    <section id="home" className="pt-32 pb-20 px-6">
      <div className="container mx-auto text-center">
        {/* Main Heading */}
        <div className="mb-8">
          <h1 className="text-5xl md:text-7xl font-bold text-white mb-6 leading-tight">
            <span className="block">TRACING</span>
            <span className="block echo-text-gradient">SIGNALS</span>
            <span className="block">REVEALING VALUE</span>
          </h1>
          <p className="text-xl md:text-2xl text-gray-300 max-w-4xl mx-auto leading-relaxed">
            A signal-aware layer that tracks attention propagation across content, platforms and networks
          </p>
        </div>

        {/* CTA Buttons */}
        <div className="mb-16">
          <div className="flex flex-col sm:flex-row gap-4 justify-center items-center">
            <button className="group bg-echo-purple text-white px-8 py-4 rounded-lg text-lg font-semibold hover:bg-opacity-80 transition-all duration-300 flex items-center space-x-2">
              <span>Explore App</span>
              <ArrowRight className="group-hover:translate-x-1 transition-transform" size={20} />
            </button>
            <button className="border border-echo-blue text-echo-blue px-8 py-4 rounded-lg text-lg font-semibold hover:bg-echo-blue hover:text-white transition-all duration-300">
              View Score Engine
            </button>
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-4xl mx-auto">
          <div className="glass-effect rounded-lg p-6">
            <div className="flex items-center justify-center mb-4">
              <Zap className="text-echo-green" size={32} />
            </div>
            <h3 className="text-2xl font-bold text-white mb-2">Real-time Tracking</h3>
            <p className="text-gray-400">Monitor attention propagation across platforms instantly</p>
          </div>
          <div className="glass-effect rounded-lg p-6">
            <div className="flex items-center justify-center mb-4">
              <TrendingUp className="text-echo-purple" size={32} />
            </div>
            <h3 className="text-2xl font-bold text-white mb-2">Echo Index™</h3>
            <p className="text-gray-400">Multi-dimensional attention scoring engine</p>
          </div>
          <div className="glass-effect rounded-lg p-6">
            <div className="flex items-center justify-center mb-4">
              <div className="w-8 h-8 echo-gradient rounded-full"></div>
            </div>
            <h3 className="text-2xl font-bold text-white mb-2">Decentralized</h3>
            <p className="text-gray-400">Built on Solana for transparent value distribution</p>
          </div>
        </div>
      </div>
    </section>
  );
}; 