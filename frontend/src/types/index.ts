// Frontend-specific types only
// Note: Use @echolayer/shared for core domain types

export interface NavigationItem {
  label: string;
  href: string;
  icon?: string;
  badge?: string;
  children?: NavigationItem[];
}

export interface Toast {
  id: string;
  title: string;
  description?: string;
  type: 'success' | 'error' | 'warning' | 'info';
  duration?: number;
  action?: {
    label: string;
    onClick: () => void;
  };
}

export interface ThemeConfig {
  mode: 'light' | 'dark' | 'system';
  primaryColor: string;
  borderRadius: number;
  fontFamily: string;
}

export interface FrontendUserPreferences {
  theme: ThemeConfig;
  notifications: {
    email: boolean;
    push: boolean;
    echo_updates: boolean;
    rewards: boolean;
  };
  privacy: {
    show_profile: boolean;
    show_activity: boolean;
    allow_analytics: boolean;
  };
}

export interface DashboardStats {
  total_content: number;
  total_echo_score: number;
  total_rewards: number;
  rank: number;
  change_24h: {
    content: number;
    echo_score: number;
    rewards: number;
    rank: number;
  };
}

export interface ChartDataPoint {
  date: string;
  value: number;
  label?: string;
}

export interface EchoVisualizationData {
  nodes: Array<{
    id: string;
    label: string;
    size: number;
    color: string;
    type: 'user' | 'content' | 'platform';
  }>;
  edges: Array<{
    source: string;
    target: string;
    weight: number;
    type: string;
  }>;
}

export interface FormField {
  name: string;
  label: string;
  type: 'text' | 'email' | 'password' | 'textarea' | 'select' | 'checkbox' | 'radio';
  placeholder?: string;
  required?: boolean;
  validation?: {
    pattern?: RegExp;
    min?: number;
    max?: number;
    custom?: (value: any) => string | null;
  };
  options?: Array<{ value: string; label: string }>;
}

export interface LoadingState {
  isLoading: boolean;
  error: string | null;
  data: any;
}

export interface WebSocketMessage {
  type: string;
  payload: any;
  timestamp: string;
}

export interface NotificationState {
  toasts: Toast[];
  addToast: (toast: Omit<Toast, 'id'>) => void;
  removeToast: (id: string) => void;
  clearAll: () => void;
}

export interface RouteInfo {
  path: string;
  title: string;
  description?: string;
  requiresAuth?: boolean;
  roles?: string[];
}

export interface SEOData {
  title: string;
  description: string;
  keywords?: string[];
  image?: string;
  url?: string;
}

export interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full';
  children: React.ReactNode;
}

// Utility types for forms
export type FormData<T> = {
  [K in keyof T]: T[K];
};

export type FormErrors<T> = {
  [K in keyof T]?: string;
};

export type FormState<T> = {
  data: FormData<T>;
  errors: FormErrors<T>;
  isSubmitting: boolean;
  isValid: boolean;
};

// Component prop types
export type ButtonVariant = 'primary' | 'secondary' | 'outline' | 'ghost' | 'destructive';
export type ButtonSize = 'sm' | 'md' | 'lg';

export type InputVariant = 'default' | 'filled' | 'outline';
export type InputSize = 'sm' | 'md' | 'lg';

// Animation types
export type AnimationPreset = 'fadeIn' | 'slideUp' | 'slideDown' | 'slideLeft' | 'slideRight' | 'scale';

// Layout types
export interface LayoutProps {
  children: React.ReactNode;
  title?: string;
  description?: string;
  showHeader?: boolean;
  showFooter?: boolean;
  showSidebar?: boolean;
}

// Page component types
export interface PageProps {
  params?: Record<string, string>;
  searchParams?: Record<string, string | string[]>;
}

// Error boundary types
export interface ErrorBoundaryState {
  hasError: boolean;
  error?: Error;
  errorInfo?: React.ErrorInfo;
}

// Theme types
export type ThemeMode = 'light' | 'dark' | 'system';
export type ColorScheme = 'default' | 'ocean' | 'forest' | 'sunset' | 'midnight'; 