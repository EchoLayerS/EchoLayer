// Shared constants for the EchoLayer ecosystem

import { EchoIndexWeights, SocialPlatform } from '../types';

// Echo Index Configuration
export const ECHO_INDEX_WEIGHTS: EchoIndexWeights = {
  odf: 0.30,  // Originality Depth Factor: 30%
  awr: 0.25,  // Audience Weight Rating: 25%
  tpm: 0.25,  // Transmission Path Mapping: 25%
  qf: 0.20,   // Quote Frequency: 20%
};

export const ECHO_INDEX_THRESHOLDS = {
  EXCELLENT: 0.8,
  GOOD: 0.6,
  AVERAGE: 0.4,
  POOR: 0.2,
  MINIMAL: 0.0,
} as const;

// Platform Configuration
export const PLATFORM_CONFIG = {
  [SocialPlatform.TWITTER]: {
    name: 'Twitter/X',
    icon: '𝕏',
    color: '#1DA1F2',
    max_content_length: 280,
    api_rate_limit: 100,
    weight_multiplier: 1.0,
  },
  [SocialPlatform.TELEGRAM]: {
    name: 'Telegram',
    icon: '📢',
    color: '#0088CC',
    max_content_length: 4096,
    api_rate_limit: 200,
    weight_multiplier: 0.9,
  },
  [SocialPlatform.LINKEDIN]: {
    name: 'LinkedIn',
    icon: '💼',
    color: '#0077B5',
    max_content_length: 3000,
    api_rate_limit: 75,
    weight_multiplier: 1.1,
  },
  [SocialPlatform.INSTAGRAM]: {
    name: 'Instagram',
    icon: '📸',
    color: '#E4405F',
    max_content_length: 2200,
    api_rate_limit: 50,
    weight_multiplier: 0.7,
  },
  [SocialPlatform.YOUTUBE]: {
    name: 'YouTube',
    icon: '📺',
    color: '#FF0000',
    max_content_length: 5000,
    api_rate_limit: 25,
    weight_multiplier: 1.2,
  },
  [SocialPlatform.TIKTOK]: {
    name: 'TikTok',
    icon: '🎵',
    color: '#000000',
    max_content_length: 2200,
    api_rate_limit: 30,
    weight_multiplier: 0.6,
  },
  [SocialPlatform.REDDIT]: {
    name: 'Reddit',
    icon: '🤖',
    color: '#FF4500',
    max_content_length: 40000,
    api_rate_limit: 60,
    weight_multiplier: 0.9,
  },
} as const;

// API Configuration
export const API_CONFIG = {
  BASE_URL: process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1',
  WS_URL: process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:8080/ws',
  TIMEOUT: 30000, // 30 seconds
  RETRY_ATTEMPTS: 3,
  RETRY_DELAY: 1000, // 1 second
} as const;

export const API_ENDPOINTS = {
  // Health
  HEALTH: '/health',
  
  // Users
  USERS: '/users',
  USER_BY_ID: (id: string) => `/users/${id}`,
  USER_PROFILE: (id: string) => `/users/${id}/profile`,
  USER_REWARDS: (id: string) => `/users/${id}/rewards`,
  
  // Content
  CONTENT: '/content',
  CONTENT_BY_ID: (id: string) => `/content/${id}`,
  CALCULATE_ECHO_INDEX: (id: string) => `/content/${id}/calculate-echo-index`,
  CONTENT_PROPAGATIONS: (id: string) => `/content/${id}/propagations`,
  
  // Analytics
  ANALYTICS_ECHO_INDEX: '/analytics/echo-index',
  ANALYTICS_PROPAGATION: '/analytics/propagation',
  ANALYTICS_USERS: '/analytics/users',
  
  // Rewards
  REWARDS_DISTRIBUTE: '/rewards/distribute',
  REWARDS_LEADERBOARD: '/rewards/leaderboard',
} as const;

// Pagination Defaults
export const PAGINATION_DEFAULTS = {
  PAGE: 1,
  LIMIT: 20,
  MAX_LIMIT: 100,
} as const;

// Rate Limiting
export const RATE_LIMITS = {
  ANONYMOUS_HOURLY: 1000,
  AUTHENTICATED_HOURLY: 5000,
  CONTENT_CREATION_DAILY: 50,
  ECHO_INDEX_CALCULATION_HOURLY: 100,
} as const;

// Reward Configuration
export const REWARD_CONFIG = {
  BASE_CONTENT_REWARD: 10,
  HIGH_ECHO_INDEX_MULTIPLIER: 2.0,
  VIRAL_PROPAGATION_MULTIPLIER: 1.5,
  EARLY_ADOPTION_BONUS: 5,
  REFERRAL_BONUS: 25,
  QUALITY_INTERACTION_REWARD: 3,
  COMMUNITY_CONTRIBUTION_REWARD: 15,
} as const;

export const REWARD_THRESHOLDS = {
  HIGH_ECHO_INDEX: 0.7,
  VIRAL_PROPAGATION_COUNT: 50,
  QUALITY_INTERACTION_SCORE: 0.8,
} as const;

