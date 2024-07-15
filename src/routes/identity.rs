//! Identity route.

use crate::error::AppResult;
use axum::{self, http::StatusCode};

/// POST /identity handler to create a new identity.
#[utoipa::path(
    post,
    path = "/identity",
    responses(
        (status = 200, description = "Identity created successfully", body=StatusCode),
        (status = 500, description = "Identity creation wasn't successfull", body=AppError)
    )
)]

pub async fn create() -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// PUT /identity/:id handler to update an existing identity. i.e rotate keys
#[utoipa::path(
    put,
    path = "/identity/:id",
    responses(
        (status = 200, description = "Identity updated successfully", body=StatusCode),
        (status = 500, description = "Identity update wasn't successfull", body=AppError)
    )
)]
pub async fn update() -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}
