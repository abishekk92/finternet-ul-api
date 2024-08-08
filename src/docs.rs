//! OpenAPI doc generation.

use crate::{
    error::AppError,
    routes::{health, identity, ping, smartcontract},
    types::*,
};
use utoipa::OpenApi;

/// API documentation generator.
#[derive(OpenApi)]
#[openapi(
        paths(
               identity::create,
               identity::rotate_key,
               identity::close,
               identity::get_asset_units,
               smartcontract::create,
               smartcontract::upgrade,
               smartcontract::close,
               smartcontract::freeze_upgrade,
               smartcontract::execute,
               smartcontract::dry_run,
               smartcontract::estimate_fee,
               health::healthcheck,
               ping::get,
        ),
        components(schemas( AppError,
                PublicKey,
                Signature,
                SmartContractTransaction,
                Context,
                SignatureRecord,
                identity::NewIdentity,
                identity::RotateKey,
                smartcontract::NewSmartContract,
                smartcontract::UpgradeSmartContract,
        )),
        tags(
            (name = "Finternet Unified Ledger API", description = "Finternet Unified Ledger API")
        )
    )]

/// Tied to OpenAPI documentation.
#[derive(Debug)]
pub struct ApiDoc;
