use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Content {
    pub id: Uuid,
    pub author_id: Uuid,
    pub text: String,
    pub platform: String,
    pub original_url: String,
    pub echo_index: EchoIndex,
    pub propagation_count: i32,
    pub total_interactions: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EchoIndex {
    pub originality_depth_factor: f64,
    pub audience_weight_rating: f64,
    pub transmission_path_mapping: f64,
    pub quote_frequency: f64,
    pub overall_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContentRequest {
    pub text: String,
    pub platform: String,
    pub original_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentSummary {
    pub id: Uuid,
    pub text: String,
    pub platform: String,
    pub echo_score: f64,
    pub propagation_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Propagation {
    pub id: Uuid,
    pub content_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Option<Uuid>,
    pub platform: String,
    pub propagation_type: String,
    pub depth: i32,
    pub weight: f64,
    pub timestamp: DateTime<Utc>,
}

impl Content {
    pub fn new(author_id: Uuid, text: String, platform: String, original_url: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            author_id,
            text,
            platform,
            original_url,
            echo_index: EchoIndex::default(),
            propagation_count: 0,
            total_interactions: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_echo_index(&mut self, echo_index: EchoIndex) {
        self.echo_index = echo_index;
        self.updated_at = Utc::now();
    }

    pub fn add_propagation(&mut self, weight: f64) {
        self.propagation_count += 1;
        self.echo_index.transmission_path_mapping += weight * 0.1; // Scale factor
        self.calculate_overall_score();
        self.updated_at = Utc::now();
    }

    fn calculate_overall_score(&mut self) {
        let weights = EchoIndexWeights::default();
        self.echo_index.overall_score = 
            self.echo_index.originality_depth_factor * weights.odf +
            self.echo_index.audience_weight_rating * weights.awr +
            self.echo_index.transmission_path_mapping * weights.tpm +
            self.echo_index.quote_frequency * weights.qf;
    }
}

impl Default for EchoIndex {
    fn default() -> Self {
        Self {
            originality_depth_factor: 0.9, // High for new content
            audience_weight_rating: 0.5,   // Default until analyzed
            transmission_path_mapping: 0.0, // No propagation yet
            quote_frequency: 0.0,           // No quotes yet
            overall_score: 0.0,
        }
    }
}

pub struct EchoIndexWeights {
    pub odf: f64,
    pub awr: f64,
    pub tpm: f64,
    pub qf: f64,
}

impl Default for EchoIndexWeights {
    fn default() -> Self {
        Self {
            odf: 0.3,  // 30%
            awr: 0.25, // 25%
            tpm: 0.25, // 25%
            qf: 0.2,   // 20%
        }
    }
} 