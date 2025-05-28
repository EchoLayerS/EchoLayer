use crate::services::rewards::{RewardsService, RewardType, EchoDropReward};
use crate::services::echo_engine::{EchoEngine, EchoMetrics};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub struct RewardService {
    rewards_engine: RewardsService,
    echo_engine: EchoEngine,
    user_engagement_cache: HashMap<String, f64>,
    content_metrics_cache: HashMap<String, EchoMetrics>,
}

impl RewardService {
    pub fn new(daily_pool: f64) -> Self {
        Self {
            rewards_engine: RewardsService::new(daily_pool),
            echo_engine: EchoEngine::default(),
            user_engagement_cache: HashMap::new(),
            content_metrics_cache: HashMap::new(),
        }
    }

    /// Process content creation and award appropriate rewards
    pub async fn process_content_creation(
        &mut self,
        user_id: String,
        content_id: String,
        content_data: ContentCreationData,
    ) -> Result<String, String> {
        // Calculate Echo Index for new content
        let (echo_index, metrics) = self.echo_engine.calculate_complete_echo_index(
            0, // No shares initially
            0, // No total shares initially
            content_data.estimated_reach,
            &HashMap::new(), // No engagement initially
            0.0, // No view time initially
            0, // No views initially
            content_data.creation_timestamp,
            content_data.creation_timestamp,
            0.0, // No interaction frequency initially
            content_data.sentiment_score,
            content_data.credibility_score,
            content_data.relevance_score,
            content_data.originality_score,
        );

        // Cache the metrics
        self.content_metrics_cache.insert(content_id.clone(), metrics);

        // Calculate and award creation reward
        let reward_amount = self.rewards_engine.calculate_content_creation_reward(
            echo_index,
            content_data.quality_score,
            content_data.initial_engagement,
        );

        let reward_id = self.rewards_engine.award_reward(
            user_id,
            content_id,
            RewardType::ContentCreation,
            reward_amount,
            echo_index,
        )?;

        Ok(reward_id)
    }

    /// Process content propagation and award propagation rewards
    pub async fn process_content_propagation(
        &mut self,
        propagator_user_id: String,
        original_content_id: String,
        propagation_data: PropagationData,
    ) -> Result<Vec<String>, String> {
        let mut reward_ids = Vec::new();

        // Get original content metrics
        let original_metrics = self.content_metrics_cache
            .get(&original_content_id)
            .ok_or_else(|| "Original content metrics not found".to_string())?;

        let original_echo_index = self.echo_engine.calculate_echo_index(original_metrics);

        // Get propagator influence
        let propagator_influence = self.user_engagement_cache
            .get(&propagator_user_id)
            .copied()
            .unwrap_or(0.5);

        // Calculate propagation reward
        let propagation_reward = self.rewards_engine.calculate_propagation_reward(
            original_echo_index,
            propagation_data.propagation_weight,
            propagator_influence,
            propagation_data.loop_strength,
        );

        // Award propagation reward to propagator
        let propagator_reward_id = self.rewards_engine.award_reward(
            propagator_user_id,
            original_content_id.clone(),
            RewardType::PropagationBonus,
            propagation_reward,
            original_echo_index * propagation_data.propagation_weight,
        )?;
        reward_ids.push(propagator_reward_id);

        // Award smaller reward to original creator if different user
        if propagation_data.original_creator_id != propagator_user_id {
            let creator_reward = propagation_reward * 0.3; // 30% to original creator
            let creator_reward_id = self.rewards_engine.award_reward(
                propagation_data.original_creator_id,
                original_content_id,
                RewardType::EchoLoopParticipation,
                creator_reward,
                original_echo_index * 0.1,
            )?;
            reward_ids.push(creator_reward_id);
        }

        Ok(reward_ids)
    }

    /// Process content discovery and award discovery bonus
    pub async fn process_content_discovery(
        &mut self,
        discoverer_user_id: String,
        discovered_content_id: String,
        discovery_data: DiscoveryData,
    ) -> Result<String, String> {
        // Get discovered content metrics
        let content_metrics = self.content_metrics_cache
            .get(&discovered_content_id)
            .ok_or_else(|| "Content metrics not found".to_string())?;

        let content_echo_index = self.echo_engine.calculate_echo_index(content_metrics);

        // Get discoverer influence
        let discoverer_influence = self.user_engagement_cache
            .get(&discoverer_user_id)
            .copied()
            .unwrap_or(0.5);

        // Calculate discovery timing bonus (earlier discovery = higher bonus)
        let discovery_timing = discovery_data.discovery_timing_factor;

        // Calculate and award discovery bonus
        let discovery_bonus = self.rewards_engine.calculate_discovery_bonus(
            content_echo_index,
            discovery_timing,
            discoverer_influence,
        );

        let reward_id = self.rewards_engine.award_reward(
            discoverer_user_id,
            discovered_content_id,
            RewardType::DiscoveryBonus,
            discovery_bonus,
            content_echo_index * 0.1,
        )?;

        Ok(reward_id)
    }

