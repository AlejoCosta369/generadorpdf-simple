mod auth;
mod clientes;
mod empresa;
mod productos;
mod remitos;

use axum::Router;
use sqlx::SqlitePool;

pub fn router() -> Router<SqlitePool> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/clientes", clientes::router())
        .nest("/empresa", empresa::router())
        .nest("/productos", productos::router())
        .nest("/remitos", remitos::router())
}
