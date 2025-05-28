// Common types shared across the EchoLayer ecosystem

// ====================================
// CORE TYPES
// ====================================

export interface User {
  id: string;
  walletAddress: string;
  username?: string;
  email?: string;
  displayName?: string;
  avatarUrl?: string;
  bio?: string;
  wallet_address?: string; // Legacy field for backward compatibility
  echoScore: number;
  echo_score?: number; // Legacy field for backward compatibility
  totalRewards: number;
  total_content_created?: number;
  total_rewards_earned?: number;
  rank?: number;
  isVerified: boolean;
  preferences: UserPreferences;
  connectedPlatforms: Platform[];
  social_accounts?: SocialAccount[];
  createdAt: string;
  updatedAt: string;
  created_at?: string; // Legacy field for backward compatibility
  updated_at?: string; // Legacy field for backward compatibility
}

export interface UserPreferences {
  notifications: boolean;
  emailNotifications: boolean;
  publicProfile: boolean;
  analyticsSharing: boolean;
  theme: Theme;
  language: string;
}

export enum Theme {
  LIGHT = 'light',
  DARK = 'dark',
  AUTO = 'auto',
}

export interface SocialAccount {
  id: string;
  user_id: string;
  platform: SocialPlatform | Platform;
  account_id: string;
  username: string;
  verified: boolean;
  follower_count?: number;
  created_at: string;
}

export interface Content {
  id: string;
  userId: string;
  author_id?: string; // Legacy field for backward compatibility
  platform: Platform | SocialPlatform;
  externalId: string;
  original_url?: string; // Legacy field for backward compatibility
  contentType: ContentType;
  title: string;
  body: string;
  text?: string; // Legacy field for backward compatibility
  mediaUrls: string[];
  tags: string[];
  echoIndex: number;
  echo_index?: EchoIndex; // Legacy field for backward compatibility
  propagationCount: number;
  propagation_count?: number; // Legacy field for backward compatibility
  totalRewards: number;
  total_interactions?: number; // Legacy field for backward compatibility
  status: ContentStatus;
  platformMetadata: Record<string, any>;
  createdAt: string;
  updatedAt: string;
  created_at?: string; // Legacy field for backward compatibility
  updated_at?: string; // Legacy field for backward compatibility
}

// Platform enum with all supported platforms
export enum Platform {
  TWITTER = 'twitter',
  TELEGRAM = 'telegram',
  LINKEDIN = 'linkedin',
  YOUTUBE = 'youtube',
  INSTAGRAM = 'instagram',
  TIKTOK = 'tiktok',
  REDDIT = 'reddit',
}

// Legacy alias for backward compatibility
export const SocialPlatform = Platform;
export type SocialPlatform = Platform;

export enum ContentType {
  TEXT = 'text',
  IMAGE = 'image',
  VIDEO = 'video',
  AUDIO = 'audio',
  DOCUMENT = 'document',
  LINK = 'link',
  POLL = 'poll',
}

export enum ContentStatus {
  DRAFT = 'draft',
  ACTIVE = 'active',
  PUBLISHED = 'published',
  ANALYZING = 'analyzing',
  PROPAGATING = 'propagating',
  ARCHIVED = 'archived',
  DELETED = 'deleted',
  FLAGGED = 'flagged',
}

export enum UserRole {
  USER = 'user',
  CREATOR = 'creator',
  INFLUENCER = 'influencer',
  MODERATOR = 'moderator',
  ADMIN = 'admin',
}

// ====================================
// ECHO INDEX TYPES
// ====================================

export interface EchoIndexData {
  id: string;
  contentId: string;
  odfScore: number; // Organic Discovery Factor (0-100)
  awrScore: number; // Attention Weighted Reach (0-100)
  tpmScore: number; // Time-based Propagation Metric (0-100)
  qfScore: number;  // Quality Factor (0-100)
  finalScore: number; // Final Echo Index score (0-100)
  calculatedAt: Date;
  metadata?: EchoIndexMetadata;
}

export interface EchoIndexMetadata {
  totalPropagations: number;
  platformsReached: number;
  organicRatio: number; // 0-1
  engagementRate: number; // 0-1
  propagationVelocity?: number;
  audienceQuality?: number;
  contentOriginality?: number;
  [key: string]: any;
}

