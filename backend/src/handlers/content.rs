use actix_web::{get, post, put, delete, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateContentRequest {
    pub user_id: String,
    pub platform: String,
    pub external_id: String,
    pub content_type: String,
    pub title: String,
    pub body: String,
    pub media_urls: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
pub struct ContentResponse {
    pub id: String,
    pub user_id: String,
    pub platform: String,
    pub external_id: String,
    pub content_type: String,
    pub title: String,
    pub body: String,
    pub media_urls: Vec<String>,
    pub tags: Vec<String>,
    pub echo_index: f64,
    pub propagation_count: u32,
    pub total_rewards: f64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Create new content
#[post("")]
pub async fn create_content(content_data: web::Json<CreateContentRequest>) -> Result<HttpResponse> {
    let content = ContentResponse {
        id: Uuid::new_v4().to_string(),
        user_id: content_data.user_id.clone(),
        platform: content_data.platform.clone(),
        external_id: content_data.external_id.clone(),
        content_type: content_data.content_type.clone(),
        title: content_data.title.clone(),
        body: content_data.body.clone(),
        media_urls: content_data.media_urls.clone(),
        tags: content_data.tags.clone(),
        echo_index: 0.0, // Will be calculated by Echo Index engine
        propagation_count: 0,
        total_rewards: 0.0,
        status: "active".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(HttpResponse::Created().json(json!({
        "success": true,
        "data": content,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Get content by ID
#[get("/{content_id}")]
pub async fn get_content(path: web::Path<String>) -> Result<HttpResponse> {
    let content_id = path.into_inner();
    
    // Mock content data
    let content = ContentResponse {
        id: content_id,
        user_id: "user_123".to_string(),
        platform: "twitter".to_string(),
        external_id: "tweet_456".to_string(),
        content_type: "text".to_string(),
        title: "The Future of Decentralized Social Networks".to_string(),
        body: "Exploring how blockchain technology is revolutionizing social media and content monetization...".to_string(),
        media_urls: vec![],
        tags: vec!["blockchain".to_string(), "social".to_string(), "decentralized".to_string()],
        echo_index: 78.5,
        propagation_count: 25,
        total_rewards: 150.0,
        status: "active".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": content,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// List content with pagination
#[get("")]
pub async fn list_content(query: web::Query<ListContentQuery>) -> Result<HttpResponse> {
    // Mock content list
    let contents = vec![
        ContentResponse {
            id: "content_1".to_string(),
            user_id: "user_123".to_string(),
            platform: "twitter".to_string(),
            external_id: "tweet_456".to_string(),
            content_type: "text".to_string(),
            title: "The Future of Decentralized Social Networks".to_string(),
            body: "Exploring how blockchain technology is revolutionizing...".to_string(),
            media_urls: vec![],
            tags: vec!["blockchain".to_string(), "social".to_string()],
            echo_index: 78.5,
            propagation_count: 25,
            total_rewards: 150.0,
            status: "active".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
        ContentResponse {
            id: "content_2".to_string(),
            user_id: "user_124".to_string(),
            platform: "telegram".to_string(),
            external_id: "msg_789".to_string(),
            content_type: "image".to_string(),
            title: "EchoLayer Architecture Diagram".to_string(),
            body: "Visual representation of the EchoLayer ecosystem".to_string(),
            media_urls: vec!["https://example.com/diagram.png".to_string()],
            tags: vec!["architecture".to_string(), "diagram".to_string()],
            echo_index: 85.2,
            propagation_count: 42,
            total_rewards: 220.0,
            status: "active".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    ];

    let pagination = json!({
        "page": query.page.unwrap_or(1),
        "limit": query.limit.unwrap_or(20),
        "total": 2,
        "total_pages": 1
    });

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": contents,
        "pagination": pagination,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Update content
#[put("/{content_id}")]
pub async fn update_content(
    path: web::Path<String>,
    content_data: web::Json<CreateContentRequest>
) -> Result<HttpResponse> {
    let content_id = path.into_inner();
    
    let content = ContentResponse {
        id: content_id,
        user_id: content_data.user_id.clone(),
        platform: content_data.platform.clone(),
        external_id: content_data.external_id.clone(),
        content_type: content_data.content_type.clone(),
        title: content_data.title.clone(),
        body: content_data.body.clone(),
        media_urls: content_data.media_urls.clone(),
        tags: content_data.tags.clone(),
        echo_index: 78.5,
        propagation_count: 25,
        total_rewards: 150.0,
        status: "active".to_string(),
        created_at: "2024-01-01T12:00:00Z".to_string(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": content,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// Delete content
#[delete("/{content_id}")]
pub async fn delete_content(path: web::Path<String>) -> Result<HttpResponse> {
    let _content_id = path.into_inner();
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Content deleted successfully",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

#[derive(Deserialize)]
pub struct ListContentQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub user_id: Option<String>,
    pub platform: Option<String>,
    pub status: Option<String>,
} 