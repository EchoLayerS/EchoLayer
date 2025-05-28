use actix_web::{web, HttpResponse, Result as ActixResult, HttpRequest};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use std::collections::HashMap;

/// Wallet authentication request
#[derive(Deserialize)]
pub struct WalletAuthRequest {
    pub wallet_address: String,
    pub signature: String,
    pub message: String,
    pub wallet_type: WalletType,
    pub platform: Option<String>,
}

/// Wallet type enumeration
#[derive(Deserialize, Serialize, Clone)]
pub enum WalletType {
    #[serde(rename = "mpc")]
    MPC,
    #[serde(rename = "phantom")]
    Phantom,
    #[serde(rename = "solflare")]
    Solflare,
    #[serde(rename = "metamask")]
    MetaMask,
    #[serde(rename = "walletconnect")]
    WalletConnect,
}

/// Authentication response with tokens
#[derive(Serialize)]
pub struct AuthResponse {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub wallet_address: String,
    pub user_profile: UserProfile,
}

/// User profile information
#[derive(Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub user_id: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub total_echo_score: f64,
    pub tier: String,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub preferences: UserPreferences,
}

/// User preferences
#[derive(Serialize, Deserialize, Clone)]
pub struct UserPreferences {
    pub notifications_enabled: bool,
    pub email_notifications: bool,
    pub public_profile: bool,
    pub analytics_sharing: bool,
    pub theme: String, // "light", "dark", "auto"
    pub language: String,
}

/// Token refresh request
#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Token response
#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

/// Session information
#[derive(Serialize)]
pub struct SessionInfo {
    pub user_id: String,
    pub wallet_address: String,
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
}

/// JWT Claims structure
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,           // User ID
    pub wallet: String,        // Wallet address
    pub exp: usize,           // Expiration timestamp
    pub iat: usize,           // Issued at timestamp
    pub jti: String,          // JWT ID
    pub session_id: String,   // Session identifier
}

/// Authentication service implementation
pub struct AuthService;

impl AuthService {
    /// Verify wallet signature for authentication
    pub fn verify_wallet_signature(
        wallet_address: &str,
        signature: &str,
        message: &str,
        wallet_type: &WalletType,
    ) -> Result<bool, String> {
        // In a real implementation, this would verify the cryptographic signature
        // For different wallet types, we would use their respective signature schemes
        
        match wallet_type {
            WalletType::MPC => {
                // MPC wallet signature verification
                // This would integrate with the MPC wallet SDK
                tracing::info!("Verifying MPC wallet signature for: {}", wallet_address);
                
                // Mock verification - in production, use actual MPC verification
                if wallet_address.len() == 44 && signature.len() > 64 {
                    Ok(true)
                } else {
                    Err("Invalid MPC wallet signature".to_string())
                }
            },
            WalletType::Phantom | WalletType::Solflare => {
                // Solana wallet signature verification
                tracing::info!("Verifying Solana wallet signature for: {}", wallet_address);
                
                // Mock verification - in production, use ed25519 verification
                if wallet_address.len() == 44 && signature.len() > 64 {
                    Ok(true)
                } else {
                    Err("Invalid Solana wallet signature".to_string())
                }
            },
            WalletType::MetaMask | WalletType::WalletConnect => {
                // Ethereum wallet signature verification
                tracing::info!("Verifying Ethereum wallet signature for: {}", wallet_address);
                
                // Mock verification - in production, use secp256k1 verification
                if wallet_address.starts_with("0x") && wallet_address.len() == 42 {
                    Ok(true)
                } else {
                    Err("Invalid Ethereum wallet signature".to_string())
                }
            },
        }
    }
    
    /// Generate JWT access token
    pub fn generate_access_token(
        user_id: &str,
        wallet_address: &str,
        session_id: &str,
    ) -> Result<String, String> {
        let expiration = Utc::now() + Duration::hours(24);
        let claims = Claims {
            sub: user_id.to_string(),
            wallet: wallet_address.to_string(),
            exp: expiration.timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
        };
        
        // In production, use a proper JWT library with secret/key management
        let token = format!("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.{}.signature", 
                           base64::encode(serde_json::to_string(&claims).unwrap()));
        
        Ok(token)
    }
    
