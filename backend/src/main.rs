use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware::Logger};
use log::info;
use std::env;

mod handlers;
mod models;
mod services;
mod utils;

use handlers::{health, auth, echo_index, content, users, propagation};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init();

    // Get server configuration from environment
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    info!("Starting EchoLayer Backend Server at {}:{}", host, port);

    // Start HTTP server
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    // Health check
                    .service(health::health_check)
                    .service(health::ready_check)
                    
                    // Authentication
                    .service(
                        web::scope("/auth")
                            .service(auth::login)
                            .service(auth::logout)
                            .service(auth::verify_token)
                            .service(auth::refresh_token)
                    )
                    
                    // Users
                    .service(
                        web::scope("/users")
                            .service(users::create_user)
                            .service(users::get_user)
                            .service(users::update_user)
                            .service(users::get_user_analytics)
                            .service(users::get_leaderboard)
                    )
                    
                    // Content
                    .service(
                        web::scope("/content")
                            .service(content::create_content)
                            .service(content::get_content)
                            .service(content::list_content)
                            .service(content::update_content)
                            .service(content::delete_content)
                    )
                    
                    // Echo Index
                    .service(
                        web::scope("/echo-index")
                            .service(echo_index::calculate_echo_index)
                            .service(echo_index::get_echo_index)
                            .service(echo_index::get_echo_index_history)
                            .service(echo_index::recalculate_echo_index)
                    )
                    
                    // Propagation
                    .service(
                        web::scope("/propagation")
                            .service(propagation::create_propagation)
                            .service(propagation::get_propagation_network)
                            .service(propagation::get_propagation_analytics)
                    )
            )
    })
    .bind((host.as_str(), port))?
    .run()
    .await
} 