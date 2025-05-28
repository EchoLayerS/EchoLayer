use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EchoMetrics {
    pub content_length: usize,
    pub word_count: usize,
    pub unique_words: usize,
    pub sentiment_score: f64,
    pub readability_score: f64,
    pub originality_markers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropagationMetrics {
    pub total_propagations: i32,
    pub unique_propagators: i32,
    pub platform_distribution: HashMap<String, i32>,
    pub propagation_velocity: f64,
    pub network_reach: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudienceMetrics {
    pub total_interactions: i32,
    pub quality_interactions: i32,
    pub audience_diversity: f64,
    pub influencer_ratio: f64,
    pub engagement_depth: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteMetrics {
    pub direct_quotes: i32,
    pub indirect_references: i32,
    pub discussion_threads: i32,
    pub citation_quality: f64,
}

pub struct EchoIndexCalculator;

impl EchoIndexCalculator {
    /// Calculate Originality Depth Factor (ODF)
    pub fn calculate_odf(content: &str, metrics: &EchoMetrics) -> f64 {
        let mut score = 0.0;

        // Base originality from content analysis
        let originality_base = Self::analyze_content_originality(content);
        score += originality_base * 0.4;

        // Unique word ratio
        let uniqueness_ratio = metrics.unique_words as f64 / metrics.word_count as f64;
        score += uniqueness_ratio * 0.3;

        // Sentiment and readability contribution
        score += (metrics.sentiment_score.abs() * 0.15);
        score += (metrics.readability_score * 0.15);

        score.min(1.0).max(0.0)
    }

    /// Calculate Audience Weight Rating (AWR)
    pub fn calculate_awr(audience_metrics: &AudienceMetrics) -> f64 {
        let mut score = 0.0;

        // Quality interaction ratio
        let quality_ratio = if audience_metrics.total_interactions > 0 {
            audience_metrics.quality_interactions as f64 / audience_metrics.total_interactions as f64
        } else {
            0.0
        };
        score += quality_ratio * 0.4;

        // Audience diversity
        score += audience_metrics.audience_diversity * 0.3;

        // Influencer engagement
        score += audience_metrics.influencer_ratio * 0.2;

        // Engagement depth
        score += audience_metrics.engagement_depth * 0.1;

        score.min(1.0).max(0.0)
    }

    /// Calculate Transmission Path Mapping (TPM)
    pub fn calculate_tpm(propagation_metrics: &PropagationMetrics) -> f64 {
        let mut score = 0.0;

        // Network reach factor
        let reach_factor = (propagation_metrics.network_reach as f64).ln() / 10.0;
        score += reach_factor.min(0.4);

        // Propagation velocity
        let velocity_factor = propagation_metrics.propagation_velocity / 100.0;
        score += velocity_factor.min(0.3);

        // Platform diversity
        let platform_diversity = propagation_metrics.platform_distribution.len() as f64 / 10.0;
        score += platform_diversity.min(0.3);

        score.min(1.0).max(0.0)
    }

    /// Calculate Quote Frequency (QF)
    pub fn calculate_qf(quote_metrics: &QuoteMetrics) -> f64 {
        let mut score = 0.0;

        // Direct quotes weight
        let quote_factor = (quote_metrics.direct_quotes as f64).ln() / 5.0;
        score += quote_factor.min(0.4);

        // Citation quality
        score += quote_metrics.citation_quality * 0.3;

        // Discussion generation
        let discussion_factor = (quote_metrics.discussion_threads as f64).ln() / 5.0;
        score += discussion_factor.min(0.3);

        score.min(1.0).max(0.0)
    }

    /// Calculate overall Echo Index score
    pub fn calculate_overall_score(odf: f64, awr: f64, tpm: f64, qf: f64) -> f64 {
        (odf * 0.3) + (awr * 0.25) + (tpm * 0.25) + (qf * 0.2)
    }

    /// Analyze content originality using simple heuristics
    fn analyze_content_originality(content: &str) -> f64 {
        let word_count = content.split_whitespace().count();
        let char_count = content.chars().count();
        
        // Basic originality heuristics
        let length_factor = if word_count > 20 { 0.8 } else { 0.4 };
        let complexity_factor = char_count as f64 / word_count as f64 / 10.0;
        
        (length_factor + complexity_factor.min(0.2)).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odf_calculation() {
        let content = "This is a test content with some originality and depth in the analysis of complex topics.";
        let metrics = EchoMetrics {
            content_length: content.len(),
            word_count: 15,
            unique_words: 14,
            sentiment_score: 0.8,
            readability_score: 0.7,
            originality_markers: vec!["analysis".to_string(), "complex".to_string()],
        };

        let odf = EchoIndexCalculator::calculate_odf(content, &metrics);
        assert!(odf > 0.0 && odf <= 1.0);
    }

    #[test]
    fn test_overall_score_calculation() {
        let score = EchoIndexCalculator::calculate_overall_score(0.8, 0.7, 0.6, 0.5);
        assert!(score > 0.0 && score <= 1.0);
        
        // Check if the calculation is correct with the weights
        let expected = (0.8 * 0.3) + (0.7 * 0.25) + (0.6 * 0.25) + (0.5 * 0.2);
        assert!((score - expected).abs() < 0.001);
    }
} 