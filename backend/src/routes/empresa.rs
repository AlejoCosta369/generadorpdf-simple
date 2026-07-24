use crate::auth::AuthUser;
use crate::models::{Empresa, EmpresaInput};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use sqlx::SqlitePool;

pub fn router() -> Router<SqlitePool> {
    Router::new().route("/", get(get_empresa).put(update_empresa))
}

async fn get_empresa(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
) -> Result<Json<Empresa>, (StatusCode, String)> {
    let empresa = sqlx::query_as::<_, Empresa>("SELECT * FROM empresa WHERE id = 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(empresa))
}

async fn update_empresa(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Json(input): Json<EmpresaInput>,
) -> Result<Json<Empresa>, (StatusCode, String)> {
    sqlx::query(
        "UPDATE empresa SET nombre = ?, direccion = ?, cuit = ?, telefono = ?, updated_at = datetime('now')
         WHERE id = 1",
    )
    .bind(&input.nombre)
    .bind(&input.direccion)
    .bind(&input.cuit)
    .bind(&input.telefono)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let empresa = sqlx::query_as::<_, Empresa>("SELECT * FROM empresa WHERE id = 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(empresa))
}
