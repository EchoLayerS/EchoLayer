use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Echo Index Integration Tests
/// Tests the core Echo Index calculation functionality

#[derive(Debug, Clone)]
pub struct TestContent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub platform: String,
    pub external_id: String,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub engagement_metrics: HashMap<String, i64>,
    pub quality_indicators: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct TestPropagation {
    pub id: Uuid,
    pub content_id: Uuid,
    pub source_platform: String,
    pub target_platform: String,
    pub propagation_type: String,
    pub created_at: DateTime<Utc>,
    pub reach: i64,
    pub engagement: i64,
}

#[derive(Debug, Clone)]
pub struct EchoIndexCalculation {
    pub odf_score: f64,
    pub awr_score: f64,
    pub tpm_score: f64,
    pub qf_score: f64,
    pub final_score: f64,
}

/// Echo Index Calculator
pub struct EchoIndexCalculator {
    pub odf_weight: f64,
    pub awr_weight: f64,
    pub tpm_weight: f64,
    pub qf_weight: f64,
}

impl Default for EchoIndexCalculator {
    fn default() -> Self {
        Self {
            odf_weight: 0.30,
            awr_weight: 0.25,
            tpm_weight: 0.25,
            qf_weight: 0.20,
        }
    }
}

impl EchoIndexCalculator {
    /// Calculate Organic Discovery Factor (ODF)
    /// Measures how naturally content is discovered without paid promotion
    pub fn calculate_odf(&self, content: &TestContent, propagations: &[TestPropagation]) -> f64 {
        if propagations.is_empty() {
            return 0.0;
        }

        let organic_propagations = propagations.iter()
            .filter(|p| p.propagation_type != "paid_promotion")
            .count() as f64;
        
        let total_propagations = propagations.len() as f64;
        let organic_ratio = organic_propagations / total_propagations;

        // Platform diversity bonus
        let unique_platforms: std::collections::HashSet<_> = propagations.iter()
            .map(|p| &p.target_platform)
            .collect();
        let platform_diversity = (unique_platforms.len() as f64 / 4.0).min(1.0); // Max 4 platforms

        // Time-based organic discovery
        let now = Utc::now();
        let content_age_hours = (now - content.created_at).num_hours() as f64;
        let discovery_velocity = if content_age_hours > 0.0 {
            (organic_propagations / content_age_hours).min(10.0) / 10.0
        } else {
            0.0
        };

        let base_score = organic_ratio * 40.0;
        let diversity_bonus = platform_diversity * 30.0;
        let velocity_bonus = discovery_velocity * 30.0;

        (base_score + diversity_bonus + velocity_bonus).min(100.0)
    }

    /// Calculate Attention Weighted Reach (AWR)
    /// Measures the quality-adjusted reach of content
    pub fn calculate_awr(&self, content: &TestContent, propagations: &[TestPropagation]) -> f64 {
        let total_reach: i64 = propagations.iter()
            .map(|p| p.reach)
            .sum();

        let total_engagement: i64 = propagations.iter()
            .map(|p| p.engagement)
            .sum();

        if total_reach == 0 {
            return 0.0;
        }

        // Engagement rate
        let engagement_rate = (total_engagement as f64) / (total_reach as f64);
        let engagement_score = (engagement_rate * 1000.0).min(100.0); // Scale engagement rate

        // Cross-platform reach bonus
        let platform_reach: HashMap<String, i64> = propagations.iter()
            .fold(HashMap::new(), |mut acc, p| {
                *acc.entry(p.target_platform.clone()).or_insert(0) += p.reach;
                acc
            });

        let cross_platform_bonus = if platform_reach.len() > 1 {
            let max_reach = *platform_reach.values().max().unwrap_or(&0) as f64;
            let total_cross_reach: i64 = platform_reach.values()
                .filter(|&&reach| reach < max_reach as i64)
                .sum();
            
            (total_cross_reach as f64 / max_reach).min(1.0) * 20.0
        } else {
            0.0
        };

        // Reach scale factor (logarithmic scaling for very high reach)
        let reach_scale = if total_reach > 10000 {
            80.0 + (total_reach as f64).log10() * 5.0
        } else {
            (total_reach as f64 / 10000.0) * 80.0
        };

        (engagement_score * 0.6 + cross_platform_bonus + reach_scale * 0.4).min(100.0)
    }

