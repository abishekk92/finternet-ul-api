//! OpenAPI doc generation.

use crate::{
    error::AppError,
    routes::{action, contract, health, identity, ping},
};
use utoipa::OpenApi;

/// API documentation generator.
#[derive(OpenApi)]
#[openapi(
        paths(
               health::healthcheck,
               ping::get,
               identity::create,
               identity::update,
               contract::create,
               contract::update,
               contract::execute,
               action::transfer,
               action::lock,
               action::unlock,
        ),
        components(schemas(
                AppError,
                identity::PublicKey,
                identity::Signature,
                identity::NewIdentity,
                identity::UpdateIdentity,
                contract::NewContract,
                contract::UpdateContract
        )),
        tags(
            (name = "", description = "ul-api service/middleware")
        )
    )]

/// Tied to OpenAPI documentation.
#[derive(Debug)]
pub struct ApiDoc;
