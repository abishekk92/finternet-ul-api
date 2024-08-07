//! Main [axum::Router] interface for webserver.

use crate::{
    middleware::logging::{log_request_response, Logger},
    routes::{fallback::notfound_404, health, identity, ping, smartcontract},
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

/// Setup main router for application.
pub fn setup_app_router() -> Router {
    let identity_router = Router::new()
        .route("/", post(identity::create))
        .route("/:id/rotate_key", post(identity::rotate_key))
        .route("/:id/close", delete(identity::close))
        .route("/:id/asset_units", get(identity::get_asset_units));

    let smartcontract_router = Router::new()
        .route("/", post(smartcontract::create))
        .route(
            "/:smartcontract_address/upgrade",
            put(smartcontract::upgrade),
        )
        .route("/:smartcontract_address/close", post(smartcontract::close))
        .route(
            // Freeze probably not the best name for this endpoint
            // Could be confused with freezing assets
            "/:smartcontract_address/freeze_upgrade",
            post(smartcontract::freeze_upgrade),
        )
        .route(
            "/:smartcontract_address/execute",
            post(smartcontract::execute),
        )
        .route(
            "/:smartcontract_address/dry_run",
            post(smartcontract::dry_run),
        )
        .route(
            "/:smartcontract_address/estimate_fee",
            post(smartcontract::estimate_fee),
        );

    let ledger_router = Router::new()
        .nest("/identity", identity_router)
        .nest("/smartcontract", smartcontract_router);

    Router::new()
        .layer(axum::middleware::from_fn(log_request_response::<Logger>))
        .route("/ping", get(ping::get))
        .route("/healthcheck", get(health::healthcheck))
        .nest("/v1/ledger", ledger_router)
        .fallback(notfound_404)
}
