//! Common types used across the application.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Public key or signing key.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PublicKey([u8; 32]);

/// Signature
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Signature([u8; 32]);

/// SignatureRecord
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SignatureRecord {
    signer: PublicKey,
    signature: Signature,
}

/// Context
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Context {
    contract_address: PublicKey,
    method: String,
    raw_args: Vec<u8>, // Deserialized into the actual arguments based on ABI/IDL at runtime
}

/// SmartContractTransaction
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SmartContractTransaction {
    payload: Vec<Context>, // Important for atomic composability
    signatures: Vec<SignatureRecord>,
}