// Legacy Echo Index interface for backward compatibility
export interface EchoIndex {
  originality_depth_factor: number;    // ODF: 0.0-1.0
  audience_weight_rating: number;      // AWR: 0.0-1.0
  transmission_path_mapping: number;   // TPM: 0.0-1.0
  quote_frequency: number;             // QF: 0.0-1.0
  overall_score: number;               // Weighted average: 0.0-1.0
}

export interface EchoIndexCalculation {
  contentId: string;
  version: string;
  components: {
    odf: EchoIndexComponent;
    awr: EchoIndexComponent;
    tpm: EchoIndexComponent;
    qf: EchoIndexComponent;
  };
  finalScore: number;
  calculatedAt: string;
}

export interface EchoIndexComponent {
  score: number;
  weight: number;
  weightedScore: number;
  factors: Record<string, number>;
}

export interface EchoLoop {
  propagation_id: string;
  content_id: string;
  resonance_score: number;
  amplification_factor: number;
  viral_coefficient: number;
  network_reach: number;
  created_at: string;
}

// ====================================
// PROPAGATION TYPES
// ====================================

export interface Propagation {
  id: string;
  contentId: string;
  content_id?: string; // Legacy field for backward compatibility
  sourceUserId?: string;
  from_user_id?: string; // Legacy field for backward compatibility
  targetUserId?: string;
  to_user_id?: string; // Legacy field for backward compatibility
  propagationType: PropagationType;
  sourcePlatform: Platform;
  targetPlatform: Platform;
  platform?: Platform; // Legacy field for backward compatibility
  sourceExternalId?: string;
  targetExternalId?: string;
  echoBoost: number;
  rewardAmount: number;
  depth?: number; // Legacy field for backward compatibility
  weight?: number; // Legacy field for backward compatibility
  influence_score?: number; // Legacy field for backward compatibility
  engagementMetrics: EngagementMetrics;
  metadata: Record<string, any>;
  createdAt: string;
  timestamp?: string; // Legacy field for backward compatibility
}

export enum PropagationType {
  SHARE = 'share',
  REPOST = 'repost',
  QUOTE = 'quote',
  REPLY = 'reply',
  MENTION = 'mention',
  LINK = 'link',
  EMBED = 'embed',
  CROSS_POST = 'cross_post',
}

export interface EngagementMetrics {
  views: number;
  likes: number;
  comments: number;
  shares: number;
  reaches: number;
  clicks: number;
  saves: number;
}

export interface PropagationNetwork {
  nodes: PropagationNode[];
  edges: PropagationEdge[];
  metrics: NetworkMetrics;
}

export interface PropagationNode {
  id: string;
  userId: string;
  platform: Platform;
  influence: number;
  echoScore: number;
}

export interface PropagationEdge {
  sourceId: string;
  targetId: string;
  weight: number;
  propagationType: PropagationType;
  timestamp: string;
}

export interface NetworkMetrics {
  totalNodes: number;
  totalEdges: number;
  density: number;
  averagePathLength: number;
  clusteringCoefficient: number;
  // Legacy fields for backward compatibility
  total_nodes?: number;
  total_edges?: number;
  node_count?: number;
  edge_count?: number;
  clustering_coefficient?: number;
  average_path_length?: number;
  network_density?: number;
}

// ====================================
// REWARD TYPES
// ====================================

export interface Reward {
  id: string;
  userId: string;
  contentId?: string;
  propagationId?: string;
  rewardType: RewardType;
  amount: number;
  tokenSymbol: string;
  reason: string;
  status: RewardStatus;
  transactionHash?: string;
  blockchainNetwork: string;
  metadata: Record<string, any>;
  createdAt: string;
  updatedAt: string;
}

export interface EchoDrop {
  id: string;
  user_id: string;
  content_id?: string;
  points: number;
  reason: RewardReason;
  multiplier: number;
  transaction_hash?: string;
  timestamp: string;
}

export enum RewardType {
  CONTENT_CREATION = 'content_creation',
  CONTENT_PROPAGATION = 'content_propagation',
  PLATFORM_CONNECTION = 'platform_connection',
  MILESTONE_ACHIEVEMENT = 'milestone_achievement',
  REFERRAL_BONUS = 'referral_bonus',
  COMMUNITY_CONTRIBUTION = 'community_contribution',
}

