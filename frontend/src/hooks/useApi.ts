import { useState } from 'react';
import { User, Content, ApiResponse } from '@echolayer/shared';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1';

interface UseApiResult {
  loading: boolean;
  error: string | null;
  createUser: (userData: { username: string; email: string }) => Promise<User | null>;
  getUser: (userId: string) => Promise<User | null>;
  createContent: (contentData: { 
    title: string; 
    body: string; 
    platform: string; 
    contentType: string;
  }) => Promise<Content | null>;
  getContent: (contentId: string) => Promise<Content | null>;
  clearError: () => void;
}

export function useApi(): UseApiResult {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const clearError = () => setError(null);

  const apiCall = async <T>(url: string, options?: RequestInit): Promise<T | null> => {
    setLoading(true);
    setError(null);

    try {
      const response = await fetch(`${API_BASE_URL}${url}`, {
        headers: {
          'Content-Type': 'application/json',
          ...options?.headers,
        },
        ...options,
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data: ApiResponse<T> = await response.json();
      
      if (!data.success) {
        throw new Error(data.error || 'API call failed');
      }

      return data.data;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error occurred';
      setError(errorMessage);
      console.error('API call failed:', errorMessage);
      return null;
    } finally {
      setLoading(false);
    }
  };

  const createUser = async (userData: { username: string; email: string }): Promise<User | null> => {
    return apiCall<User>('/users', {
      method: 'POST',
      body: JSON.stringify({
        ...userData,
        social_accounts: []
      }),
    });
  };

  const getUser = async (userId: string): Promise<User | null> => {
    return apiCall<User>(`/users/${userId}`);
  };

  const createContent = async (contentData: { 
    title: string; 
    body: string; 
    platform: string; 
    contentType: string;
  }): Promise<Content | null> => {
    return apiCall<Content>('/content', {
      method: 'POST',
      body: JSON.stringify({
        title: contentData.title,
        body: contentData.body,
        platform: contentData.platform,
        contentType: contentData.contentType,
        mediaUrls: [],
        tags: [],
        platformMetadata: {}
      }),
    });
  };

  const getContent = async (contentId: string): Promise<Content | null> => {
    return apiCall<Content>(`/content/${contentId}`);
  };

  return {
    loading,
    error,
    createUser,
    getUser,
    createContent,
    getContent,
    clearError
  };
} 