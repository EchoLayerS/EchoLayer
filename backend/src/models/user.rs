use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub wallet_address: Option<String>,
    pub echo_score: f64,
    pub total_content_created: i64,
    pub total_rewards_earned: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub social_accounts: Vec<SocialAccount>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SocialAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub platform: String,
    pub account_id: String,
    pub username: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub user: User,
    pub social_accounts: Vec<SocialAccount>,
    pub recent_content: Vec<crate::models::content::ContentSummary>,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            wallet_address: None,
            echo_score: 0.0,
            total_content_created: 0,
            total_rewards_earned: 0.0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_echo_score(&mut self, new_score: f64) {
        self.echo_score = new_score;
        self.updated_at = Utc::now();
    }

    pub fn add_content(&mut self) {
        self.total_content_created += 1;
        self.updated_at = Utc::now();
    }

    pub fn add_rewards(&mut self, amount: f64) {
        self.total_rewards_earned += amount;
        self.updated_at = Utc::now();
    }
} 