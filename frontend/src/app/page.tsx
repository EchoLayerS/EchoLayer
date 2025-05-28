import React from 'react';
import { Header } from '@/components/Header';
import { Hero } from '@/components/Hero';
import { Features } from '@/components/Features';
import { Footer } from '@/components/Footer';

export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-dark-300 via-dark-200 to-dark-100">
      <Header />
      <Hero />
      <Features />
      <Footer />
    </main>
  );
} 