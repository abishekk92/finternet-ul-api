//! OpenAPI doc generation.

use crate::{
    error::AppError,
    routes::{action, health, identity, ping, smartcontract},
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
               identity::actions,
               action::submit,
               action::get,
               smartcontract::create,
               smartcontract::upgrade,
               smartcontract::close,
               smartcontract::freeze_upgrade,
               smartcontract::execute,
               smartcontract::dry_run,
               health::healthcheck,
               ping::get,
        ),
        components(schemas(
                AppError,
                PublicKey,
                Signature,
                SmartContractTransaction,
                Context,
                SignatureRecord,
                Action,
                ActionFilter,
                ActionStatus,
                ActionPayload,
                ActionType,
                AssetType,
                FungibleAssetAction,
                NonFungibleAssetAction,
                CurrencyAction,
                PropertyAction,
                identity::NewIdentity,
                identity::RotateKey,
                smartcontract::NewSmartContract,
                smartcontract::UpgradeSmartContract,
        )),
        tags(
            (name = "", description = "ul-api service/middleware")
        )
    )]

/// Tied to OpenAPI documentation.
#[derive(Debug)]
pub struct ApiDoc;
