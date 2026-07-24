use crate::auth::AuthUser;
use crate::models::{Cliente, Empresa, Producto, Remito, RemitoCompleto, RemitoInput, RemitoItem};
use crate::pdf::generate_remito_pdf;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use sqlx::SqlitePool;

pub fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(get_one))
        .route("/{id}/pdf", get(download_pdf))
}

async fn list(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
) -> Result<Json<Vec<Remito>>, (StatusCode, String)> {
    let remitos = sqlx::query_as::<_, Remito>("SELECT * FROM remitos ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(remitos))
}

async fn fetch_completo(
    pool: &SqlitePool,
    id: i64,
) -> Result<RemitoCompleto, (StatusCode, String)> {
    let remito = sqlx::query_as::<_, Remito>("SELECT * FROM remitos WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "remito no encontrado".to_string()))?;

    let items = sqlx::query_as::<_, RemitoItem>("SELECT * FROM remito_items WHERE remito_id = ?")
        .bind(id)
        .fetch_all(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(RemitoCompleto { remito, items })
}

async fn get_one(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<RemitoCompleto>, (StatusCode, String)> {
    Ok(Json(fetch_completo(&pool, id).await?))
}

async fn create(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Json(input): Json<RemitoInput>,
) -> Result<Json<RemitoCompleto>, (StatusCode, String)> {
    if input.items.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "el remito debe tener al menos un producto".to_string(),
        ));
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut total_centavos: i64 = 0;
    let mut line_items: Vec<(String, i64, i64, i64, i64)> = Vec::new(); // producto_id, precio, cantidad, subtotal

    for item in &input.items {
        if item.cantidad <= 0 {
            return Err((StatusCode::BAD_REQUEST, "cantidad invalida".to_string()));
        }
        let producto = sqlx::query_as::<_, Producto>("SELECT * FROM productos WHERE id = ?")
            .bind(item.producto_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or((
                StatusCode::BAD_REQUEST,
                format!("producto {} no encontrado", item.producto_id),
            ))?;

        let subtotal = producto.precio_centavos * item.cantidad;
        total_centavos += subtotal;
        line_items.push((
            producto.nombre,
            producto.id,
            producto.precio_centavos,
            item.cantidad,
            subtotal,
        ));
    }

    let remito_id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO remitos (cliente_id, total_centavos) VALUES (?, ?) RETURNING id",
    )
    .bind(input.cliente_id)
    .bind(total_centavos)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for (nombre, producto_id, precio, cantidad, subtotal) in &line_items {
        sqlx::query(
            "INSERT INTO remito_items (remito_id, producto_id, nombre_producto, precio_unitario_centavos, cantidad, subtotal_centavos)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(remito_id)
        .bind(producto_id)
        .bind(nombre)
        .bind(precio)
        .bind(cantidad)
        .bind(subtotal)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    tx.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(fetch_completo(&pool, remito_id).await?))
}

async fn download_pdf(
    State(pool): State<SqlitePool>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Response, (StatusCode, String)> {
    let completo = fetch_completo(&pool, id).await?;

    let cliente = sqlx::query_as::<_, Cliente>("SELECT * FROM clientes WHERE id = ?")
        .bind(completo.remito.cliente_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let empresa = sqlx::query_as::<_, Empresa>("SELECT * FROM empresa WHERE id = 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let bytes = generate_remito_pdf(&empresa, &cliente, &completo)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let filename = format!("remito-{:06}.pdf", completo.remito.id);

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/pdf".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename),
            ),
        ],
        Body::from(bytes),
    )
        .into_response())
}
