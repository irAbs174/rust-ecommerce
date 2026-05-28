mod config;
mod handlers;
mod models;
mod middleware;
mod services;
mod database;
mod errors;
mod utils;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::Config::from_env()?;
    tracing::info!("Starting e-commerce server on {}:{}", config.server_host, config.server_port);

    // Initialize database
    let db = database::init_db(&config.database_url).await?;
    let app_state = Arc::new(AppState { db });

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health::health_check))
        
        // Authentication routes
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/refresh", post(handlers::auth::refresh_token))
        
        // Products routes
        .route("/api/products", get(handlers::products::list_products))
        .route("/api/products/:id", get(handlers::products::get_product))
        .route("/api/products", post(handlers::products::create_product))
        .route("/api/products/:id", put(handlers::products::update_product))
        .route("/api/products/:id", delete(handlers::products::delete_product))
        
        // Categories routes
        .route("/api/categories", get(handlers::categories::list_categories))
        .route("/api/categories", post(handlers::categories::create_category))
        
        // Shopping Cart routes
        .route("/api/cart", get(handlers::cart::get_cart))
        .route("/api/cart/items", post(handlers::cart::add_to_cart))
        .route("/api/cart/items/:item_id", put(handlers::cart::update_cart_item))
        .route("/api/cart/items/:item_id", delete(handlers::cart::remove_from_cart))
        
        // Orders routes
        .route("/api/orders", post(handlers::orders::create_order))
        .route("/api/orders", get(handlers::orders::list_orders))
        .route("/api/orders/:id", get(handlers::orders::get_order))
        .route("/api/orders/:id/cancel", post(handlers::orders::cancel_order))
        
        // User routes
        .route("/api/users/profile", get(handlers::users::get_profile))
        .route("/api/users/profile", put(handlers::users::update_profile))
        .route("/api/users/addresses", get(handlers::users::list_addresses))
        .route("/api/users/addresses", post(handlers::users::add_address))
        
        // Review and Ratings
        .route("/api/products/:id/reviews", get(handlers::reviews::get_reviews))
        .route("/api/reviews", post(handlers::reviews::create_review))
        
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Run server
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
        .await?;
    
    tracing::info!("Server listening on http://{}:{}", config.server_host, config.server_port);
    axum::serve(listener, app).await?;

    Ok(())
}

pub struct AppState {
    pub db: sqlx::PgPool,
}
