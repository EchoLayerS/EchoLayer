import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

// Utility to merge Tailwind classes
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// Format large numbers
export function formatNumber(num: number): string {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M';
  }
  if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K';
  }
  return num.toString();
}

// Format echo score
export function formatEchoScore(score: number): string {
  return score.toFixed(2);
}

// Calculate time ago
export function timeAgo(date: Date): string {
  const now = new Date();
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  const intervals = [
    { label: 'year', seconds: 31536000 },
    { label: 'month', seconds: 2592000 },
    { label: 'week', seconds: 604800 },
    { label: 'day', seconds: 86400 },
    { label: 'hour', seconds: 3600 },
    { label: 'minute', seconds: 60 },
  ];

  for (const interval of intervals) {
    const count = Math.floor(diffInSeconds / interval.seconds);
    if (count >= 1) {
      return `${count} ${interval.label}${count > 1 ? 's' : ''} ago`;
    }
  }

  return 'Just now';
}

// Truncate text
export function truncateText(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength) + '...';
}

// Generate random color for visualization
export function generateColor(seed: string): string {
  let hash = 0;
  for (let i = 0; i < seed.length; i++) {
    const char = seed.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32-bit integer
  }
  
  const hue = Math.abs(hash) % 360;
  return `hsl(${hue}, 70%, 60%)`;
}

// Validate Solana wallet address
export function isValidSolanaAddress(address: string): boolean {
  try {
    // Basic validation for Solana address format
    // Solana addresses are base58 encoded and typically 32-44 characters long
    if (!address || address.length < 32 || address.length > 44) {
      return false;
    }
    
    // Check if it contains only valid base58 characters
    const base58Regex = /^[A-HJ-NP-Z1-9]+$/;
    return base58Regex.test(address);
  } catch {
    return false;
  }
}

// Debounce function
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout;
  return (...args: Parameters<T>) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func(...args), wait);
  };
}

// Format wallet address
export function formatWalletAddress(address: string, chars = 4): string {
  if (!address) return '';
  if (address.length <= chars * 2) return address;
  return `${address.slice(0, chars)}...${address.slice(-chars)}`;
}

// Calculate echo index score
export function calculateEchoIndex(
  odf: number,
  awr: number,
  tpm: number,
  qf: number
): number {
  // Weighted calculation of echo index
  const weights = { odf: 0.3, awr: 0.25, tpm: 0.25, qf: 0.2 };
  return odf * weights.odf + awr * weights.awr + tpm * weights.tpm + qf * weights.qf;
}

// Platform detection
export function detectPlatform(url: string): string {
  if (url.includes('twitter.com') || url.includes('x.com')) return 'twitter';
  if (url.includes('telegram.org') || url.includes('t.me')) return 'telegram';
  if (url.includes('linkedin.com')) return 'linkedin';
  if (url.includes('instagram.com')) return 'instagram';
  return 'unknown';
}

// Sleep utility
export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

// Local storage utilities
export const storage = {
  get: <T>(key: string, defaultValue?: T): T | undefined => {
    try {
      const item = localStorage.getItem(key);
      return item ? JSON.parse(item) : defaultValue;
    } catch {
      return defaultValue;
    }
  },
  
  set: <T>(key: string, value: T): void => {
    try {
      localStorage.setItem(key, JSON.stringify(value));
    } catch {
      // Handle storage errors silently
    }
  },
  
  remove: (key: string): void => {
    try {
      localStorage.removeItem(key);
    } catch {
      // Handle storage errors silently
    }
  },
}; 