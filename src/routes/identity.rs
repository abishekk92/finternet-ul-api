//! Identity route.

use crate::error::AppResult;
use axum::extract::Path;
use axum::{self, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Public key or signing key.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PublicKey([u8; 32]);

/// Signature
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Signature([u8; 32]);

/// New identity request.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewIdentity {
    signing_key: PublicKey, // Or could be bs58 encoded string
    // Add a message and signature to verify the identity
    message: String,
    signature: Signature,
}

/// POST /identity handler to create a new identity.
#[utoipa::path(
    post,
    path = "/identity",
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

/// Update identity request.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateIdentity {
    signing_key: PublicKey,
    // Add a message and signature to verify the identity
    message: String,
    signature: Signature,
}

/// PUT /identity/{id} handler to update an existing identity. i.e rotate keys
#[utoipa::path(
    put,
    path = "/identity/{id}",
    request_body = UpdateIdentity,
    responses(
        (status = 200, description = "Identity updated successfully", body=StatusCode),
        (status = NOT_FOUND, description = "Identity not found"),
        (status = 500, description = "Identity update wasn't successfull", body=AppError)
    ),
    params(
        ("id" = String, Path, description = "Public key (or) signing key of the identity")
    )
)]
pub async fn update(
    _id: Path<String>,
    Json(identity): Json<UpdateIdentity>,
) -> AppResult<StatusCode> {
    tracing::info!("Updating identity: {:?}", identity);
    Ok(StatusCode::OK)
}