    /// Calculate Time-based Propagation Metric (TPM)
    /// Measures the speed and sustainability of content propagation
    pub fn calculate_tpm(&self, content: &TestContent, propagations: &[TestPropagation]) -> f64 {
        if propagations.is_empty() {
            return 0.0;
        }

        let now = Utc::now();
        let content_age_hours = (now - content.created_at).num_hours() as f64;
        
        if content_age_hours <= 0.0 {
            return 100.0; // New content gets max initial score
        }

        // Calculate propagation velocity over time
        let propagations_per_hour = propagations.len() as f64 / content_age_hours;
        let velocity_score = (propagations_per_hour * 10.0).min(50.0);

        // Calculate propagation decay/growth pattern
        let mut time_buckets = vec![0; 24]; // 24 hour buckets
        for prop in propagations {
            let hours_since_content = (prop.created_at - content.created_at).num_hours();
            if hours_since_content >= 0 && hours_since_content < 24 {
                time_buckets[hours_since_content as usize] += 1;
            }
        }

        // Analyze propagation pattern
        let peak_hour = time_buckets.iter().position(|&x| x == *time_buckets.iter().max().unwrap()).unwrap_or(0);
        let sustainability_score = if peak_hour > 0 {
            let before_peak: i32 = time_buckets[..peak_hour].iter().sum();
            let after_peak: i32 = time_buckets[peak_hour + 1..].iter().sum();
            
            if before_peak > 0 {
                (after_peak as f64 / before_peak as f64).min(1.0) * 30.0
            } else {
                15.0
            }
        } else {
            20.0 // Early content
        };

        // Recent activity bonus
        let recent_propagations = propagations.iter()
            .filter(|p| (now - p.created_at).num_hours() < 6)
            .count() as f64;
        let recent_activity_score = (recent_propagations / 5.0).min(1.0) * 20.0;

        (velocity_score + sustainability_score + recent_activity_score).min(100.0)
    }

    /// Calculate Quality Factor (QF)
    /// Measures content quality based on various indicators
    pub fn calculate_qf(&self, content: &TestContent, _propagations: &[TestPropagation]) -> f64 {
        let mut quality_score = 0.0;

        // Content length and structure
        let word_count = content.body.split_whitespace().count();
        let length_score = match word_count {
            0..=10 => 10.0,
            11..=50 => 25.0,
            51..=200 => 40.0,
            201..=500 => 35.0,
            _ => 20.0,
        };

        // Engagement quality metrics
        let likes = content.engagement_metrics.get("likes").unwrap_or(&0);
        let comments = content.engagement_metrics.get("comments").unwrap_or(&0);
        let shares = content.engagement_metrics.get("shares").unwrap_or(&0);

        // Comments indicate higher engagement quality
        let engagement_quality = if likes + comments + shares > 0 {
            let comment_ratio = (*comments as f64) / ((likes + comments + shares) as f64);
            comment_ratio * 30.0
        } else {
            0.0
        };

        // Content quality indicators
        let originality = content.quality_indicators.get("originality").unwrap_or(&0.5);
        let readability = content.quality_indicators.get("readability").unwrap_or(&0.5);
        let informativeness = content.quality_indicators.get("informativeness").unwrap_or(&0.5);

        let content_quality = (originality + readability + informativeness) / 3.0 * 30.0;

        quality_score = length_score + engagement_quality + content_quality;
        quality_score.min(100.0)
    }

