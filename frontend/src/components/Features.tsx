'use client';

import React from 'react';
import { Brain, Network, Coins, Eye } from 'lucide-react';

export const Features: React.FC = () => {
  const features = [
    {
      icon: Brain,
      title: 'Echo Index™ Engine',
      description: 'Multi-dimensional attention scoring analyzing originality depth, audience quality, and transmission paths.',
      color: 'text-echo-purple'
    },
    {
      icon: Network,
      title: 'Echo Loop™ Mechanism',
      description: 'Smart propagation resonance system with real-time visualization and tiered reward distribution.',
      color: 'text-echo-blue'
    },
    {
      icon: Coins,
      title: 'Echo Drop Rewards',
      description: 'Behavioral point system rewarding content creation, quality interaction, and propagation contributions.',
      color: 'text-echo-green'
    },
    {
      icon: Eye,
      title: 'MPC Wallet Support',
      description: 'Zero-threshold crypto entry with social-bound light wallets and distributed key management.',
      color: 'text-primary-500'
    }
  ];

  return (
    <section id="features" className="py-20 px-6">
      <div className="container mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-4xl md:text-5xl font-bold text-white mb-6">
            Core Features
          </h2>
          <p className="text-xl text-gray-300 max-w-3xl mx-auto">
            Revolutionary technology stack for decentralized attention tracking and value distribution
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
          {features.map((feature, index) => (
            <div key={index} className="glass-effect rounded-lg p-6 hover:bg-opacity-20 transition-all duration-300">
              <div className="mb-4">
                <feature.icon className={`${feature.color} mb-4`} size={48} />
                <h3 className="text-xl font-bold text-white mb-3">{feature.title}</h3>
              </div>
              <p className="text-gray-400 leading-relaxed">{feature.description}</p>
            </div>
          ))}
        </div>

        {/* Additional Info Section */}
        <div className="mt-20">
          <div className="glass-effect rounded-lg p-8 md:p-12">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 items-center">
              <div>
                <h3 className="text-3xl font-bold text-white mb-6">
                  Attention Propagation Visualization
                </h3>
                <p className="text-gray-300 mb-6 leading-relaxed">
                  Track how content flows through networks with our real-time propagation graphs. 
                  Identify key transmission nodes, measure influence paths, and visualize the 
                  complete journey of attention across platforms.
                </p>
                <button className="bg-echo-blue text-white px-6 py-3 rounded-lg hover:bg-opacity-80 transition-colors">
                  Explore Visualization
                </button>
              </div>
              <div className="lg:text-right">
                <div className="inline-block p-8 glass-effect rounded-lg">
                  <div className="w-32 h-32 echo-gradient rounded-full mx-auto mb-4 animate-pulse-slow"></div>
                  <p className="text-echo-green font-mono text-sm">Propagation Network Active</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}; 