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
        components(schemas(AppError)),
        tags(
            (name = "", description = "ul-api service/middleware")
        )
    )]

/// Tied to OpenAPI documentation.
#[derive(Debug)]
pub struct ApiDoc;