    /// Update user engagement metrics
    pub fn update_user_engagement(&mut self, user_id: String, engagement_score: f64) {
        self.user_engagement_cache.insert(user_id, engagement_score);
    }

    /// Update content metrics after new interactions
    pub async fn update_content_metrics(
        &mut self,
        content_id: String,
        updated_data: ContentUpdateData,
    ) -> Result<f64, String> {
        // Recalculate Echo Index with updated data
        let (new_echo_index, new_metrics) = self.echo_engine.calculate_complete_echo_index(
            updated_data.shares_from_discovery,
            updated_data.total_shares,
            updated_data.platform_reach,
            &updated_data.engagement_metrics,
            updated_data.avg_view_time,
            updated_data.total_views,
            updated_data.creation_timestamp,
            updated_data.last_interaction,
            updated_data.interaction_frequency,
            updated_data.sentiment_score,
            updated_data.credibility_score,
            updated_data.relevance_score,
            updated_data.originality_score,
        );

        // Update cache
        self.content_metrics_cache.insert(content_id, new_metrics);

        Ok(new_echo_index)
    }

    /// Process pending rewards for a user
    pub async fn process_user_rewards(&mut self, user_id: &str) -> Result<Vec<EchoDropReward>, String> {
        self.rewards_engine.process_pending_rewards(user_id)
    }

    /// Get user's total rewards
    pub fn get_user_total_rewards(&self, user_id: &str) -> f64 {
        self.rewards_engine.get_user_total_rewards(user_id)
    }

    /// Get user's pending rewards
    pub fn get_user_pending_rewards(&self, user_id: &str) -> f64 {
        self.rewards_engine.get_pending_rewards(user_id)
    }

    /// Get leaderboard
    pub fn get_leaderboard(&mut self) -> Vec<(String, crate::services::rewards::UserRewardStats)> {
        self.rewards_engine.calculate_leaderboard()
    }

    /// Reset daily pool (should be called daily)
    pub fn reset_daily_pool(&mut self) {
        self.rewards_engine.reset_daily_pool();
    }

    /// Get pool status
    pub fn get_pool_status(&self) -> (f64, f64, f64) {
        self.rewards_engine.get_pool_status()
    }

    /// Award quality bonus for high-performing content
    pub async fn award_quality_bonus(
        &mut self,
        user_id: String,
        content_id: String,
        quality_metrics: QualityMetrics,
    ) -> Result<String, String> {
        let bonus_multiplier = if quality_metrics.viral_coefficient > 2.0 {
            2.0
        } else if quality_metrics.engagement_rate > 0.8 {
            1.5
        } else if quality_metrics.retention_rate > 0.7 {
            1.2
        } else {
            1.0
        };

        let base_bonus = quality_metrics.echo_index_improvement * 10.0;
        let bonus_amount = base_bonus * bonus_multiplier;

        let reward_id = self.rewards_engine.award_reward(
            user_id,
            content_id,
            RewardType::QualityBonus,
            bonus_amount,
            quality_metrics.echo_index_improvement,
        )?;

        Ok(reward_id)
    }

    /// Get reward analytics
    pub fn get_reward_analytics(&self, since: DateTime<Utc>) -> crate::services::rewards::RewardAnalytics {
        self.rewards_engine.get_reward_analytics(since)
    }
}

#[derive(Debug)]
pub struct ContentCreationData {
    pub creation_timestamp: i64,
    pub estimated_reach: u32,
    pub sentiment_score: f64,
    pub credibility_score: f64,
    pub relevance_score: f64,
    pub originality_score: f64,
    pub quality_score: f64,
    pub initial_engagement: f64,
}

#[derive(Debug)]
pub struct PropagationData {
    pub original_creator_id: String,
    pub propagation_weight: f64,
    pub loop_strength: f64,
    pub platform_amplification: f64,
}

#[derive(Debug)]
pub struct DiscoveryData {
    pub discovery_timing_factor: f64, // 0.0 = very late, 1.0 = very early
    pub discovery_method: String,
    pub platform: String,
}

#[derive(Debug)]
pub struct ContentUpdateData {
    pub shares_from_discovery: u32,
    pub total_shares: u32,
    pub platform_reach: u32,
    pub engagement_metrics: HashMap<String, f64>,
    pub avg_view_time: f64,
    pub total_views: u32,
    pub creation_timestamp: i64,
    pub last_interaction: i64,
    pub interaction_frequency: f64,
    pub sentiment_score: f64,
    pub credibility_score: f64,
    pub relevance_score: f64,
    pub originality_score: f64,
}

#[derive(Debug)]
pub struct QualityMetrics {
    pub echo_index_improvement: f64,
    pub viral_coefficient: f64,
    pub engagement_rate: f64,
    pub retention_rate: f64,
    pub social_impact_score: f64,
} 