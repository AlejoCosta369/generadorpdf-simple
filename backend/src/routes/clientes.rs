use crate::auth::AuthUser;
use crate::models::{Cliente, ClienteInput};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use sqlx::SqlitePool;

pub fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(get_one).put(update).delete(remove))
}

async fn list(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
) -> Result<Json<Vec<Cliente>>, (StatusCode, String)> {
    let clientes = sqlx::query_as::<_, Cliente>("SELECT * FROM clientes ORDER BY nombre")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(clientes))
}

async fn get_one(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<Cliente>, (StatusCode, String)> {
    let cliente = sqlx::query_as::<_, Cliente>("SELECT * FROM clientes WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "cliente no encontrado".to_string()))?;
    Ok(Json(cliente))
}

async fn create(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Json(input): Json<ClienteInput>,
) -> Result<Json<Cliente>, (StatusCode, String)> {
    let id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO clientes (nombre, direccion, cuit_dni, telefono) VALUES (?, ?, ?, ?) RETURNING id",
    )
    .bind(&input.nombre)
    .bind(&input.direccion)
    .bind(&input.cuit_dni)
    .bind(&input.telefono)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let cliente = sqlx::query_as::<_, Cliente>("SELECT * FROM clientes WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(cliente))
}

async fn update(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Path(id): Path<i64>,
    Json(input): Json<ClienteInput>,
) -> Result<Json<Cliente>, (StatusCode, String)> {
    sqlx::query(
        "UPDATE clientes SET nombre = ?, direccion = ?, cuit_dni = ?, telefono = ?, updated_at = datetime('now')
         WHERE id = ?",
    )
    .bind(&input.nombre)
    .bind(&input.direccion)
    .bind(&input.cuit_dni)
    .bind(&input.telefono)
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let cliente = sqlx::query_as::<_, Cliente>("SELECT * FROM clientes WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "cliente no encontrado".to_string()))?;

    Ok(Json(cliente))
}

async fn remove(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM clientes WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