    /// Calculate final Echo Index score
    pub fn calculate_echo_index(&self, content: &TestContent, propagations: &[TestPropagation]) -> EchoIndexCalculation {
        let odf_score = self.calculate_odf(content, propagations);
        let awr_score = self.calculate_awr(content, propagations);
        let tpm_score = self.calculate_tpm(content, propagations);
        let qf_score = self.calculate_qf(content, propagations);

        let final_score = (odf_score * self.odf_weight) +
                         (awr_score * self.awr_weight) +
                         (tpm_score * self.tpm_weight) +
                         (qf_score * self.qf_weight);

        EchoIndexCalculation {
            odf_score,
            awr_score,
            tpm_score,
            qf_score,
            final_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn create_test_content() -> TestContent {
        let mut engagement_metrics = HashMap::new();
        engagement_metrics.insert("likes".to_string(), 100);
        engagement_metrics.insert("comments".to_string(), 25);
        engagement_metrics.insert("shares".to_string(), 50);

        let mut quality_indicators = HashMap::new();
        quality_indicators.insert("originality".to_string(), 0.8);
        quality_indicators.insert("readability".to_string(), 0.7);
        quality_indicators.insert("informativeness".to_string(), 0.9);

        TestContent {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            platform: "twitter".to_string(),
            external_id: "tweet_123".to_string(),
            title: "Amazing blockchain innovation".to_string(),
            body: "This is a comprehensive analysis of the latest blockchain technology that enables decentralized attention tracking across multiple social media platforms. The innovation promises to revolutionize how we measure and reward content quality.".to_string(),
            created_at: Utc::now() - Duration::hours(6),
            engagement_metrics,
            quality_indicators,
        }
    }

    fn create_test_propagations(content_id: Uuid) -> Vec<TestPropagation> {
        vec![
            TestPropagation {
                id: Uuid::new_v4(),
                content_id,
                source_platform: "twitter".to_string(),
                target_platform: "telegram".to_string(),
                propagation_type: "share".to_string(),
                created_at: Utc::now() - Duration::hours(5),
                reach: 1500,
                engagement: 180,
            },
            TestPropagation {
                id: Uuid::new_v4(),
                content_id,
                source_platform: "telegram".to_string(),
                target_platform: "linkedin".to_string(),
                propagation_type: "cross_post".to_string(),
                created_at: Utc::now() - Duration::hours(3),
                reach: 800,
                engagement: 120,
            },
            TestPropagation {
                id: Uuid::new_v4(),
                content_id,
                source_platform: "linkedin".to_string(),
                target_platform: "reddit".to_string(),
                propagation_type: "share".to_string(),
                created_at: Utc::now() - Duration::hours(1),
                reach: 600,
                engagement: 90,
            },
        ]
    }

    #[test]
    fn test_echo_index_calculation_basic() {
        let calculator = EchoIndexCalculator::default();
        let content = create_test_content();
        let propagations = create_test_propagations(content.id);

        let result = calculator.calculate_echo_index(&content, &propagations);

        // Verify all components are calculated
        assert!(result.odf_score > 0.0);
        assert!(result.awr_score > 0.0);
        assert!(result.tpm_score > 0.0);
        assert!(result.qf_score > 0.0);
        assert!(result.final_score > 0.0);

        // Verify final score is within expected range
        assert!(result.final_score <= 100.0);
        assert!(result.final_score >= 0.0);

        println!("Echo Index Calculation Results:");
        println!("ODF Score: {:.2}", result.odf_score);
        println!("AWR Score: {:.2}", result.awr_score);
        println!("TPM Score: {:.2}", result.tpm_score);
        println!("QF Score: {:.2}", result.qf_score);
        println!("Final Score: {:.2}", result.final_score);
    }

    #[test]
    fn test_odf_calculation() {
        let calculator = EchoIndexCalculator::default();
        let content = create_test_content();
        let propagations = create_test_propagations(content.id);

        let odf_score = calculator.calculate_odf(&content, &propagations);

        assert!(odf_score > 0.0);
        assert!(odf_score <= 100.0);

        // Test with no propagations
        let empty_propagations = vec![];
        let odf_empty = calculator.calculate_odf(&content, &empty_propagations);
        assert_eq!(odf_empty, 0.0);
    }

    #[test]
    fn test_awr_calculation() {
        let calculator = EchoIndexCalculator::default();
        let content = create_test_content();
        let propagations = create_test_propagations(content.id);

        let awr_score = calculator.calculate_awr(&content, &propagations);

        assert!(awr_score > 0.0);
        assert!(awr_score <= 100.0);

        // Test with no propagations
        let empty_propagations = vec![];
        let awr_empty = calculator.calculate_awr(&content, &empty_propagations);
        assert_eq!(awr_empty, 0.0);
    }

    #[test]
    fn test_tpm_calculation() {
        let calculator = EchoIndexCalculator::default();
        let content = create_test_content();
        let propagations = create_test_propagations(content.id);

        let tpm_score = calculator.calculate_tpm(&content, &propagations);

        assert!(tpm_score > 0.0);
        assert!(tpm_score <= 100.0);

        // Test with no propagations
        let empty_propagations = vec![];
        let tpm_empty = calculator.calculate_tpm(&content, &empty_propagations);
        assert_eq!(tpm_empty, 0.0);
    }

    #[test]
    fn test_qf_calculation() {
        let calculator = EchoIndexCalculator::default();
        let content = create_test_content();
        let propagations = create_test_propagations(content.id);

        let qf_score = calculator.calculate_qf(&content, &propagations);

        assert!(qf_score > 0.0);
        assert!(qf_score <= 100.0);
    }

    #[test]
    fn test_echo_index_weights() {
        let calculator = EchoIndexCalculator::default();
        
        // Verify weights sum to 1.0
        let total_weight = calculator.odf_weight + calculator.awr_weight + 
                          calculator.tpm_weight + calculator.qf_weight;
        assert!((total_weight - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_high_quality_content() {
        let calculator = EchoIndexCalculator::default();
        
        // Create high-quality content
        let mut content = create_test_content();
        content.quality_indicators.insert("originality".to_string(), 0.95);
        content.quality_indicators.insert("readability".to_string(), 0.90);
        content.quality_indicators.insert("informativeness".to_string(), 0.95);
        
        // Add more engagement
        content.engagement_metrics.insert("likes".to_string(), 500);
        content.engagement_metrics.insert("comments".to_string(), 150);
        content.engagement_metrics.insert("shares".to_string(), 200);
        
        let propagations = create_test_propagations(content.id);
        let result = calculator.calculate_echo_index(&content, &propagations);

        // High-quality content should have good QF score
        assert!(result.qf_score > 50.0);
        assert!(result.final_score > 40.0);
    }

    #[test]
    fn test_viral_content() {
        let calculator = EchoIndexCalculator::default();
        let content = create_test_content();
        
        // Create viral propagation pattern
        let mut viral_propagations = vec![];
        for i in 0..10 {
            viral_propagations.push(TestPropagation {
                id: Uuid::new_v4(),
                content_id: content.id,
                source_platform: "twitter".to_string(),
                target_platform: "telegram".to_string(),
                propagation_type: "share".to_string(),
                created_at: Utc::now() - Duration::hours(i),
                reach: 5000,
                engagement: 800,
            });
        }

        let result = calculator.calculate_echo_index(&content, &viral_propagations);

        // Viral content should have high AWR and TPM scores
        assert!(result.awr_score > 70.0);
        assert!(result.tpm_score > 60.0);
        assert!(result.final_score > 50.0);
    }

    #[test]
    fn test_cross_platform_propagation() {
        let calculator = EchoIndexCalculator::default();
        let content = create_test_content();
        
        // Create diverse cross-platform propagations
        let cross_platform_propagations = vec![
            TestPropagation {
                id: Uuid::new_v4(),
                content_id: content.id,
                source_platform: "twitter".to_string(),
                target_platform: "telegram".to_string(),
                propagation_type: "share".to_string(),
                created_at: Utc::now() - Duration::hours(4),
                reach: 2000,
                engagement: 300,
            },
            TestPropagation {
                id: Uuid::new_v4(),
                content_id: content.id,
                source_platform: "telegram".to_string(),
                target_platform: "linkedin".to_string(),
                propagation_type: "cross_post".to_string(),
                created_at: Utc::now() - Duration::hours(3),
                reach: 1500,
                engagement: 250,
            },
            TestPropagation {
                id: Uuid::new_v4(),
                content_id: content.id,
                source_platform: "linkedin".to_string(),
                target_platform: "reddit".to_string(),
                propagation_type: "share".to_string(),
                created_at: Utc::now() - Duration::hours(2),
                reach: 1000,
                engagement: 180,
            },
            TestPropagation {
                id: Uuid::new_v4(),
                content_id: content.id,
                source_platform: "linkedin".to_string(),
                target_platform: "reddit".to_string(),
                propagation_type: "link".to_string(),
                created_at: Utc::now() - Duration::hours(1),
                reach: 800,
                engagement: 120,
            },
        ];

        let result = calculator.calculate_echo_index(&content, &cross_platform_propagations);

        // Cross-platform content should have high ODF score
        assert!(result.odf_score > 60.0);
        assert!(result.final_score > 45.0);
    }

    #[test]
    fn test_echo_index_consistency() {
        let calculator = EchoIndexCalculator::default();
        let content = create_test_content();
        let propagations = create_test_propagations(content.id);

        // Calculate multiple times to ensure consistency
        let result1 = calculator.calculate_echo_index(&content, &propagations);
        let result2 = calculator.calculate_echo_index(&content, &propagations);

        assert_eq!(result1.final_score, result2.final_score);
        assert_eq!(result1.odf_score, result2.odf_score);
        assert_eq!(result1.awr_score, result2.awr_score);
        assert_eq!(result1.tpm_score, result2.tpm_score);
        assert_eq!(result1.qf_score, result2.qf_score);
    }
} 