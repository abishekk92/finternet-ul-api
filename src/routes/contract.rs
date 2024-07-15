//! Contract route.

use crate::error::AppResult;
use axum::{self, http::StatusCode};

/// POST /contract handler to create a contract.
#[utoipa::path(
    post,
    path = "/contract",
    responses(
        (status = 200, description = "Contract successfully created", body=StatusCode),
        (status = 500, description = "Contract not successful", body=AppError)
    )
)]

pub async fn create() -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// PUT /contract/<contract_id> to update a contract.
#[utoipa::path(
    put,
    path = "/contract/:contract_id",
    responses(
        (status = 200, description = "Contract successfully updated", body=StatusCode),
        (status = 500, description = "Contract not successful", body=AppError)
    )
)]
pub async fn update() -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// DELETE /contract/<contract_id> to update a contract.
#[utoipa::path(
    delete,
    path = "/contract/:contract_id",
    responses(
        (status = 200, description = "Contract successfully deleted", body=StatusCode),
        (status = 500, description = "Contract not successful", body=AppError)
    )
)]
pub async fn delete() -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}

/// POST /contract/<contract_id>/execute to execute a contract.
// NOTE: Tokenize/Detokenize should be submitted as transactions to be executed by the contract
#[utoipa::path(
    post,
    path = "/contract/:contract_id/execute",
    responses(
        (status = 200, description = "Contract successfully executed", body=StatusCode),
        (status = 500, description = "Contract not successful", body=AppError)
    )
)]
pub async fn execute() -> AppResult<StatusCode> {
    Ok(StatusCode::OK)
}
