use actix_web::{get, post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreatePropagationRequest {
    pub content_id: String,
    pub source_user_id: Option<String>,
    pub target_user_id: Option<String>,
    pub propagation_type: String,
    pub source_platform: String,
    pub target_platform: String,
    pub source_external_id: Option<String>,
    pub target_external_id: Option<String>,
}

#[derive(Serialize)]
pub struct PropagationResponse {
    pub id: String,
    pub content_id: String,
    pub source_user_id: Option<String>,
    pub target_user_id: Option<String>,
    pub propagation_type: String,
    pub source_platform: String,
    pub target_platform: String,
    pub echo_boost: f64,
    pub reward_amount: f64,
    pub engagement_metrics: EngagementMetrics,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct EngagementMetrics {
    pub views: u32,
    pub likes: u32,
    pub comments: u32,
    pub shares: u32,
    pub reaches: u32,
    pub clicks: u32,
    pub saves: u32,
}

#[derive(Serialize)]
pub struct PropagationNode {
    pub id: String,
    pub user_id: String,
    pub platform: String,
    pub influence: f64,
    pub echo_score: f64,
}

#[derive(Serialize)]
pub struct PropagationEdge {
    pub source_id: String,
    pub target_id: String,
    pub weight: f64,
    pub propagation_type: String,
    pub timestamp: String,
}

#[derive(Serialize)]
pub struct PropagationNetwork {
    pub nodes: Vec<PropagationNode>,
    pub edges: Vec<PropagationEdge>,
    pub metrics: NetworkMetrics,
}

#[derive(Serialize)]
pub struct NetworkMetrics {
    pub total_nodes: u32,
    pub total_edges: u32,
    pub density: f64,
    pub average_path_length: f64,
    pub clustering_coefficient: f64,
}

/// Create a new propagation record
#[post("")]
pub async fn create_propagation(
    propagation_data: web::Json<CreatePropagationRequest>
) -> Result<HttpResponse> {
    let propagation = PropagationResponse {
        id: Uuid::new_v4().to_string(),
        content_id: propagation_data.content_id.clone(),
        source_user_id: propagation_data.source_user_id.clone(),
        target_user_id: propagation_data.target_user_id.clone(),
        propagation_type: propagation_data.propagation_type.clone(),
        source_platform: propagation_data.source_platform.clone(),
        target_platform: propagation_data.target_platform.clone(),
        echo_boost: 1.25, // Calculated based on propagation quality
        reward_amount: 5.0, // Token reward for successful propagation
        engagement_metrics: EngagementMetrics {
            views: 150,
            likes: 12,
            comments: 3,
            shares: 8,
            reaches: 120,
            clicks: 25,
            saves: 4,
        },
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(HttpResponse::Created().json(json!({
        "success": true,
        "data": propagation,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Get propagation network for content
#[get("/{content_id}/network")]
pub async fn get_propagation_network(path: web::Path<String>) -> Result<HttpResponse> {
    let _content_id = path.into_inner();
    
    // Mock propagation network data
    let network = PropagationNetwork {
        nodes: vec![
            PropagationNode {
                id: "node_1".to_string(),
                user_id: "user_1".to_string(),
                platform: "twitter".to_string(),
                influence: 85.5,
                echo_score: 92.3,
            },
            PropagationNode {
                id: "node_2".to_string(),
                user_id: "user_2".to_string(),
                platform: "telegram".to_string(),
                influence: 72.8,
                echo_score: 78.1,
            },
            PropagationNode {
                id: "node_3".to_string(),
                user_id: "user_3".to_string(),
                platform: "linkedin".to_string(),
                influence: 68.2,
                echo_score: 74.5,
            },
        ],
        edges: vec![
            PropagationEdge {
                source_id: "node_1".to_string(),
                target_id: "node_2".to_string(),
                weight: 0.8,
                propagation_type: "share".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            PropagationEdge {
                source_id: "node_2".to_string(),
                target_id: "node_3".to_string(),
                weight: 0.6,
                propagation_type: "repost".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        ],
        metrics: NetworkMetrics {
            total_nodes: 3,
            total_edges: 2,
            density: 0.33,
            average_path_length: 1.5,
            clustering_coefficient: 0.0,
        },
    };

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": network,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Get propagation analytics
#[get("/{content_id}/analytics")]
pub async fn get_propagation_analytics(path: web::Path<String>) -> Result<HttpResponse> {
    let _content_id = path.into_inner();
    
    // Mock propagation analytics
    let analytics = json!({
        "overview": {
            "total_propagations": 42,
            "unique_platforms": 4,
            "total_reach": 15420,
            "total_engagement": 1850,
            "propagation_velocity": 2.5,
            "virality_coefficient": 1.8
        },
        "timeline": [
            {
                "timestamp": "2024-01-01T00:00:00Z",
                "propagations": 1,
                "cumulative_reach": 250
            },
            {
                "timestamp": "2024-01-01T06:00:00Z",
                "propagations": 5,
                "cumulative_reach": 1200
            },
            {
                "timestamp": "2024-01-01T12:00:00Z",
                "propagations": 12,
                "cumulative_reach": 4500
            },
            {
                "timestamp": "2024-01-01T18:00:00Z",
                "propagations": 24,
                "cumulative_reach": 8900
            }
        ],
        "platform_breakdown": {
            "twitter": {
                "propagations": 18,
                "reach": 6800,
                "engagement_rate": 0.12
            },
            "telegram": {
                "propagations": 12,
                "reach": 4200,
                "engagement_rate": 0.15
            },
            "linkedin": {
                "propagations": 8,
                "reach": 2800,
                "engagement_rate": 0.22
            }
        },
        "top_propagators": [
            {
                "user_id": "user_1",
                "username": "crypto_influencer",
                "propagations": 8,
                "total_reach": 3200,
                "echo_boost": 2.4
            },
            {
                "user_id": "user_2",
                "username": "tech_enthusiast",
                "propagations": 6,
                "total_reach": 2400,
                "echo_boost": 1.8
            }
        ]
    });

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": analytics,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
} 