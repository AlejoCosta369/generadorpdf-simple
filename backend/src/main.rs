mod auth;
mod db;
mod models;
mod pdf;
mod routes;

use axum::Router;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

async fn ensure_admin_user(pool: &SqlitePool) {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM usuarios")
        .fetch_one(pool)
        .await
        .expect("failed to count usuarios");

    if count > 0 {
        return;
    }

    let username = std::env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let password = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin".to_string());
    let hash = auth::hash_password(&password);

    sqlx::query("INSERT INTO usuarios (username, password_hash) VALUES (?, ?)")
        .bind(&username)
        .bind(&hash)
        .execute(pool)
        .await
        .expect("failed to create admin user");

    tracing::info!("usuario admin creado: {username}");
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = db::init_pool().await;
    ensure_admin_user(&pool).await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", routes::router())
        .layer(cors)
        .with_state(pool);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
