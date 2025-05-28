import { useState, useEffect, useRef, useCallback } from 'react';
import { WebSocketMessage } from '../types';

interface UseWebSocketReturn {
  isConnected: boolean;
  connectionState: 'connecting' | 'connected' | 'disconnected' | 'error';
  lastMessage: WebSocketMessage | null;
  error: string | null;
  sendMessage: (message: any) => void;
  connect: () => void;
  disconnect: () => void;
}

interface WebSocketConfig {
  url: string;
  protocols?: string | string[];
  reconnectAttempts?: number;
  reconnectInterval?: number;
  heartbeatInterval?: number;
}

export function useWebSocket(config: WebSocketConfig): UseWebSocketReturn {
  const [isConnected, setIsConnected] = useState(false);
  const [connectionState, setConnectionState] = useState<'connecting' | 'connected' | 'disconnected' | 'error'>('disconnected');
  const [lastMessage, setLastMessage] = useState<WebSocketMessage | null>(null);
  const [error, setError] = useState<string | null>(null);
  
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const heartbeatIntervalRef = useRef<NodeJS.Timeout | null>(null);
  const reconnectAttemptsRef = useRef(0);
  const maxReconnectAttempts = config.reconnectAttempts || 5;
  const reconnectInterval = config.reconnectInterval || 3000;
  const heartbeatInterval = config.heartbeatInterval || 30000;

  const clearTimers = useCallback(() => {
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current);
      reconnectTimeoutRef.current = null;
    }
    if (heartbeatIntervalRef.current) {
      clearInterval(heartbeatIntervalRef.current);
      heartbeatIntervalRef.current = null;
    }
  }, []);

  const startHeartbeat = useCallback(() => {
    if (heartbeatIntervalRef.current) {
      clearInterval(heartbeatIntervalRef.current);
    }

    heartbeatIntervalRef.current = setInterval(() => {
      if (wsRef.current?.readyState === WebSocket.OPEN) {
        wsRef.current.send(JSON.stringify({ type: 'ping', timestamp: new Date().toISOString() }));
      }
    }, heartbeatInterval);
  }, [heartbeatInterval]);

  const connect = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      return;
    }

    setConnectionState('connecting');
    setError(null);

    try {
      wsRef.current = new WebSocket(config.url, config.protocols);

      wsRef.current.onopen = () => {
        setIsConnected(true);
        setConnectionState('connected');
        setError(null);
        reconnectAttemptsRef.current = 0;
        startHeartbeat();
        
        console.log('WebSocket connected to:', config.url);
      };

      wsRef.current.onmessage = (event) => {
        try {
          const message: WebSocketMessage = JSON.parse(event.data);
          
          // Handle pong messages
          if (message.type === 'pong') {
            return;
          }

          setLastMessage(message);
        } catch (err) {
          console.error('Failed to parse WebSocket message:', err);
        }
      };

      wsRef.current.onclose = (event) => {
        setIsConnected(false);
        setConnectionState('disconnected');
        clearTimers();
        
        console.log('WebSocket disconnected:', event.code, event.reason);

        // Attempt reconnection if not a normal closure
        if (event.code !== 1000 && reconnectAttemptsRef.current < maxReconnectAttempts) {
          reconnectAttemptsRef.current++;
          console.log(`Attempting to reconnect (${reconnectAttemptsRef.current}/${maxReconnectAttempts})...`);
          
          reconnectTimeoutRef.current = setTimeout(() => {
            connect();
          }, reconnectInterval);
        } else if (reconnectAttemptsRef.current >= maxReconnectAttempts) {
          setError('Maximum reconnection attempts reached');
          setConnectionState('error');
        }
      };

      wsRef.current.onerror = (event) => {
        setError('WebSocket connection error');
        setConnectionState('error');
        console.error('WebSocket error:', event);
      };

    } catch (err) {
      setError(`Failed to connect: ${err instanceof Error ? err.message : 'Unknown error'}`);
      setConnectionState('error');
    }
  }, [config.url, config.protocols, maxReconnectAttempts, reconnectInterval, startHeartbeat, clearTimers]);

  const disconnect = useCallback(() => {
    clearTimers();
    reconnectAttemptsRef.current = maxReconnectAttempts; // Prevent automatic reconnection
    
    if (wsRef.current) {
      wsRef.current.close(1000, 'Manual disconnect');
      wsRef.current = null;
    }
    
    setIsConnected(false);
    setConnectionState('disconnected');
    setError(null);
  }, [clearTimers, maxReconnectAttempts]);

  const sendMessage = useCallback((message: any) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      try {
        const messageWithTimestamp = {
          ...message,
          timestamp: new Date().toISOString(),
        };
        wsRef.current.send(JSON.stringify(messageWithTimestamp));
      } catch (err) {
        console.error('Failed to send WebSocket message:', err);
        setError(`Failed to send message: ${err instanceof Error ? err.message : 'Unknown error'}`);
      }
    } else {
      console.warn('WebSocket is not connected');
      setError('WebSocket is not connected');
    }
  }, []);

  // Connect on mount if URL is provided
  useEffect(() => {
    if (config.url) {
      connect();
    }

    // Cleanup on unmount
    return () => {
      disconnect();
    };
  }, [config.url]); // Only reconnect if URL changes

  // Handle page visibility changes
  useEffect(() => {
    const handleVisibilityChange = () => {
      if (document.visibilityState === 'visible' && !isConnected && config.url) {
        // Reconnect when page becomes visible
        connect();
      } else if (document.visibilityState === 'hidden') {
        // Optionally disconnect when page is hidden to save resources
        // disconnect();
      }
    };

    document.addEventListener('visibilitychange', handleVisibilityChange);
    return () => {
      document.removeEventListener('visibilitychange', handleVisibilityChange);
    };
  }, [isConnected, config.url, connect]);

  return {
    isConnected,
    connectionState,
    lastMessage,
    error,
    sendMessage,
    connect,
    disconnect,
  };
}

// Specialized hook for EchoLayer real-time updates
export function useEchoLayerWebSocket() {
  const wsUrl = process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:8080/ws';
  
  const {
    isConnected,
    connectionState,
    lastMessage,
    error,
    sendMessage,
    connect,
    disconnect,
  } = useWebSocket({
    url: wsUrl,
    reconnectAttempts: 5,
    reconnectInterval: 3000,
    heartbeatInterval: 30000,
  });

  // Subscribe to specific content updates
  const subscribeToContent = useCallback((contentId: string) => {
    sendMessage({
      type: 'subscribe',
      payload: {
        channel: 'content_updates',
        content_id: contentId,
      },
    });
  }, [sendMessage]);

  // Subscribe to user's propagation events
  const subscribeToUserPropagation = useCallback((userId: string) => {
    sendMessage({
      type: 'subscribe',
      payload: {
        channel: 'user_propagation',
        user_id: userId,
      },
    });
  }, [sendMessage]);

  // Subscribe to global Echo Index updates
  const subscribeToEchoIndex = useCallback(() => {
    sendMessage({
      type: 'subscribe',
      payload: {
        channel: 'echo_index_updates',
      },
    });
  }, [sendMessage]);

  // Subscribe to reward notifications
  const subscribeToRewards = useCallback((userId: string) => {
    sendMessage({
      type: 'subscribe',
      payload: {
        channel: 'user_rewards',
        user_id: userId,
      },
    });
  }, [sendMessage]);

  return {
    isConnected,
    connectionState,
    lastMessage,
    error,
    connect,
    disconnect,
    subscribeToContent,
    subscribeToUserPropagation,
    subscribeToEchoIndex,
    subscribeToRewards,
  };
} 