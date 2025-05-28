use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct EchoDropReward {
    pub id: String,
    pub user_id: String,
    pub content_id: String,
    pub reward_type: RewardType,
    pub amount: f64,
    pub echo_index_contribution: f64,
    pub timestamp: DateTime<Utc>,
    pub transaction_hash: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RewardType {
    ContentCreation,
    QualityBonus,
    PropagationBonus,
    DiscoveryBonus,
    EngagementReward,
    EchoLoopParticipation,
    CommunityContribution,
}

#[derive(Debug, Clone)]
pub struct RewardMultiplier {
    pub base_rate: f64,
    pub quality_multiplier: f64,
    pub propagation_multiplier: f64,
    pub time_decay_factor: f64,
    pub community_bonus: f64,
}

impl Default for RewardMultiplier {
    fn default() -> Self {
        Self {
            base_rate: 1.0,
            quality_multiplier: 1.5,
            propagation_multiplier: 2.0,
            time_decay_factor: 0.95,
            community_bonus: 1.2,
        }
    }
}

#[derive(Debug)]
pub struct UserRewardStats {
    pub total_earned: f64,
    pub content_rewards: f64,
    pub propagation_rewards: f64,
    pub quality_bonuses: f64,
    pub current_multiplier: f64,
    pub rank: u32,
    pub reward_velocity: f64, // Rewards per hour
}

pub struct RewardsService {
    multipliers: RewardMultiplier,
    pending_rewards: HashMap<String, Vec<EchoDropReward>>,
    processed_rewards: HashMap<String, Vec<EchoDropReward>>,
    user_stats: HashMap<String, UserRewardStats>,
    daily_pool: f64,
    current_pool_remaining: f64,
}

impl RewardsService {
    pub fn new(daily_pool: f64) -> Self {
        Self {
            multipliers: RewardMultiplier::default(),
            pending_rewards: HashMap::new(),
            processed_rewards: HashMap::new(),
            user_stats: HashMap::new(),
            daily_pool,
            current_pool_remaining: daily_pool,
        }
    }

    /// Calculate reward for content creation
    pub fn calculate_content_creation_reward(
        &self,
        echo_index: f64,
        content_quality_score: f64,
        initial_engagement: f64,
    ) -> f64 {
        let base_reward = echo_index * self.multipliers.base_rate;
        let quality_bonus = if content_quality_score > 0.7 {
            base_reward * (self.multipliers.quality_multiplier - 1.0)
        } else {
            0.0
        };
        let engagement_factor = (initial_engagement * 0.1).min(0.5);
        
        (base_reward + quality_bonus) * (1.0 + engagement_factor)
    }

    /// Calculate reward for propagation participation
    pub fn calculate_propagation_reward(
        &self,
        original_echo_index: f64,
        propagation_weight: f64,
        user_influence: f64,
        loop_strength: f64,
    ) -> f64 {
        let base_propagation_reward = original_echo_index * propagation_weight * 0.1;
        let influence_bonus = user_influence * 0.05;
        let loop_bonus = if loop_strength > 0.5 {
            base_propagation_reward * (self.multipliers.propagation_multiplier - 1.0)
        } else {
            0.0
        };

        base_propagation_reward + influence_bonus + loop_bonus
    }

    /// Calculate discovery bonus for organic content discovery
    pub fn calculate_discovery_bonus(
        &self,
        discovered_content_echo_index: f64,
        discovery_timing: f64, // Earlier discovery = higher bonus
        discoverer_influence: f64,
    ) -> f64 {
        let timing_bonus = (1.0 - discovery_timing).max(0.0); // Earlier = higher bonus
        let base_discovery_reward = discovered_content_echo_index * 0.05;
        let influence_factor = (discoverer_influence * 0.1).min(0.3);

        base_discovery_reward * (1.0 + timing_bonus + influence_factor)
    }

    /// Award reward to user
    pub fn award_reward(
        &mut self,
        user_id: String,
        content_id: String,
        reward_type: RewardType,
        amount: f64,
        echo_index_contribution: f64,
    ) -> Result<String, String> {
        if amount > self.current_pool_remaining {
            return Err("Daily reward pool exhausted".to_string());
        }

        let reward_id = format!("reward_{}", uuid::Uuid::new_v4());
        let reward = EchoDropReward {
            id: reward_id.clone(),
            user_id: user_id.clone(),
            content_id,
            reward_type: reward_type.clone(),
            amount,
            echo_index_contribution,
            timestamp: Utc::now(),
            transaction_hash: None,
        };

        // Add to pending rewards
        self.pending_rewards
            .entry(user_id.clone())
            .or_insert_with(Vec::new)
            .push(reward);

        // Update user stats
        self.update_user_stats(&user_id, amount, &reward_type);

        // Reduce pool
        self.current_pool_remaining -= amount;

        Ok(reward_id)
    }

    /// Update user reward statistics
    fn update_user_stats(&mut self, user_id: &str, amount: f64, reward_type: &RewardType) {
        let stats = self.user_stats
            .entry(user_id.to_string())
            .or_insert_with(|| UserRewardStats {
                total_earned: 0.0,
                content_rewards: 0.0,
                propagation_rewards: 0.0,
                quality_bonuses: 0.0,
                current_multiplier: 1.0,
                rank: 0,
                reward_velocity: 0.0,
            });

        stats.total_earned += amount;

        match reward_type {
            RewardType::ContentCreation => stats.content_rewards += amount,
            RewardType::PropagationBonus | RewardType::EchoLoopParticipation => {
                stats.propagation_rewards += amount;
            }
            RewardType::QualityBonus => stats.quality_bonuses += amount,
            _ => {}
        }

        // Calculate reward velocity (rewards per hour over last 24h)
        stats.reward_velocity = self.calculate_reward_velocity(user_id);
        
        // Update multiplier based on recent activity
        stats.current_multiplier = self.calculate_user_multiplier(user_id);
    }

    /// Calculate user's reward velocity
    fn calculate_reward_velocity(&self, user_id: &str) -> f64 {
        let cutoff_time = Utc::now() - chrono::Duration::hours(24);
        
        let recent_rewards: f64 = self.processed_rewards
            .get(user_id)
            .map(|rewards| {
                rewards
                    .iter()
                    .filter(|r| r.timestamp >= cutoff_time)
                    .map(|r| r.amount)
                    .sum()
            })
            .unwrap_or(0.0);

        recent_rewards / 24.0 // Per hour average
    }

    /// Calculate user's current multiplier based on activity
    fn calculate_user_multiplier(&self, user_id: &str) -> f64 {
        let stats = self.user_stats.get(user_id);
        if stats.is_none() {
            return 1.0;
        }

        let stats = stats.unwrap();
        let mut multiplier = self.multipliers.base_rate;

        // Quality content bonus
        if stats.quality_bonuses > stats.total_earned * 0.2 {
            multiplier += 0.2;
        }

        // High propagation activity bonus
        if stats.propagation_rewards > stats.total_earned * 0.3 {
            multiplier += 0.3;
        }

        // Consistency bonus (high velocity)
        if stats.reward_velocity > 0.5 {
            multiplier += 0.1;
        }

        multiplier.min(3.0) // Cap at 3x multiplier
    }

    /// Process pending rewards and prepare for blockchain distribution
    pub fn process_pending_rewards(&mut self, user_id: &str) -> Result<Vec<EchoDropReward>, String> {
        let pending = self.pending_rewards.remove(user_id).unwrap_or_default();
        
        if pending.is_empty() {
            return Ok(Vec::new());
        }

        // Simulate blockchain transaction processing
        let mut processed = Vec::new();
        for mut reward in pending {
            // In real implementation, this would create actual blockchain transaction
            reward.transaction_hash = Some(format!("tx_{}", uuid::Uuid::new_v4()));
            processed.push(reward);
        }

        // Move to processed rewards
        self.processed_rewards
            .entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .extend(processed.clone());

        Ok(processed)
    }

    /// Get user's total accumulated rewards
    pub fn get_user_total_rewards(&self, user_id: &str) -> f64 {
        self.user_stats
            .get(user_id)
            .map(|stats| stats.total_earned)
            .unwrap_or(0.0)
    }

    /// Get user's pending rewards
    pub fn get_pending_rewards(&self, user_id: &str) -> f64 {
        self.pending_rewards
            .get(user_id)
            .map(|rewards| rewards.iter().map(|r| r.amount).sum())
            .unwrap_or(0.0)
    }

    /// Calculate leaderboard rankings
    pub fn calculate_leaderboard(&mut self) -> Vec<(String, UserRewardStats)> {
        let mut users: Vec<_> = self.user_stats
            .iter()
            .map(|(id, stats)| (id.clone(), stats.clone()))
            .collect();

        users.sort_by(|a, b| b.1.total_earned.partial_cmp(&a.1.total_earned).unwrap());

        // Update ranks
        for (rank, (user_id, _)) in users.iter().enumerate() {
            if let Some(stats) = self.user_stats.get_mut(user_id) {
                stats.rank = (rank + 1) as u32;
            }
        }

        users
    }

    /// Reset daily reward pool
    pub fn reset_daily_pool(&mut self) {
        self.current_pool_remaining = self.daily_pool;
    }

    /// Get pool status
    pub fn get_pool_status(&self) -> (f64, f64, f64) {
        (
            self.daily_pool,
            self.current_pool_remaining,
            (self.daily_pool - self.current_pool_remaining) / self.daily_pool
        )
    }

    /// Get reward analytics for time period
    pub fn get_reward_analytics(&self, since: DateTime<Utc>) -> RewardAnalytics {
        let mut total_distributed = 0.0;
        let mut rewards_by_type: HashMap<String, f64> = HashMap::new();
        let mut unique_recipients = std::collections::HashSet::new();

        for rewards in self.processed_rewards.values() {
            for reward in rewards {
                if reward.timestamp >= since {
                    total_distributed += reward.amount;
                    unique_recipients.insert(reward.user_id.clone());
                    
                    let type_key = format!("{:?}", reward.reward_type);
                    *rewards_by_type.entry(type_key).or_insert(0.0) += reward.amount;
                }
            }
        }

        RewardAnalytics {
            total_distributed,
            unique_recipients: unique_recipients.len(),
            rewards_by_type,
            pool_utilization: (self.daily_pool - self.current_pool_remaining) / self.daily_pool,
        }
    }
}

#[derive(Debug)]
pub struct RewardAnalytics {
    pub total_distributed: f64,
    pub unique_recipients: usize,
    pub rewards_by_type: HashMap<String, f64>,
    pub pool_utilization: f64,
} 