export enum RewardReason {
  CONTENT_CREATION = 'content_creation',
  HIGH_ECHO_INDEX = 'high_echo_index',
  VIRAL_PROPAGATION = 'viral_propagation',
  QUALITY_INTERACTION = 'quality_interaction',
  EARLY_ADOPTION = 'early_adoption',
  COMMUNITY_CONTRIBUTION = 'community_contribution',
  REFERRAL_BONUS = 'referral_bonus',
}

export enum RewardStatus {
  PENDING = 'pending',
  PROCESSING = 'processing',
  COMPLETED = 'completed',
  FAILED = 'failed',
  CANCELLED = 'cancelled',
}

export interface WalletBalance {
  totalBalance: number;
  availableBalance: number;
  pendingBalance: number;
  tokenSymbol: string;
  walletAddress: string;
  lastUpdated: string;
}

export interface WalletConnection {
  address: string;
  provider: WalletProvider;
  network: SolanaNetwork;
  balance: number;
  connected_at: string;
}

export enum WalletProvider {
  PHANTOM = 'phantom',
  SOLFLARE = 'solflare',
  BACKPACK = 'backpack',
  GLOW = 'glow',
  MPC_WALLET = 'mpc_wallet',
}

// ====================================
// API TYPES
// ====================================

export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error?: string;
  timestamp: string;
}

export interface PaginationParams {
  page: number;
  limit: number;
  sort?: string;
  order?: SortOrder;
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    total_pages: number;
  };
}

export enum SortOrder {
  ASC = 'asc',
  DESC = 'desc',
}

// ====================================
// ANALYTICS TYPES
// ====================================

export interface Analytics {
  timeframe: AnalyticsTimeframe;
  total_content: number;
  total_users: number;
  total_propagations: number;
  average_echo_score: number;
  top_performers: ContentSummary[];
  platform_distribution: Record<Platform, number>;
  trend_data: TrendDataPoint[];
}

export interface ContentSummary {
  id: string;
  text: string;
  platform: Platform;
  echo_score: number;
  propagation_count: number;
  author_username: string;
  created_at: string;
}

export interface TrendDataPoint {
  date: string;
  average_score: number;
  content_count: number;
  propagation_count: number;
  active_users: number;
}

export enum AnalyticsTimeframe {
  HOUR_24 = '24h',
  DAYS_7 = '7d',
  DAYS_30 = '30d',
  DAYS_90 = '90d',
  YEAR_1 = '1y',
  ALL_TIME = 'all',
}

export interface AnalyticsEvent {
  id: string;
  userId?: string;
  sessionId: string;
  eventType: EventType;
  eventName: string;
  properties: Record<string, any>;
  timestamp: string;
}

export enum EventType {
  USER_SIGNUP = 'user_signup',
  USER_LOGIN = 'user_login',
  PLATFORM_CONNECTED = 'platform_connected',
  CONTENT_CREATED = 'content_created',
  CONTENT_SHARED = 'content_shared',
  REWARD_EARNED = 'reward_earned',
  REWARD_WITHDRAWN = 'reward_withdrawn',
  ECHO_INDEX_CALCULATED = 'echo_index_calculated',
  API_REQUEST = 'api_request',
}

export interface UserAnalytics {
  userId: string;
  timeframe: string;
  summary: {
    totalEchoScore: number;
    contentCount: number;
    propagationCount: number;
    rewardsEarned: number;
    rankChange: number;
  };
  echoScoreTrend: Array<{
    date: string;
    score: number;
  }>;
  platformBreakdown: Record<string, {
    contentCount: number;
    avgEchoIndex: number;
    totalRewards: number;
  }>;
  topContent: Array<{
    id: string;
    title: string;
    echoIndex: number;
    rewards: number;
  }>;
}

// ====================================
// BLOCKCHAIN TYPES
// ====================================

export interface Transaction {
  id: string;
  hash: string;
  type: TransactionType;
  status: TransactionStatus;
  amount: number;
  fromAddress: string;
  toAddress: string;
  gasUsed: number;
  gasPrice: number;
  blockNumber: number;
  timestamp: string;
}

export enum TransactionType {
  REWARD = 'reward',
  WITHDRAWAL = 'withdrawal',
  STAKE = 'stake',
  UNSTAKE = 'unstake',
}

