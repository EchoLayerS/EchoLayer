import { useState, useEffect } from 'react';
import { EchoIndex, EchoIndexData } from '@echolayer/shared';

export interface UseEchoIndexReturn {
  echoIndex: EchoIndex | null;
  loading: boolean;
  error: string | null;
  refetch: () => void;
}

export function useEchoIndex(contentId: string): UseEchoIndexReturn {
  const [echoIndex, setEchoIndex] = useState<EchoIndex | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchEchoIndex = async () => {
    if (!contentId) return;
    
    setLoading(true);
    setError(null);
    
    try {
      // Mock implementation for development
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // Generate mock Echo Index data
      const mockIndex: EchoIndex = {
        originality_depth_factor: Math.random() * 0.5 + 0.5, // 0.5-1.0
        audience_weight_rating: Math.random() * 0.4 + 0.6, // 0.6-1.0
        transmission_path_mapping: Math.random() * 0.6 + 0.4, // 0.4-1.0
        quote_frequency: Math.random() * 0.3 + 0.7, // 0.7-1.0
        overall_score: 0, // Will be calculated below
      };

      // Calculate overall score (weighted average)
      mockIndex.overall_score = calculateWeightedScore(
        mockIndex.originality_depth_factor,
        mockIndex.audience_weight_rating,
        mockIndex.transmission_path_mapping,
        mockIndex.quote_frequency
      );

      setEchoIndex(mockIndex);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch Echo Index');
    } finally {
      setLoading(false);
    }
  };

  const refetch = () => {
    fetchEchoIndex();
  };

  useEffect(() => {
    fetchEchoIndex();
  }, [contentId]);

  return {
    echoIndex,
    loading,
    error,
    refetch,
  };
}

// Helper function to calculate Echo Index score
function calculateWeightedScore(
  odf: number,
  awr: number,
  tpm: number,
  qf: number
): number {
  // Weights for each component (should sum to 1.0)
  const weights = {
    odf: 0.30, // Organic Discovery Factor
    awr: 0.25, // Attention Weighted Reach
    tpm: 0.25, // Time-based Propagation Metric
    qf: 0.20,  // Quality Factor
  };

  return (
    odf * weights.odf +
    awr * weights.awr +
    tpm * weights.tpm +
    qf * weights.qf
  );
} 