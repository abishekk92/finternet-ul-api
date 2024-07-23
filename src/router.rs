//! Main [axum::Router] interface for webserver.

use crate::{
    middleware::logging::{log_request_response, Logger},
    routes::{fallback::notfound_404, health, identity, ping, smartcontract},
};
use axum::{
    routing::{get, post, put},
    Router,
};

/// Setup main router for application.
pub fn setup_app_router() -> Router {
    Router::new()
        .layer(axum::middleware::from_fn(log_request_response::<Logger>))
        .route("/v1/identity", post(identity::create))
        .route("/v1/identity/:id/rotate_key", post(identity::rotate_key))
        .route("/v1/identity/:id/close", post(identity::close))
        .route(
            "/v1/identity/:id/asset_units",
            get(identity::get_asset_units),
        )
        .route("/v1/identity/:id/actions", get(identity::get_actions))
        .route(
            "/v1/identity/:id/submit_action",
            post(identity::submit_action),
        )
        .route(
            "/v/1/identity/:id/actions/:action_id",
            get(identity::get_action_status),
        )
        .route("/v1/smartcontract", post(smartcontract::create))
        .route(
            "/v1/smartcontract/:smartcontract_address/upgrade",
            put(smartcontract::upgrade),
        )
        .route(
            "/v1/smartcontract/:smartcontract_address/close",
            post(smartcontract::close),
        )
        .route(
            // Freeze probably not the best name for this endpoint
            // Could be confused with freezing assets
            "/v1/smartcontract/:smartcontract_address/freeze_upgrade",
            post(smartcontract::freeze_upgrade),
        )
        .route(
            "/v1/smartcontract/:smartcontract_address/execute",
            post(smartcontract::execute),
        )
        .route(
            "/v1/smartcontract/:smartcontract_address/dry_run",
            post(smartcontract::dry_run),
        )
        .route(
            "/v1/smartcontract/:smartcontract_address/estimate_fee",
            post(smartcontract::estimate_fee),
        )
        .route("/ping", get(ping::get))
        .route("/healthcheck", get(health::healthcheck))
        .fallback(notfound_404)
}
