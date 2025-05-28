use actix_web::{get, post, put, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub wallet_address: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub wallet_address: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub echo_score: f64,
    pub total_rewards: f64,
    pub rank: Option<u32>,
    pub is_verified: bool,
    pub created_at: String,
}

/// Create a new user
#[post("")]
pub async fn create_user(user_data: web::Json<CreateUserRequest>) -> Result<HttpResponse> {
    // Mock implementation - in real app, this would save to database
    let user = UserResponse {
        id: Uuid::new_v4().to_string(),
        wallet_address: user_data.wallet_address.clone(),
        username: user_data.username.clone(),
        display_name: user_data.display_name.clone(),
        echo_score: 0.0,
        total_rewards: 0.0,
        rank: None,
        is_verified: false,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(HttpResponse::Created().json(json!({
        "success": true,
        "data": user,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Get user by ID
#[get("/{user_id}")]
pub async fn get_user(path: web::Path<String>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    
    // Mock user data
    let user = UserResponse {
        id: user_id,
        wallet_address: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
        username: Some("echo_pioneer".to_string()),
        display_name: Some("Echo Pioneer".to_string()),
        echo_score: 75.8,
        total_rewards: 1250.0,
        rank: Some(42),
        is_verified: true,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": user,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Update user information
#[put("/{user_id}")]
pub async fn update_user(
    path: web::Path<String>,
    user_data: web::Json<CreateUserRequest>
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    
    // Mock update response
    let user = UserResponse {
        id: user_id,
        wallet_address: user_data.wallet_address.clone(),
        username: user_data.username.clone(),
        display_name: user_data.display_name.clone(),
        echo_score: 75.8,
        total_rewards: 1250.0,
        rank: Some(42),
        is_verified: true,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": user,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Get user analytics
#[get("/{user_id}/analytics")]
pub async fn get_user_analytics(path: web::Path<String>) -> Result<HttpResponse> {
    let _user_id = path.into_inner();
    
    // Mock analytics data
    let analytics = json!({
        "summary": {
            "total_echo_score": 75.8,
            "content_count": 23,
            "propagation_count": 156,
            "rewards_earned": 1250.0,
            "rank_change": 5
        },
        "echo_score_trend": [
            {"date": "2024-01-01", "score": 65.2},
            {"date": "2024-01-02", "score": 68.1},
            {"date": "2024-01-03", "score": 71.5},
            {"date": "2024-01-04", "score": 75.8}
        ],
        "platform_breakdown": {
            "twitter": {
                "content_count": 15,
                "avg_echo_index": 72.3,
                "total_rewards": 850.0
            },
            "telegram": {
                "content_count": 8,
                "avg_echo_index": 81.2,
                "total_rewards": 400.0
            }
        }
    });

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": analytics,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Get leaderboard
#[get("/leaderboard")]
pub async fn get_leaderboard() -> Result<HttpResponse> {
    // Mock leaderboard data
    let leaderboard = vec![
        json!({
            "rank": 1,
            "user_id": "user_1",
            "username": "echo_master",
            "display_name": "Echo Master",
            "echo_score": 95.8,
            "total_rewards": 5000.0
        }),
        json!({
            "rank": 2,
            "user_id": "user_2",
            "username": "propagation_pro",
            "display_name": "Propagation Pro",
            "echo_score": 89.3,
            "total_rewards": 4200.0
        }),
        json!({
            "rank": 3,
            "user_id": "user_3",
            "username": "content_king",
            "display_name": "Content King",
            "echo_score": 85.7,
            "total_rewards": 3800.0
        })
    ];

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": leaderboard,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
} 