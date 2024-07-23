//! Identity route.

use crate::error::AppResult;
use crate::types::{Action, ActionFilter, PublicKey, Signature};
use axum::extract::Path;
use axum::{self, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// New identity request.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewIdentity {
    signing_key: PublicKey, // Or could be bs58 encoded string
    // Add a message and signature to verify the identity
    message: String,
    signature: Signature,
}

/// Create a new identity
#[utoipa::path(
    post,
    path = "/v1/identity",
    request_body = NewIdentity,
    responses(
        (status = 200, description = "Identity created successfully", body=StatusCode),
        (status = 500, description = "Identity creation wasn't successfull", body=AppError)
    )
)]

pub async fn create(Json(identity): Json<NewIdentity>) -> AppResult<StatusCode> {
    tracing::info!("Creating new identity: {:?}", identity);
    Ok(StatusCode::OK)
}

/// Rotate key to change the signing key of an existing identity.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RotateKey {
    signing_key: PublicKey,
    // Add a message and signature to verify the identity
    message: String,
    signature: Signature,
}

/// Rotate key to change the signing key of an existing identity.
#[utoipa::path(
    post,
    path = "/v1/identity/:id/rotate_key",
    request_body = RotateKey,
    responses(
        (status = 200, description = "Identity updated successfully", body=StatusCode),
        (status = NOT_FOUND, description = "Identity not found"),
        (status = 500, description = "Identity update wasn't successfull", body=AppError)
    ),
    params(
        ("id" = String, Path, description = "Public key (or) signing key of the identity")
    )
)]
pub async fn rotate_key(
    _id: Path<String>,
    Json(identity): Json<RotateKey>,
) -> AppResult<StatusCode> {
    tracing::info!("Updating identity: {:?}", identity);
    Ok(StatusCode::OK)
}

/// Close an identity. This is a soft delete, the identity can't be removed from the ledger since it's immutable.
#[utoipa::path(
    post,
    path = "/v1/identity/:id/close",
    responses(
        (status = 200, description = "Identity closed successfully", body=StatusCode),
        (status = NOT_FOUND, description = "Identity not found"),
        (status = 500, description = "Identity closing wasn't successfull", body=AppError)
    ),
    params(
        ("id" = String, Path, description = "Public key (or) signing key of the identity")
    )
)]
pub async fn close(_id: Path<String>) -> AppResult<StatusCode> {
    tracing::info!("Closing identity: {:?}", _id);
    Ok(StatusCode::OK)
}

/// Get asset_units of an identity.
#[utoipa::path(
    get,
    path = "/v1/identity/:id/asset_units",
    responses(
        (status = 200, description = "Asset Units retrieved successfully", body=StatusCode),
        (status = NOT_FOUND, description = "Identity not found"),
        (status = 500, description = "Asset Units retrieval wasn't successfull", body=AppError)
    ),
    params(
        ("id" = String, Path, description = "Public key (or) signing key of the identity")
    )
)]
pub async fn get_asset_units(_id: Path<String>) -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// Submit a signed action.
#[utoipa::path(
    post,
    path = "/v1/identity/:id/submit_action",
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
pub async fn submit_action(_id: Path<String>, Json(action): Json<Action>) -> AppResult<StatusCode> {
    tracing::info!("Submitting action: {:?}", action);
    Ok(StatusCode::OK)
}

/// Get status of an action.
#[utoipa::path(
    get,
    path = "/v1/identity/:id/actions/:action_id",
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
pub async fn get_action_status(
    _id: Path<String>,
    _action_id: Path<String>,
) -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// Get past actions of an identity. The actions can be filtered by type, time, etc.
#[utoipa::path(
    get,
    path = "/v1/identity/:id/actions",
    request_body = ActionFilter,
    responses(
        (status = 200, description = "Asset Units retrieved successfully", body=StatusCode),
        (status = NOT_FOUND, description = "Identity not found"),
        (status = 500, description = "Asset Units retrieval wasn't successfull", body=AppError)
    ),
    params(
        ("id" = String, Path, description = "Public key (or) signing key of the identity")
    )
)]
pub async fn get_actions(
    _id: Path<String>,
    Json(action_filter): Json<ActionFilter>,
) -> AppResult<StatusCode> {
    tracing::info!("Getting actions: {:?}", action_filter);
    Ok(StatusCode::OK)
}