    /// Generate refresh token
    pub fn generate_refresh_token() -> String {
        Uuid::new_v4().to_string()
    }
    
    /// Create or update user profile
    pub fn create_user_profile(wallet_address: &str, wallet_type: &WalletType) -> UserProfile {
        let user_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        UserProfile {
            user_id,
            username: None,
            display_name: None,
            avatar_url: None,
            bio: None,
            total_echo_score: 0.0,
            tier: "Basic".to_string(),
            created_at: now,
            last_active: now,
            preferences: UserPreferences {
                notifications_enabled: true,
                email_notifications: false,
                public_profile: true,
                analytics_sharing: true,
                theme: "auto".to_string(),
                language: "en".to_string(),
            },
        }
    }
}

/// Authenticate user with wallet signature
#[actix_web::post("/login")]
pub async fn login_with_wallet(
    request: web::Json<WalletAuthRequest>,
    req: HttpRequest,
) -> ActixResult<HttpResponse> {
    tracing::info!("Authentication attempt for wallet: {}", request.wallet_address);
    
    // Verify wallet signature
    match AuthService::verify_wallet_signature(
        &request.wallet_address,
        &request.signature,
        &request.message,
        &request.wallet_type,
    ) {
        Ok(true) => {
            tracing::info!("Wallet signature verified successfully");
            
            // Create or retrieve user profile
            let user_profile = AuthService::create_user_profile(
                &request.wallet_address, 
                &request.wallet_type
            );
            
            // Generate session
            let session_id = Uuid::new_v4().to_string();
            
            // Generate tokens
            let access_token = AuthService::generate_access_token(
                &user_profile.user_id,
                &request.wallet_address,
                &session_id,
            ).map_err(|e| {
                tracing::error!("Failed to generate access token: {}", e);
                actix_web::error::ErrorInternalServerError("Token generation failed")
            })?;
            
            let refresh_token = AuthService::generate_refresh_token();
            
            // Store session information (in production, store in database/cache)
            tracing::info!("Session created for user: {}", user_profile.user_id);
            
            let response = AuthResponse {
                user_id: user_profile.user_id.clone(),
                access_token,
                refresh_token,
                expires_in: 24 * 3600, // 24 hours
                wallet_address: request.wallet_address.clone(),
                user_profile,
            };
            
            Ok(HttpResponse::Ok().json(response))
        },
        Ok(false) => {
            tracing::warn!("Invalid wallet signature for: {}", request.wallet_address);
            Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "invalid_signature",
                "message": "Wallet signature verification failed"
            })))
        },
        Err(e) => {
            tracing::error!("Signature verification error: {}", e);
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "verification_error",
                "message": e
            })))
        }
    }
}

/// Refresh access token using refresh token
#[actix_web::post("/refresh")]
pub async fn refresh_token(
    request: web::Json<RefreshTokenRequest>,
) -> ActixResult<HttpResponse> {
    tracing::info!("Token refresh requested");
    
    // In production, validate refresh token against database
    if request.refresh_token.len() < 10 {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "invalid_refresh_token",
            "message": "Invalid or expired refresh token"
        })));
    }
    
    // Mock user data - in production, retrieve from database using refresh token
    let user_id = "user_123";
    let wallet_address = "mock_wallet_address";
    let session_id = Uuid::new_v4().to_string();
    
    let new_access_token = AuthService::generate_access_token(
        user_id,
        wallet_address,
        &session_id,
    ).map_err(|e| {
        tracing::error!("Failed to generate new access token: {}", e);
        actix_web::error::ErrorInternalServerError("Token generation failed")
    })?;
    
    let response = TokenResponse {
        access_token: new_access_token,
        expires_in: 24 * 3600,
        token_type: "Bearer".to_string(),
    };
    
    tracing::info!("Access token refreshed successfully");
    Ok(HttpResponse::Ok().json(response))
}

/// Logout and invalidate session
#[actix_web::post("/logout")]
pub async fn logout(
    req: HttpRequest,
) -> ActixResult<HttpResponse> {
    // Extract token from Authorization header
    let auth_header = req.headers().get("Authorization");
    
    if let Some(token) = auth_header {
        if let Ok(token_str) = token.to_str() {
            if token_str.starts_with("Bearer ") {
                let token = &token_str[7..];
                tracing::info!("Logout requested for token: {}...", &token[..10]);
                
                // In production, invalidate token in database/cache
                // Add token to blacklist or remove session
                
                tracing::info!("Session invalidated successfully");
                return Ok(HttpResponse::Ok().json(serde_json::json!({
                    "message": "Logged out successfully"
                })));
            }
        }
    }
    
    Ok(HttpResponse::BadRequest().json(serde_json::json!({
        "error": "invalid_request",
        "message": "No valid authentication token provided"
    })))
}

