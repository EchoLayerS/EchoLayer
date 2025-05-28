'use client';

import React, { useState, useEffect, useMemo } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ChevronDown, ChevronUp, RefreshCw, TrendingUp, Clock, Users, Zap } from 'lucide-react';
import { EchoIndexData } from '@echolayer/shared';

interface EchoIndexProps {
  data: EchoIndexData | null;
  isLoading?: boolean;
  error?: string;
  contentId?: string;
  showBreakdown?: boolean;
  showTimestamp?: boolean;
  expandable?: boolean;
  animated?: boolean;
  realTimeUpdates?: boolean;
  enableCaching?: boolean;
  autoCalculate?: boolean;
  className?: string;
  size?: 'small' | 'medium' | 'large';
  variant?: 'default' | 'minimal' | 'detailed';
  onRefresh?: () => void;
  onDataUpdate?: (data: EchoIndexData) => void;
  alertThreshold?: number;
}

const EchoIndex: React.FC<EchoIndexProps> = ({
  data,
  isLoading = false,
  error,
  contentId,
  showBreakdown = false,
  showTimestamp = false,
  expandable = false,
  animated = true,
  realTimeUpdates = false,
  enableCaching = false,
  autoCalculate = false,
  className = '',
  size = 'medium',
  variant = 'default',
  onRefresh,
  onDataUpdate,
  alertThreshold = 30,
}) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [animatedScore, setAnimatedScore] = useState(0);

  // Animate score changes
  useEffect(() => {
    if (data && animated) {
      const timer = setTimeout(() => {
        setAnimatedScore(data.finalScore);
      }, 100);
      return () => clearTimeout(timer);
    } else if (data) {
      setAnimatedScore(data.finalScore);
    }
  }, [data, animated]);

  // Score color classification
  const getScoreColor = (score: number) => {
    if (score >= 80) return 'text-green-600';
    if (score >= 60) return 'text-yellow-600';
    if (score >= 40) return 'text-orange-600';
    return 'text-red-600';
  };

  // Score background color
  const getScoreBg = (score: number) => {
    if (score >= 80) return 'bg-green-50 border-green-200';
    if (score >= 60) return 'bg-yellow-50 border-yellow-200';
    if (score >= 40) return 'bg-orange-50 border-orange-200';
    return 'bg-red-50 border-red-200';
  };

  // Size classes
  const sizeClasses = {
    small: 'text-sm',
    medium: 'text-base',
    large: 'text-lg',
  };

  // Format timestamp
  const formatTimestamp = (date: Date) => {
    return new Intl.DateTimeFormat('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    }).format(date);
  };

  // Loading state
  if (isLoading) {
    return (
      <div
        data-testid="echo-index-loading"
        className={`flex items-center justify-center p-6 ${className}`}
      >
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <span className="ml-3 text-gray-600">Calculating Echo Index...</span>
      </div>
    );
  }

  // Error state
  if (error) {
    return (
      <div
        data-testid="echo-index-error"
        className={`p-4 border border-red-200 rounded-lg bg-red-50 ${className}`}
      >
        <div className="text-red-800">{error}</div>
      </div>
    );
  }

  // No data state
  if (!data) {
    return (
      <div className={`p-4 border border-gray-200 rounded-lg bg-gray-50 ${className}`}>
        <div className="text-gray-600">No Echo Index data available</div>
      </div>
    );
  }

  const componentBreakdown = [
    {
      label: 'Organic Discovery Factor',
      value: data.odfScore,
      icon: TrendingUp,
      description: 'Natural content discovery without paid promotion',
    },
    {
      label: 'Attention Weighted Reach',
      value: data.awrScore,
      icon: Users,
      description: 'Quality-adjusted reach and engagement',
    },
    {
      label: 'Time-based Propagation',
      value: data.tpmScore,
      icon: Clock,
      description: 'Speed and sustainability of propagation',
    },
    {
      label: 'Quality Factor',
      value: data.qfScore,
      icon: Zap,
      description: 'Content quality and originality',
    },
  ];

  return (
    <div
      data-testid="echo-index-container"
      role="region"
      aria-labelledby="echo-index-title"
      className={`border rounded-lg p-6 ${getScoreBg(data.finalScore)} ${className} ${sizeClasses[size]} size-${size} variant-${variant}`}
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <h3 id="echo-index-title" className="text-lg font-semibold text-gray-900">
          Echo Index Score
        </h3>
        {onRefresh && (
          <button
            onClick={onRefresh}
            className="p-2 rounded-lg hover:bg-gray-100 transition-colors"
            title="Refresh Echo Index"
          >
            <RefreshCw className="h-4 w-4" />
          </button>
        )}
      </div>

      {/* Main Score */}
      <div className="text-center mb-6">
        <motion.div
          data-testid="echo-index-score"
          aria-label={`Echo Index Score: ${data.finalScore} out of 100`}
          className={`text-4xl font-bold ${getScoreColor(data.finalScore)} ${animated ? 'animate-score-change' : ''}`}
          initial={animated ? { scale: 0.8, opacity: 0 } : false}
          animate={animated ? { scale: 1, opacity: 1 } : false}
          transition={{ duration: 0.5 }}
        >
          {animatedScore.toFixed(1)}
        </motion.div>
        <div className="text-sm text-gray-600 mt-1">out of 100</div>
      </div>

      {/* Low Score Alert */}
      {data.finalScore < alertThreshold && (
        <div
          data-testid="low-score-alert"
          className="mb-4 p-3 bg-amber-50 border border-amber-200 rounded-lg"
        >
          <div className="text-amber-800 text-sm">
            Score is below recommended threshold of {alertThreshold}. Consider optimizing content quality and engagement.
          </div>
        </div>
      )}

      {/* Component Breakdown */}
      {(showBreakdown || variant === 'detailed') && (
        <div className="space-y-3 mb-4">
          {componentBreakdown.map((component, index) => (
            <div key={component.label} className="flex items-center justify-between">
              <div className="flex items-center">
                <component.icon className="h-4 w-4 text-blue-600 mr-2" />
                <span className="text-sm text-gray-700">{component.label}</span>
              </div>
              <span className={`font-semibold ${getScoreColor(component.value)}`}>
                {component.value.toFixed(1)}
              </span>
            </div>
          ))}
        </div>
      )}

      {/* Expandable Details */}
      {expandable && (
        <div>
          <button
            data-testid="expand-details"
            onClick={() => setIsExpanded(!isExpanded)}
            className="flex items-center justify-between w-full py-2 text-sm text-blue-600 hover:text-blue-800"
          >
            <span>Additional Metrics</span>
            {isExpanded ? <ChevronUp className="h-4 w-4" /> : <ChevronDown className="h-4 w-4" />}
          </button>

          <AnimatePresence>
            {isExpanded && (
              <motion.div
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: 'auto', opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
                transition={{ duration: 0.3 }}
                className="mt-3 pt-3 border-t border-gray-200"
              >
                {data.metadata ? (
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-gray-600">Total Propagations:</span>
                      <span className="font-medium">{data.metadata.totalPropagations}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600">Platforms Reached:</span>
                      <span className="font-medium">{data.metadata.platformsReached}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600">Organic Ratio:</span>
                      <span className="font-medium">
                        {(data.metadata.organicRatio * 100).toFixed(0)}%
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600">Engagement Rate:</span>
                      <span className="font-medium">
                        {(data.metadata.engagementRate * 100).toFixed(0)}%
                      </span>
                    </div>
                  </div>
                ) : (
                  <div className="text-sm text-gray-500">No additional metrics available</div>
                )}
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      )}

      {/* Timestamp */}
      {showTimestamp && data.calculatedAt && (
        <div className="mt-4 pt-3 border-t border-gray-200 text-xs text-gray-500">
          Calculated at: {formatTimestamp(new Date(data.calculatedAt))}
        </div>
      )}
    </div>
  );
};

export default EchoIndex; 