//! smartcontract route.

use crate::error::AppResult;
use crate::types::{Signature, SmartContractTransaction};
use axum::extract::Path;
use axum::{self, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Deploy a new smartcontract.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewSmartContract {
    binary_url: String,
    binary_checksum: String,
    signature: Signature,
}

/// Deploy a new smartcontract.
#[utoipa::path(
    post,
    path = "/v1/ledger/smartcontract",
    request_body = NewSmartContract,
    responses(
        (status = 200, description = "smartcontract successfully created", body=StatusCode),
        (status = 500, description = "smartcontract deployment not successful", body=AppError)
    )
)]

pub async fn create(Json(smartcontract): Json<NewSmartContract>) -> AppResult<StatusCode> {
    tracing::info!("Creating new smartcontract: {:?}", smartcontract);
    Ok(StatusCode::OK)
}

/// Retrieve a smartcontract's ABI or IDL
#[utoipa::path(
    get,
    path = "/v1/ledger/smartcontract/:smartcontract_address",
    request_body = Newsmartcontract,
    responses(
        (status = 200, description = "Smartcontract successfully created", body=StatusCode),
        (status = 500, description = "Smartcontract deployment not successful", body=AppError)
    )
)]
pub async fn get(_smartcontract_address: Path<String>) -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// Upgrade a smartcontract with a new binary.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpgradeSmartContract {
    binary_url: String,
    binary_checksum: String,
    signature: Signature,
}

/// Upgrade a smartcontract with a new binary.
#[utoipa::path(
    post,
    path = "/v1/ledger/smartcontract/:smartcontract_address/upgrade",
    request_body = UpgradeSmartContract,
    responses(
        (status = 200, description = "Smartcontract successfully updated", body=StatusCode),
        (status = NOT_FOUND, description = "Smartcontract not found"),
        (status = 500, description = "Smartcontract could not be updated", body=AppError)
    )
)]
pub async fn upgrade(
    _smartcontract_address: Path<String>,
    Json(update_smartcontract): Json<UpgradeSmartContract>,
) -> AppResult<StatusCode> {
    tracing::info!("Updating smartcontract: {:?}", update_smartcontract);
    Ok(StatusCode::OK)
}

/// Close a smartcontract. This is a soft delete, the smartcontract can't be removed from the ledger since it's immutable.
#[utoipa::path(
    post,
    path = "/v1/ledger/smartcontract/:smartcontract_address/close",
    responses(
        (status = 200, description = "Smartcontract successfully closed", body=StatusCode),
        (status = NOT_FOUND, description = "Smartcontract not found"),
        (status = 500, description = "Smartcontract could not be closed", body=AppError)
    )
)]
pub async fn close(_smartcontract_address: Path<String>) -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// Freeze a smartcontract from future upgrades.
#[utoipa::path(
    post,
    path = "/v1/ledger/smartcontract/:smartcontract_address/freeze_upgrade",
    responses(
        (status = 200, description = "Smartcontract successfully frozen from upgrades", body=StatusCode),
        (status = NOT_FOUND, description = "Smartcontract not found"),
        (status = 500, description = "Smartcontract could not be froze from upgrades", body=AppError)
    )
)]
pub async fn freeze_upgrade(_smartcontract_address: Path<String>) -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// Submit a transaction to be executed.
#[utoipa::path(
    post,
    path = "/v1/ledger/smartcontract/execute",
    request_body = Transaction,
    responses(
        (status = 200, description = "Smartcontract successfully executed", body=StatusCode),
        (status = NOT_FOUND, description = "Smartcontract not found"),
        (status = 500, description = "Smartcontract could not be executed", body=AppError)
    )
)]
pub async fn execute(Json(transaction): Json<SmartContractTransaction>) -> AppResult<StatusCode> {
    tracing::info!("Executing transaction: {:?}", transaction);
    // Should ideally return an ACK and a transaction ID
    Ok(StatusCode::OK)
}

/// Submit a transaction to be dry run.
#[utoipa::path(
    post,
    path = "/v1/ledger/smartcontract/dry_run",
    request_body = Transaction,
    responses(
        (status = 200, description = "Dry run successful", body=StatusCode),
        (status = NOT_FOUND, description = "Smartcontract not found"),
        (status = 500, description = "Dry run failed", body=AppError)
    )
)]
pub async fn dry_run(Json(transaction): Json<SmartContractTransaction>) -> AppResult<StatusCode> {
    tracing::info!("Dry run for transaction: {:?}", transaction);
    Ok(StatusCode::OK)
}

/// Submit a transaction to get an estimate of the fee.
#[utoipa::path(
    post,
    path = "/v1/ledger/smartcontract/estimate_fee",
    request_body = Transaction,
    responses(
        (status = 200, description = "Fee estimate for executing the transaction", body=StatusCode),
        (status = NOT_FOUND, description = "Smartcontract not found"),
        (status = 500, description = "Fee could not be estimated", body=AppError)
    )
)]
pub async fn estimate_fee(
    Json(transaction): Json<SmartContractTransaction>,
) -> AppResult<StatusCode> {
    tracing::info!("Estimating fee for transaction: {:?}", transaction);
    Ok(StatusCode::OK)
}
