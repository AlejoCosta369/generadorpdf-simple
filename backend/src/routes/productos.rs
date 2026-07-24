use crate::auth::AuthUser;
use crate::models::{Producto, ProductoInput};
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
) -> Result<Json<Vec<Producto>>, (StatusCode, String)> {
    let productos = sqlx::query_as::<_, Producto>("SELECT * FROM productos ORDER BY nombre")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(productos))
}

async fn get_one(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<Producto>, (StatusCode, String)> {
    let producto = sqlx::query_as::<_, Producto>("SELECT * FROM productos WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "producto no encontrado".to_string()))?;
    Ok(Json(producto))
}

async fn create(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Json(input): Json<ProductoInput>,
) -> Result<Json<Producto>, (StatusCode, String)> {
    let id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO productos (nombre, precio_centavos, stock) VALUES (?, ?, ?) RETURNING id",
    )
    .bind(&input.nombre)
    .bind(input.precio_centavos)
    .bind(input.stock)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let producto = sqlx::query_as::<_, Producto>("SELECT * FROM productos WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(producto))
}

async fn update(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Path(id): Path<i64>,
    Json(input): Json<ProductoInput>,
) -> Result<Json<Producto>, (StatusCode, String)> {
    sqlx::query(
        "UPDATE productos SET nombre = ?, precio_centavos = ?, stock = ?, updated_at = datetime('now')
         WHERE id = ?",
    )
    .bind(&input.nombre)
    .bind(input.precio_centavos)
    .bind(input.stock)
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let producto = sqlx::query_as::<_, Producto>("SELECT * FROM productos WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "producto no encontrado".to_string()))?;

    Ok(Json(producto))
}

async fn remove(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM productos WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
