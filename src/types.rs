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

/// Action
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Action {
    payload: ActionPayload,
    signature_record: SignatureRecord,
}

/// ActionPayload
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ActionPayload {
    action_type: ActionType,
    asset_type: AssetType,
    action_context: Context,
}

/// ActionType
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ActionType {
    /// Actions on Fungible Assets
    FungibleAssetAction,
    /// Actions on Non Fungible Assets
    NonFungibleAssetAction,
    /// Actions on Currency
    CurrencyAction,
    /// Actions on Property
    PropertyAction,
}

/// Actions on Fungible Assets
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum FungibleAssetAction {
    /// Send asset unit
    Send,
    /// Receive asset unit
    Receive,
    /// Tokenize asset unit
    Tokenize,
    /// Detokenize asset unit
    Detokenize,
    /// Chargeback asset unit
    Chargeback,
    /// Lock asset unit
    Lock,
    /// Unlock asset unit
    Unlock,
}

/// Actions on Currency
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum CurrencyAction {
    /// Send asset unit
    Send,
    /// Receive asset unit
    Receive,
    /// Tokenize asset unit
    Tokenize,
    /// Detokenize asset unit
    Detokenize,
}

/// Actions on Non Fungible Assets
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum NonFungibleAssetAction {
    /// Rent asset unit
    Rent,
    /// Lease asset unit
    Lease,
    /// Unlease asset unit
    Unlease,
    /// Transfer ownership of asset unit
    TransferOwnership,
}

/// Actions on Non Fungible Assets
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum PropertyAction {
    /// Rent asset unit
    Rent,
    /// Lease asset unit
    Lease,
    /// Unlease asset unit
    Unlease,
    /// Transfer ownership of asset unit
    TransferOwnership,
}

/// AssetType
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum AssetType {
    /// Fungible
    Fungible,
    /// NonFungible
    NonFungible,
    /// Currency
    Currency,
    /// Property
    Property,
    // ----- LATER -----
    // IntellectualProperty,
    // Debt,
    // Equity,
    // Commodity,
    // Derivative,
    // ----- XXXX -----
}

/// ActionFilter
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ActionFilter {
    action_type: ActionType,
    asset_type: AssetType,
    action_status: ActionStatus,
}

/// ActionStatus
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ActionStatus {
    /// Pending
    Pending,
    /// Executed
    Executed,
    /// Failed
    Failed,
    /// Cancelled
    Cancelled,
    /// Finalized
    Finalized,
}
