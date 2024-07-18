//! OpenAPI doc generation.

use crate::{
    error::AppError,
    routes::{health, identity, ping, smartcontract},
    types::{Context, PublicKey, Signature, SignatureRecord, Transaction},
};
use utoipa::OpenApi;

/// API documentation generator.
#[derive(OpenApi)]
#[openapi(
        paths(
               identity::create,
               identity::rotate_key,
               identity::close,
               identity::balance,
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
                Transaction,
                Context,
                SignatureRecord,
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
