//! Contract route.

use crate::error::AppResult;
use axum::extract::Path;
use axum::{self, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::identity;

/// Deploy new contract request.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewContract {
    binary_url: String,
    binary_checksum: String,
    signature: identity::Signature,
}

/// POST /contract handler to create a contract.
#[utoipa::path(
    post,
    path = "/contract",
    request_body = NewContract,
    responses(
        (status = 200, description = "Contract successfully created", body=StatusCode),
        (status = 500, description = "Contract deployment not successful", body=AppError)
    )
)]

pub async fn create(Json(contract): Json<NewContract>) -> AppResult<StatusCode> {
    tracing::info!("Creating new contract: {:?}", contract);
    Ok(StatusCode::OK)
}

/// Udate an existing contract request.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateContract {
    binary_url: String,
    binary_checksum: String,
    signature: identity::Signature,
}

/// PUT /contract/{contract_id} to update a contract.
#[utoipa::path(
    put,
    path = "/contract/{contract_id}",
    request_body = UpdateContract,
    responses(
        (status = 200, description = "Contract successfully updated", body=StatusCode),
        (status = NOT_FOUND, description = "Contract not found"),
        (status = 500, description = "Contract could not be updated", body=AppError)
    )
)]
pub async fn update(
    _contract_id: Path<String>,
    Json(update_contract): Json<UpdateContract>,
) -> AppResult<StatusCode> {
    tracing::info!("Updating contract: {:?}", update_contract);
    Ok(StatusCode::OK)
}

/// DELETE /contract/<contract_id> to update a contract.
#[utoipa::path(
    delete,
    path = "/contract/:contract_id",
    responses(
        (status = 200, description = "Contract successfully deleted", body=StatusCode),
        (status = NOT_FOUND, description = "Contract not found"),
        (status = 500, description = "Contract could not be deleted", body=AppError)
    )
)]
pub async fn delete(_contract_id: Path<String>) -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// Submit a transaction to a contract to be executed.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Transaction {
    contract_id: String,
    raw_transaction: String, // Roll up to a transaction type
    signature: identity::Signature,
}

/// POST /contract/<contract_id>/execute to execute a contract.
// NOTE: Tokenize/Detokenize should be submitted as transactions to be executed by the contract
#[utoipa::path(
    post,
    path = "/contract/:contract_id/execute",
    request_body = Transaction,
    responses(
        (status = 200, description = "Contract successfully executed", body=StatusCode),
        (status = NOT_FOUND, description = "Contract not found"),
        (status = 500, description = "Contract could not be executed", body=AppError)
    )
)]
pub async fn execute() -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}
