//! Identity route.

use crate::error::AppResult;
use crate::types::{PublicKey, Signature};
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
    path = "/v1/ledger/identity",
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
    path = "/v1/ledger/identity/:id/rotate_key",
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
    delete,
    path = "/v1/ledger/identity/:id/close",
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
    path = "/v1/ledger/identity/:id/asset_units",
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

/// Attach a verifiable credential to an identity.
#[utoipa::path(
    post,
    path = "/v1/ledger/identity/:id/attach_credential",
    responses(
        (status = 200, description = "Credential attached successfully", body=StatusCode),
        (status = NOT_FOUND, description = "Identity not found"),
        (status = 500, description = "Credential attachment wasn't successfull", body=AppError)
    ),
    params(
        ("id" = String, Path, description = "Public key (or) signing key of the identity")
    )
)]
pub async fn attach_credential(_id: Path<String>) -> AppResult<StatusCode> {
    tracing::info!("Attaching credential to id: {:?}", _id);
    Ok(StatusCode::OK)
}
