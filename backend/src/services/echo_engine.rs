use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EchoMetrics {
    pub organic_discovery_factor: f64,
    pub attention_weight_ratio: f64,
    pub temporal_persistence_metric: f64,
    pub quality_factor: f64,
}

impl Default for EchoMetrics {
    fn default() -> Self {
        Self {
            organic_discovery_factor: 0.0,
            attention_weight_ratio: 0.0,
            temporal_persistence_metric: 0.0,
            quality_factor: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EchoEngineConfig {
    pub odf_weight: f64,
    pub awr_weight: f64,
    pub tpm_weight: f64,
    pub qf_weight: f64,
    pub decay_factor: f64,
    pub boost_threshold: f64,
}

impl Default for EchoEngineConfig {
    fn default() -> Self {
        Self {
            odf_weight: 0.3,
            awr_weight: 0.25,
            tpm_weight: 0.25,
            qf_weight: 0.2,
            decay_factor: 0.95,
            boost_threshold: 0.8,
        }
    }
}

pub struct EchoEngine {
    config: EchoEngineConfig,
}

impl EchoEngine {
    pub fn new(config: EchoEngineConfig) -> Self {
        Self { config }
    }

    pub fn default() -> Self {
        Self::new(EchoEngineConfig::default())
    }

    /// Calculate the Echo Index for given content
    pub fn calculate_echo_index(&self, metrics: &EchoMetrics) -> f64 {
        let weighted_score = 
            metrics.organic_discovery_factor * self.config.odf_weight +
            metrics.attention_weight_ratio * self.config.awr_weight +
            metrics.temporal_persistence_metric * self.config.tpm_weight +
            metrics.quality_factor * self.config.qf_weight;

        // Apply boost if above threshold
        if weighted_score > self.config.boost_threshold {
            weighted_score * 1.2
        } else {
            weighted_score
        }
    }

    /// Calculate Organic Discovery Factor
    pub fn calculate_odf(&self, 
        shares_from_discovery: u32,
        total_shares: u32,
        platform_reach: u32
    ) -> f64 {
        if total_shares == 0 {
            return 0.0;
        }

        let organic_ratio = shares_from_discovery as f64 / total_shares as f64;
        let reach_factor = (platform_reach as f64).ln() / 10.0; // Logarithmic scaling
        
        (organic_ratio * 0.7 + reach_factor.min(1.0) * 0.3).min(1.0)
    }

    /// Calculate Attention Weight Ratio
    pub fn calculate_awr(&self,
        engagement_metrics: &HashMap<String, f64>,
        view_time: f64,
        total_views: u32
    ) -> f64 {
        let engagement_score: f64 = engagement_metrics.values().sum();
        let time_factor = (view_time / 60.0).min(1.0); // Normalize to minutes
        let popularity_factor = (total_views as f64).ln() / 15.0; // Logarithmic scaling

        (engagement_score * 0.5 + time_factor * 0.3 + popularity_factor.min(1.0) * 0.2).min(1.0)
    }

    /// Calculate Temporal Persistence Metric
    pub fn calculate_tpm(&self,
        creation_time: i64,
        last_interaction: i64,
        interaction_frequency: f64
    ) -> f64 {
        let current_time = chrono::Utc::now().timestamp();
        let content_age = (current_time - creation_time) as f64 / 86400.0; // Age in days
        let recency = (current_time - last_interaction) as f64 / 86400.0; // Recency in days

        let age_factor = (1.0 / (1.0 + content_age * 0.1)).max(0.1);
        let recency_factor = (1.0 / (1.0 + recency * 0.2)).max(0.1);
        let frequency_factor = (interaction_frequency / 10.0).min(1.0);

        (age_factor * 0.3 + recency_factor * 0.4 + frequency_factor * 0.3).min(1.0)
    }

    /// Calculate Quality Factor
    pub fn calculate_qf(&self,
        sentiment_score: f64,
        credibility_score: f64,
        relevance_score: f64,
        originality_score: f64
    ) -> f64 {
        // Normalize all scores to 0-1 range
        let normalized_sentiment = (sentiment_score + 1.0) / 2.0; // From [-1,1] to [0,1]
        let normalized_credibility = credibility_score.max(0.0).min(1.0);
        let normalized_relevance = relevance_score.max(0.0).min(1.0);
        let normalized_originality = originality_score.max(0.0).min(1.0);

        (normalized_sentiment * 0.2 + 
         normalized_credibility * 0.3 + 
         normalized_relevance * 0.3 + 
         normalized_originality * 0.2).min(1.0)
    }

    /// Apply temporal decay to existing Echo Index
    pub fn apply_temporal_decay(&self, current_index: f64, hours_elapsed: f64) -> f64 {
        let decay_rate = self.config.decay_factor.powf(hours_elapsed / 24.0);
        current_index * decay_rate
    }

    /// Calculate complete Echo Index with all components
    pub fn calculate_complete_echo_index(&self,
        shares_from_discovery: u32,
        total_shares: u32,
        platform_reach: u32,
        engagement_metrics: &HashMap<String, f64>,
        view_time: f64,
        total_views: u32,
        creation_time: i64,
        last_interaction: i64,
        interaction_frequency: f64,
        sentiment_score: f64,
        credibility_score: f64,
        relevance_score: f64,
        originality_score: f64
    ) -> (f64, EchoMetrics) {
        let odf = self.calculate_odf(shares_from_discovery, total_shares, platform_reach);
        let awr = self.calculate_awr(engagement_metrics, view_time, total_views);
        let tpm = self.calculate_tpm(creation_time, last_interaction, interaction_frequency);
        let qf = self.calculate_qf(sentiment_score, credibility_score, relevance_score, originality_score);

        let metrics = EchoMetrics {
            organic_discovery_factor: odf,
            attention_weight_ratio: awr,
            temporal_persistence_metric: tpm,
            quality_factor: qf,
        };

        let echo_index = self.calculate_echo_index(&metrics);
        (echo_index, metrics)
    }
} 