import { useState, useEffect, useCallback } from 'react';
import { User } from '@echolayer/shared';
import { useAppStore } from '../store/useAppStore';

interface WalletConnection {
  address: string;
  chainId: number;
  isConnected: boolean;
}

interface UseAuthReturn {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  walletConnection: WalletConnection | null;
  login: (credentials: LoginCredentials) => Promise<boolean>;
  logout: () => void;
  connectWallet: () => Promise<boolean>;
  disconnectWallet: () => void;
  clearError: () => void;
}

interface LoginCredentials {
  email?: string;
  password?: string;
  walletAddress?: string;
  signature?: string;
}

export function useAuth(): UseAuthReturn {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [walletConnection, setWalletConnection] = useState<WalletConnection | null>(null);
  
  const { user, isAuthenticated, setUser, setAuthenticated } = useAppStore();

  const clearError = useCallback(() => setError(null), []);

  // Check for existing session on mount
  useEffect(() => {
    const checkExistingSession = async () => {
      const token = localStorage.getItem('echolayer_token');
      if (token) {
        try {
          // Verify token with backend
          const response = await fetch('/api/v1/auth/verify', {
            headers: {
              'Authorization': `Bearer ${token}`,
            },
          });

          if (response.ok) {
            const userData = await response.json();
            setUser(userData.user);
            setAuthenticated(true);
          } else {
            localStorage.removeItem('echolayer_token');
          }
        } catch (err) {
          console.error('Session verification failed:', err);
          localStorage.removeItem('echolayer_token');
        }
      }
    };

    checkExistingSession();
  }, [setUser, setAuthenticated]);

  // Check for existing wallet connection
  useEffect(() => {
    const checkWalletConnection = async () => {
      if (typeof window !== 'undefined' && (window as any).solana) {
        try {
          const response = await (window as any).solana.connect({ onlyIfTrusted: true });
          setWalletConnection({
            address: response.publicKey.toString(),
            chainId: 0, // Solana doesn't use chainId like Ethereum
            isConnected: true,
          });
        } catch (err) {
          // No trusted connection available
        }
      }
    };

    checkWalletConnection();
  }, []);

  const login = useCallback(async (credentials: LoginCredentials): Promise<boolean> => {
    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch('/api/v1/auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(credentials),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Login failed');
      }

      const data = await response.json();
      
      // Store token
      localStorage.setItem('echolayer_token', data.token);
      
      // Update state
      setUser(data.user);
      setAuthenticated(true);

      return true;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Login failed';
      setError(errorMessage);
      return false;
    } finally {
      setIsLoading(false);
    }
  }, [setUser, setAuthenticated]);

  const logout = useCallback(() => {
    // Clear local storage
    localStorage.removeItem('echolayer_token');
    
    // Clear state
    setUser(null);
    setAuthenticated(false);
    
    // Disconnect wallet if connected
    if (walletConnection?.isConnected) {
      disconnectWallet();
    }
  }, [walletConnection, setUser, setAuthenticated]);

  const connectWallet = useCallback(async (): Promise<boolean> => {
    setIsLoading(true);
    setError(null);

    try {
      if (typeof window === 'undefined' || !(window as any).solana) {
        throw new Error('Solana wallet not found. Please install Phantom or another Solana wallet.');
      }

      const response = await (window as any).solana.connect();
      const walletAddress = response.publicKey.toString();

      setWalletConnection({
        address: walletAddress,
        chainId: 0,
        isConnected: true,
      });

      // If user is not authenticated, attempt wallet-based login
      if (!isAuthenticated) {
        // Request signature for authentication
        const message = `Sign this message to authenticate with EchoLayer: ${Date.now()}`;
        const encodedMessage = new TextEncoder().encode(message);
        const signedMessage = await (window as any).solana.signMessage(encodedMessage);

        const loginSuccess = await login({
          walletAddress,
          signature: Buffer.from(signedMessage.signature).toString('hex'),
        });

        return loginSuccess;
      }

      return true;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Wallet connection failed';
      setError(errorMessage);
      return false;
    } finally {
      setIsLoading(false);
    }
  }, [isAuthenticated, login]);

  const disconnectWallet = useCallback(() => {
    if (typeof window !== 'undefined' && (window as any).solana) {
      (window as any).solana.disconnect();
    }
    
    setWalletConnection(null);
  }, []);

  return {
    user,
    isAuthenticated,
    isLoading,
    error,
    walletConnection,
    login,
    logout,
    connectWallet,
    disconnectWallet,
    clearError,
  };
} 