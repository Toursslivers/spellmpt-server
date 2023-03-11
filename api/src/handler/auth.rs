use axum::{http::StatusCode, Json};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use serde::Deserialize;

use crate::service::user::{create_user, find_user_by_phone};

#[derive(Deserialize)]
pub struct UserForm {
    phone_number: String,
    code: String,
}

pub async fn login(
    mut session: WritableSession,
    Json(payload): Json<UserForm>,
) -> (StatusCode, Json<serde_json::Value>) {
    let phone = payload.phone_number.clone();
    let user = find_user_by_phone(payload.phone_number).await.unwrap();
    match user {
        Some(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "status": "Not Found" })),
        ),
        None => {
            let new_user = create_user(phone).await.unwrap();
            return (
                StatusCode::OK,
                Json(serde_json::json!({ "user": new_user })),
            );
        }
    }
}

pub async fn login_out(mut session: WritableSession) -> (StatusCode, Json<serde_json::Value>) {
    session.destroy();
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    )
}