// Content Analysis
export const CONTENT_ANALYSIS = {
  MIN_WORD_COUNT: 5,
  MAX_WORD_COUNT: 1000,
  ORIGINALITY_KEYWORDS: [
    'innovative', 'breakthrough', 'revolutionary', 'novel', 'unique',
    'pioneering', 'cutting-edge', 'disruptive', 'transformative', 'groundbreaking'
  ],
  QUALITY_INDICATORS: [
    'analysis', 'research', 'insights', 'evidence', 'data',
    'study', 'findings', 'methodology', 'conclusion', 'implications'
  ],
  SENTIMENT_WEIGHTS: {
    POSITIVE: 1.1,
    NEUTRAL: 1.0,
    NEGATIVE: 0.9,
  },
} as const;

// Network Analysis
export const NETWORK_METRICS = {
  MIN_NODES_FOR_ANALYSIS: 10,
  MAX_DEPTH: 6, // Six degrees of separation
  CLUSTERING_THRESHOLD: 0.3,
  DENSITY_THRESHOLD: 0.1,
  VIRAL_COEFFICIENT_THRESHOLD: 1.5,
} as const;

// Solana Configuration
export const SOLANA_CONFIG = {
  NETWORK: process.env.NEXT_PUBLIC_SOLANA_NETWORK || 'devnet',
  RPC_URL: {
    'mainnet-beta': 'https://api.mainnet-beta.solana.com',
    'devnet': 'https://api.devnet.solana.com',
    'testnet': 'https://api.testnet.solana.com',
    'localnet': 'http://localhost:8899',
  },
  PROGRAM_ID: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS',
  TOKEN_MINT: 'ECHTokenMintAddress', // To be replaced with actual mint address
  COMMITMENT: 'confirmed' as const,
} as const;

// UI Constants
export const UI_CONFIG = {
  COLORS: {
    PRIMARY: '#0F0F23',      // Deep Space Blue
    SECONDARY: '#6366F1',    // Electric Purple
    ACCENT: '#10B981',       // Data Green
    SUCCESS: '#10B981',
    WARNING: '#F59E0B',
    ERROR: '#EF4444',
    BACKGROUND: '#000000',
    SURFACE: '#1A1A2E',
    TEXT_PRIMARY: '#FFFFFF',
    TEXT_SECONDARY: '#9CA3AF',
  },
  ANIMATIONS: {
    FAST: 150,
    NORMAL: 300,
    SLOW: 500,
    EXTRA_SLOW: 1000,
  },
  BREAKPOINTS: {
    SM: '640px',
    MD: '768px',
    LG: '1024px',
    XL: '1280px',
    '2XL': '1536px',
  },
} as const;

// Validation Rules
export const VALIDATION_RULES = {
  USERNAME: {
    MIN_LENGTH: 3,
    MAX_LENGTH: 30,
    PATTERN: /^[a-zA-Z0-9_-]+$/,
  },
  EMAIL: {
    PATTERN: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
  },
  CONTENT: {
    MIN_LENGTH: 10,
    MAX_LENGTH: 5000,
  },
  WALLET_ADDRESS: {
    LENGTH: 44,
    PATTERN: /^[1-9A-HJ-NP-Za-km-z]{43,44}$/,
  },
} as const;

// Error Messages
export const ERROR_MESSAGES = {
  NETWORK_ERROR: 'Network connection failed. Please try again.',
  AUTHENTICATION_FAILED: 'Authentication failed. Please log in again.',
  INVALID_INPUT: 'Invalid input data provided.',
  CONTENT_TOO_SHORT: 'Content must be at least 10 characters long.',
  CONTENT_TOO_LONG: 'Content exceeds maximum length limit.',
  ECHO_INDEX_CALCULATION_FAILED: 'Failed to calculate Echo Index. Please try again.',
  WALLET_CONNECTION_FAILED: 'Failed to connect wallet. Please try again.',
  TRANSACTION_FAILED: 'Transaction failed. Please try again.',
  RATE_LIMIT_EXCEEDED: 'Rate limit exceeded. Please wait before making more requests.',
} as const;

// Success Messages
export const SUCCESS_MESSAGES = {
  CONTENT_CREATED: 'Content created successfully!',
  ECHO_INDEX_CALCULATED: 'Echo Index calculated successfully!',
  REWARD_DISTRIBUTED: 'Reward distributed successfully!',
  WALLET_CONNECTED: 'Wallet connected successfully!',
  PROFILE_UPDATED: 'Profile updated successfully!',
  PROPAGATION_RECORDED: 'Propagation recorded successfully!',
} as const;

// Feature Flags
export const FEATURE_FLAGS = {
  ENABLE_REAL_TIME_UPDATES: true,
  ENABLE_ADVANCED_ANALYTICS: true,
  ENABLE_WALLET_INTEGRATION: true,
  ENABLE_SOCIAL_LOGIN: false,
  ENABLE_NOTIFICATIONS: true,
  ENABLE_DARK_MODE: true,
  ENABLE_BETA_FEATURES: false,
} as const;

// Time Constants
export const TIME_CONSTANTS = {
  SECOND: 1000,
  MINUTE: 60 * 1000,
  HOUR: 60 * 60 * 1000,
  DAY: 24 * 60 * 60 * 1000,
  WEEK: 7 * 24 * 60 * 60 * 1000,
  MONTH: 30 * 24 * 60 * 60 * 1000,
} as const; 