/// Get current session information
#[actix_web::get("/session")]
pub async fn get_session_info(
    req: HttpRequest,
) -> ActixResult<HttpResponse> {
    // Extract and validate token
    let auth_header = req.headers().get("Authorization");
    
    if let Some(token) = auth_header {
        if let Ok(token_str) = token.to_str() {
            if token_str.starts_with("Bearer ") {
                let token = &token_str[7..];
                
                // In production, decode and validate JWT token
                tracing::info!("Session info requested for token: {}...", &token[..10]);
                
                // Mock session data
                let session_info = SessionInfo {
                    user_id: "user_123".to_string(),
                    wallet_address: "mock_wallet_address".to_string(),
                    session_id: Uuid::new_v4().to_string(),
                    created_at: Utc::now() - Duration::hours(2),
                    expires_at: Utc::now() + Duration::hours(22),
                    last_activity: Utc::now(),
                    device_info: req.headers().get("User-Agent")
                        .and_then(|h| h.to_str().ok())
                        .map(|s| s.to_string()),
                    ip_address: req.peer_addr().map(|addr| addr.ip().to_string()),
                };
                
                return Ok(HttpResponse::Ok().json(session_info));
            }
        }
    }
    
    Ok(HttpResponse::Unauthorized().json(serde_json::json!({
        "error": "unauthorized",
        "message": "Valid authentication token required"
    })))
}

/// Generate authentication challenge for wallet signing
#[actix_web::get("/challenge")]
pub async fn get_auth_challenge(
    query: web::Query<HashMap<String, String>>,
) -> ActixResult<HttpResponse> {
    let wallet_address = query.get("wallet")
        .ok_or_else(|| actix_web::error::ErrorBadRequest("wallet parameter required"))?;
    
    let platform = query.get("platform").cloned().unwrap_or_else(|| "web".to_string());
    
    tracing::info!("Challenge requested for wallet: {} on platform: {}", wallet_address, platform);
    
    // Generate unique challenge message
    let timestamp = Utc::now().timestamp();
    let nonce = Uuid::new_v4().to_string();
    
    let challenge_message = format!(
        "Welcome to EchoLayer!\n\nPlease sign this message to authenticate your wallet.\n\nWallet: {}\nTimestamp: {}\nNonce: {}\n\nThis signature will not trigger any blockchain transaction or cost any gas fees.",
        wallet_address,
        timestamp,
        nonce
    );
    
    let response = serde_json::json!({
        "challenge": challenge_message,
        "nonce": nonce,
        "timestamp": timestamp,
        "expires_in": 300, // 5 minutes
        "instructions": {
            "message": "Sign this message with your wallet to authenticate",
            "note": "This will not cost any gas or trigger transactions"
        }
    });
    
    Ok(HttpResponse::Ok().json(response))
}

/// Verify token validity (for middleware use)
#[actix_web::post("/verify")]
pub async fn verify_token(
    req: HttpRequest,
) -> ActixResult<HttpResponse> {
    let auth_header = req.headers().get("Authorization");
    
    if let Some(token) = auth_header {
        if let Ok(token_str) = token.to_str() {
            if token_str.starts_with("Bearer ") {
                let token = &token_str[7..];
                
                // In production, decode and validate JWT token
                tracing::info!("Token verification requested");
                
                // Mock validation - in production, check signature, expiration, etc.
                if token.len() > 20 {
                    return Ok(HttpResponse::Ok().json(serde_json::json!({
                        "valid": true,
                        "user_id": "user_123",
                        "wallet_address": "mock_wallet_address",
                        "expires_at": Utc::now() + Duration::hours(22)
                    })));
                }
            }
        }
    }
    
    Ok(HttpResponse::Unauthorized().json(serde_json::json!({
        "valid": false,
        "error": "invalid_token",
        "message": "Token is invalid or expired"
    })))
} 