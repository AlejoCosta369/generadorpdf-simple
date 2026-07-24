use crate::auth::{clear_cookie, create_token, session_cookie, verify_password, AuthUser};
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

pub fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

#[derive(Deserialize)]
struct LoginInput {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserOut {
    id: i64,
    username: String,
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: i64,
    username: String,
    password_hash: String,
}

async fn login(
    State(pool): State<SqlitePool>,
    Json(input): Json<LoginInput>,
) -> Result<Response, (StatusCode, &'static str)> {
    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id, username, password_hash FROM usuarios WHERE username = ?",
    )
    .bind(&input.username)
    .fetch_optional(&pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "error de base de datos"))?
    .ok_or((StatusCode::UNAUTHORIZED, "usuario o contrasena invalidos"))?;

    if !verify_password(&input.password, &user.password_hash) {
        return Err((StatusCode::UNAUTHORIZED, "usuario o contrasena invalidos"));
    }

    let token = create_token(user.id);
    let cookie = session_cookie(token);

    let body = Json(UserOut {
        id: user.id,
        username: user.username,
    });

    let mut response = body.into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        cookie.to_string().parse().expect("valid cookie header"),
    );
    Ok(response)
}

async fn logout() -> Response {
    let cookie = clear_cookie();
    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        cookie.to_string().parse().expect("valid cookie header"),
    );
    response
}

async fn me(
    State(pool): State<SqlitePool>,
    auth: AuthUser,
) -> Result<Json<UserOut>, (StatusCode, &'static str)> {
    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id, username, password_hash FROM usuarios WHERE id = ?",
    )
    .bind(auth.user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "error de base de datos"))?
    .ok_or((StatusCode::UNAUTHORIZED, "no autenticado"))?;

    Ok(Json(UserOut {
        id: user.id,
        username: user.username,
    }))
}
