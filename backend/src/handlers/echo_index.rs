use actix_web::{web, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Echo Index calculation request payload
#[derive(Deserialize)]
pub struct EchoIndexRequest {
    pub content_id: String,
    pub content_type: String,
    pub content_text: String,
    pub author_id: String,
    pub platform: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Echo Index calculation response
#[derive(Serialize)]
pub struct EchoIndexResponse {
    pub content_id: String,
    pub echo_index: EchoIndex,
    pub calculated_at: DateTime<Utc>,
    pub version: String,
}

/// Echo Index components and overall score
#[derive(Serialize, Deserialize, Clone)]
pub struct EchoIndex {
    pub odf: f64,  // Originality Depth Factor
    pub awr: f64,  // Audience Weight Rating
    pub tpm: f64,  // Transmission Path Mapping
    pub qf: f64,   // Quote Frequency
    pub score: f64, // Overall Echo Index score (0-100)
    pub tier: String, // Gold, Silver, Bronze, Basic
}

/// Propagation data for Echo Index calculation
#[derive(Deserialize)]
pub struct PropagationData {
    pub shares: u32,
    pub likes: u32,
    pub comments: u32,
    pub quotes: u32,
    pub reach: u32,
    pub engagement_rate: f64,
    pub audience_quality: f64,
    pub transmission_paths: Vec<TransmissionPath>,
}

/// Individual transmission path
#[derive(Deserialize, Serialize)]
pub struct TransmissionPath {
    pub from_user: String,
    pub to_user: String,
    pub platform: String,
    pub timestamp: DateTime<Utc>,
    pub interaction_type: String,
    pub weight: f64,
}

/// Leaderboard entry
#[derive(Serialize)]
pub struct LeaderboardEntry {
    pub rank: u32,
    pub content_id: String,
    pub title: String,
    pub author: String,
    pub echo_index: f64,
    pub tier: String,
    pub created_at: DateTime<Utc>,
}

/// Echo Index calculation service
impl EchoIndex {
    /// Calculate Echo Index based on content and propagation data
    pub fn calculate(
        content: &EchoIndexRequest,
        propagation: &PropagationData,
    ) -> Self {
        let odf = Self::calculate_odf(content, propagation);
        let awr = Self::calculate_awr(propagation);
        let tpm = Self::calculate_tpm(&propagation.transmission_paths);
        let qf = Self::calculate_qf(propagation);
        
        // Weighted combination of all factors
        let score = (odf * 0.3) + (awr * 0.25) + (tpm * 0.25) + (qf * 0.2);
        let tier = Self::determine_tier(score);
        
        EchoIndex {
            odf,
            awr,
            tpm,
            qf,
            score,
            tier,
        }
    }
    
    /// Calculate Originality Depth Factor (ODF)
    /// Measures content uniqueness and depth
    fn calculate_odf(content: &EchoIndexRequest, propagation: &PropagationData) -> f64 {
        // Content length factor (longer content generally more original)
        let length_factor = (content.content_text.len() as f64 / 280.0).min(2.0);
        
        // Uniqueness factor based on quotes/shares ratio
        let uniqueness_factor = if propagation.shares > 0 {
            1.0 - (propagation.quotes as f64 / propagation.shares as f64).min(1.0)
        } else {
            1.0
        };
        
        // Engagement depth (comments vs simple likes)
        let engagement_depth = if propagation.likes > 0 {
            (propagation.comments as f64 / propagation.likes as f64).min(1.0)
        } else {
            0.0
        };
        
        // Platform factor (some platforms encourage more original content)
        let platform_factor = match content.platform.as_str() {
            "twitter" => 0.8,
            "linkedin" => 1.2,
            "medium" => 1.5,
            _ => 1.0,
        };
        
        let odf = (length_factor + uniqueness_factor + engagement_depth) 
                 * platform_factor * 33.33; // Scale to 0-100
        
        odf.min(100.0).max(0.0)
    }
    
    /// Calculate Audience Weight Rating (AWR)
    /// Measures audience quality and influence
    fn calculate_awr(propagation: &PropagationData) -> f64 {
        // Base audience quality score
        let quality_score = propagation.audience_quality * 50.0;
        
        // Engagement rate factor
        let engagement_factor = propagation.engagement_rate * 30.0;
        
        // Reach factor (logarithmic scale to prevent infinite growth)
        let reach_factor = (propagation.reach as f64).log10() * 5.0;
        
        let awr = quality_score + engagement_factor + reach_factor;
        awr.min(100.0).max(0.0)
    }
    
    /// Calculate Transmission Path Mapping (TPM)
    /// Measures propagation network complexity and reach
    fn calculate_tpm(paths: &[TransmissionPath]) -> f64 {
        if paths.is_empty() {
            return 0.0;
        }
        
        // Network diversity (unique platforms)
        let platforms: std::collections::HashSet<_> = 
            paths.iter().map(|p| &p.platform).collect();
        let platform_diversity = (platforms.len() as f64 * 10.0).min(30.0);
        
        // Path depth (number of transmission hops)
        let path_depth = (paths.len() as f64).log2() * 15.0;
        
        // Weight distribution (how balanced are the transmission weights)
        let avg_weight: f64 = paths.iter().map(|p| p.weight).sum::<f64>() / paths.len() as f64;
        let weight_variance: f64 = paths.iter()
            .map(|p| (p.weight - avg_weight).powi(2))
            .sum::<f64>() / paths.len() as f64;
        let weight_balance = (1.0 - weight_variance.sqrt().min(1.0)) * 25.0;
        
        // Time distribution (how spread out are the transmissions)
        let mut timestamps: Vec<_> = paths.iter().map(|p| p.timestamp.timestamp()).collect();
        timestamps.sort();
        let time_span = if timestamps.len() > 1 {
            (timestamps.last().unwrap() - timestamps.first().unwrap()) as f64 / 3600.0 // hours
        } else {
            0.0
        };
        let time_factor = (time_span / 24.0).min(1.0) * 30.0; // Max 30 points for 24+ hour spread
        
        let tpm = platform_diversity + path_depth + weight_balance + time_factor;
        tpm.min(100.0).max(0.0)
    }
    
    /// Calculate Quote Frequency (QF)
    /// Measures how often content is quoted vs simply shared
    fn calculate_qf(propagation: &PropagationData) -> f64 {
        if propagation.shares == 0 {
            return 0.0;
        }
        
        // Quote ratio (quotes vs total shares)
        let quote_ratio = propagation.quotes as f64 / propagation.shares as f64;
        
        // Volume factor (more quotes = higher score, but with diminishing returns)
        let volume_factor = (propagation.quotes as f64).log2().max(0.0);
        
        // Engagement context (quotes in relation to other engagements)
        let engagement_context = if propagation.likes + propagation.comments > 0 {
            propagation.quotes as f64 / (propagation.likes + propagation.comments) as f64
        } else {
            0.0
        };
        
        let qf = (quote_ratio * 40.0) + (volume_factor * 10.0) + (engagement_context * 50.0);
        qf.min(100.0).max(0.0)
    }
    
    /// Determine Echo Index tier based on score
    fn determine_tier(score: f64) -> String {
        match score {
            s if s >= 80.0 => "Gold".to_string(),
            s if s >= 60.0 => "Silver".to_string(),
            s if s >= 40.0 => "Bronze".to_string(),
            _ => "Basic".to_string(),
        }
    }
}

/// Calculate Echo Index for content
#[actix_web::post("/calculate")]
pub async fn calculate_echo_index(
    request: web::Json<EchoIndexRequest>,
) -> ActixResult<HttpResponse> {
    tracing::info!("Calculating Echo Index for content: {}", request.content_id);
    
    // In a real implementation, this would fetch propagation data from the database
    // For now, we'll use mock data based on the content metadata
    let propagation = PropagationData {
        shares: request.metadata.get("shares")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32,
        likes: request.metadata.get("likes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32,
        comments: request.metadata.get("comments")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32,
        quotes: request.metadata.get("quotes")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32,
        reach: request.metadata.get("reach")
            .and_then(|v| v.as_u64())
            .unwrap_or(1000) as u32,
        engagement_rate: request.metadata.get("engagement_rate")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.05),
        audience_quality: request.metadata.get("audience_quality")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.7),
        transmission_paths: vec![], // Would be populated from database
    };
    
    let echo_index = EchoIndex::calculate(&request, &propagation);
    
    let response = EchoIndexResponse {
        content_id: request.content_id.clone(),
        echo_index,
        calculated_at: Utc::now(),
        version: "1.0.0".to_string(),
    };
    
    tracing::info!("Echo Index calculated successfully: {}", response.echo_index.score);
    Ok(HttpResponse::Ok().json(response))
}

/// Get Echo Index for specific content
#[actix_web::get("/{content_id}")]
pub async fn get_echo_index(
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let content_id = path.into_inner();
    tracing::info!("Fetching Echo Index for content: {}", content_id);
    
    // In a real implementation, this would query the database
    // For now, return mock data
    let mock_echo_index = EchoIndex {
        odf: 75.5,
        awr: 82.3,
        tpm: 68.7,
        qf: 71.2,
        score: 74.4,
        tier: "Silver".to_string(),
    };
    
    let response = EchoIndexResponse {
        content_id,
        echo_index: mock_echo_index,
        calculated_at: Utc::now(),
        version: "1.0.0".to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

/// Get Echo Index leaderboard
#[actix_web::get("/leaderboard")]
pub async fn get_leaderboard(
    query: web::Query<HashMap<String, String>>,
) -> ActixResult<HttpResponse> {
    let limit: usize = query.get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10)
        .min(100);
    
    let platform = query.get("platform").cloned();
    let time_range = query.get("time_range").cloned().unwrap_or_else(|| "24h".to_string());
    
    tracing::info!("Fetching leaderboard with limit: {}, platform: {:?}, time_range: {}", 
                   limit, platform, time_range);
    
    // Mock leaderboard data
    let mut leaderboard = vec![
        LeaderboardEntry {
            rank: 1,
            content_id: "content_1".to_string(),
            title: "Revolutionary AI Breakthrough in Decentralized Networks".to_string(),
            author: "TechVisioneer".to_string(),
            echo_index: 94.7,
            tier: "Gold".to_string(),
            created_at: Utc::now() - chrono::Duration::hours(2),
        },
        LeaderboardEntry {
            rank: 2,
            content_id: "content_2".to_string(),
            title: "The Future of Attention Economics".to_string(),
            author: "AttentionGuru".to_string(),
            echo_index: 91.3,
            tier: "Gold".to_string(),
            created_at: Utc::now() - chrono::Duration::hours(5),
        },
        LeaderboardEntry {
            rank: 3,
            content_id: "content_3".to_string(),
            title: "Building Sustainable Creator Economies".to_string(),
            author: "CreatorAdvocate".to_string(),
            echo_index: 87.9,
            tier: "Gold".to_string(),
            created_at: Utc::now() - chrono::Duration::hours(8),
        },
        LeaderboardEntry {
            rank: 4,
            content_id: "content_4".to_string(),
            title: "Blockchain Gaming: The Next Big Wave".to_string(),
            author: "GameChanger".to_string(),
            echo_index: 83.5,
            tier: "Gold".to_string(),
            created_at: Utc::now() - chrono::Duration::hours(12),
        },
        LeaderboardEntry {
            rank: 5,
            content_id: "content_5".to_string(),
            title: "Democratizing Content Discovery".to_string(),
            author: "ContentCurator".to_string(),
            echo_index: 79.2,
            tier: "Silver".to_string(),
            created_at: Utc::now() - chrono::Duration::hours(18),
        },
    ];
    
    // Apply limit
    leaderboard.truncate(limit);
    
    Ok(HttpResponse::Ok().json(leaderboard))
}

/// Get historical Echo Index data for content
#[actix_web::get("/{content_id}/history")]
pub async fn get_echo_index_history(
    path: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
) -> ActixResult<HttpResponse> {
    let content_id = path.into_inner();
    let days: u32 = query.get("days")
        .and_then(|s| s.parse().ok())
        .unwrap_or(7)
        .min(365);
    
    tracing::info!("Fetching Echo Index history for content: {} (last {} days)", 
                   content_id, days);
    
    // Mock historical data
    let mut history = Vec::new();
    for i in 0..days {
        let timestamp = Utc::now() - chrono::Duration::days(days as i64 - i as i64);
        let base_score = 50.0;
        let variance = (i as f64 * 0.1).sin() * 20.0;
        let trend = i as f64 * 0.5;
        
        history.push(serde_json::json!({
            "timestamp": timestamp,
            "echo_index": (base_score + variance + trend).min(100.0).max(0.0),
            "odf": (base_score + variance * 0.8).min(100.0).max(0.0),
            "awr": (base_score + variance * 1.2).min(100.0).max(0.0),
            "tpm": (base_score + variance * 0.9).min(100.0).max(0.0),
            "qf": (base_score + variance * 1.1).min(100.0).max(0.0),
        }));
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "content_id": content_id,
        "history": history,
        "period_days": days,
    })))
} 