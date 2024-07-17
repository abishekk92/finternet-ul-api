//! Action route.

use crate::error::AppResult;
use axum::{self, http::StatusCode};

/// POST /action/transfer handler to transfer an asset
#[utoipa::path(
    post,
    path = "/v1/action/transfer",
    responses(
        (status = 200, description = "Transfer successful", body=StatusCode),
        (status = 500, description = "Transfer not successful", body=AppError)
    )
)]

pub async fn transfer() -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// POST /action/lock handler to lock an asset
#[utoipa::path(
    post,
    path = "/v1/action/lock",
    responses(
        (status = 200, description = "Lock successful", body=StatusCode),
        (status = 500, description = "Lock not successful", body=AppError)
    )
)]

pub async fn lock() -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// POST /action/unlock handler to unlock an asset
#[utoipa::path(
    post,
    path = "/v1/action/unlock",
    responses(
        (status = 200, description = "unlock successful", body=StatusCode),
        (status = 500, description = "unlock not successful", body=AppError)
    )
)]

pub async fn unlock() -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}