export enum TransactionStatus {
  PENDING = 'pending',
  CONFIRMED = 'confirmed',
  FAILED = 'failed',
}

export enum SolanaNetwork {
  MAINNET = 'mainnet-beta',
  DEVNET = 'devnet',
  TESTNET = 'testnet',
  LOCALNET = 'localnet',
}

// ====================================
// UTILITY TYPES
// ====================================

export type Optional<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

export type RequiredKeys<T> = {
  [K in keyof T]-?: {} extends Pick<T, K> ? never : K;
}[keyof T];

export type OptionalKeys<T> = {
  [K in keyof T]-?: {} extends Pick<T, K> ? K : never;
}[keyof T];

// Legacy utility types for backward compatibility
export type CreateUserRequest = Pick<User, 'username' | 'email'> & {
  social_accounts: Omit<SocialAccount, 'id' | 'user_id' | 'created_at'>[];
};

export type CreateContentRequest = Pick<Content, 'body' | 'platformMetadata'> & {
  text?: string;
  platform: Platform;
  original_url?: string;
};

export type UpdateUserRequest = Partial<Pick<User, 'username' | 'email' | 'walletAddress'>>;

export type EchoIndexWeights = {
  odf: number;  // Originality Depth Factor weight
  awr: number;  // Audience Weight Rating weight
  tpm: number;  // Transmission Path Mapping weight
  qf: number;   // Quote Frequency weight
};

export type InfluenceMetrics = {
  reach: number;
  engagement_rate: number;
  viral_coefficient: number;
  influence_score: number;
  amplification_factor: number;
};

// ====================================
// TYPE GUARDS
// ====================================

export function isValidEchoIndex(index: any): index is EchoIndex {
  return (
    typeof index === 'object' &&
    typeof index.originality_depth_factor === 'number' &&
    typeof index.audience_weight_rating === 'number' &&
    typeof index.transmission_path_mapping === 'number' &&
    typeof index.quote_frequency === 'number' &&
    typeof index.overall_score === 'number' &&
    index.originality_depth_factor >= 0 && index.originality_depth_factor <= 1 &&
    index.audience_weight_rating >= 0 && index.audience_weight_rating <= 1 &&
    index.transmission_path_mapping >= 0 && index.transmission_path_mapping <= 1 &&
    index.quote_frequency >= 0 && index.quote_frequency <= 1 &&
    index.overall_score >= 0 && index.overall_score <= 1
  );
}

export function isValidPlatform(platform: string): platform is Platform {
  return Object.values(Platform).includes(platform as Platform);
}

export function isValidSocialPlatform(platform: string): platform is SocialPlatform {
  return Object.values(Platform).includes(platform as Platform);
}

export function isValidContentType(type: string): type is ContentType {
  return Object.values(ContentType).includes(type as ContentType);
}

export function isValidPropagationType(type: string): type is PropagationType {
  return Object.values(PropagationType).includes(type as PropagationType);
}

export function isValidRewardType(type: string): type is RewardType {
  return Object.values(RewardType).includes(type as RewardType);
}

// ====================================
// CONSTANTS
// ====================================

export const ECHO_INDEX_WEIGHTS = {
  ODF: 0.30, // Organic Discovery Factor
  AWR: 0.25, // Attention Weighted Reach
  TPM: 0.25, // Time-based Propagation Metric
  QF: 0.20,  // Quality Factor
} as const;

export const PLATFORM_CONFIGS = {
  [Platform.TWITTER]: {
    name: 'Twitter',
    maxContentLength: 280,
    supportedTypes: [ContentType.TEXT, ContentType.IMAGE, ContentType.VIDEO],
  },
  [Platform.TELEGRAM]: {
    name: 'Telegram',
    maxContentLength: 4096,
    supportedTypes: [ContentType.TEXT, ContentType.IMAGE, ContentType.VIDEO, ContentType.DOCUMENT],
  },
  [Platform.LINKEDIN]: {
    name: 'LinkedIn',
    maxContentLength: 3000,
    supportedTypes: [ContentType.TEXT, ContentType.IMAGE, ContentType.VIDEO, ContentType.DOCUMENT],
  },
} as const;

export const DEFAULT_PAGINATION = {
  page: 1,
  limit: 20,
  sort: 'createdAt',
  order: SortOrder.DESC,
} as const; 