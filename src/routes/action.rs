//! Action route.

use crate::error::AppResult;
use crate::types::Action;
use axum::extract::Path;
use axum::{self, http::StatusCode, Json};

/// Submit a signed action.
#[utoipa::path(
    post,
    path = "/v1/ledger/action/submit",
    request_body = Action,
    responses(
        (status = 200, description = "Action submitted successfully", body=StatusCode),
        (status = NOT_FOUND, description = "Identity not found"),
        (status = 500, description = "Action submission wasn't successfull", body=AppError)
    ),
    params(
        ("id" = String, Path, description = "Public key (or) signing key of the identity")
    )
)]
pub async fn submit(Json(action): Json<Action>) -> AppResult<StatusCode> {
    tracing::info!("Submitting action: {:?}", action);
    Ok(StatusCode::OK)
}

/// Get action
#[utoipa::path(
    get,
    path = "/v1/ledger/action/:action_id",
    responses(
        (status = 200, description = "Action status retrieved successfully", body=StatusCode),
        (status = NOT_FOUND, description = "Action not found"),
        (status = 500, description = "Action status retrieval wasn't successfull", body=AppError)
    ),
    params(
        ("id" = String, Path, description = "Public key (or) signing key of the identity"),
        ("action_id" = String, Path, description = "Action ID")
    )
)]
pub async fn get(_action_id: Path<String>) -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}
