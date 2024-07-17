//! Main [axum::Router] interface for webserver.

use crate::{
    middleware::logging::{log_request_response, Logger},
    routes::{action, contract, fallback::notfound_404, health, identity, ping},
};
use axum::{
    routing::{get, post, put},
    Router,
};

/// Setup main router for application.
pub fn setup_app_router() -> Router {
    Router::new()
        .layer(axum::middleware::from_fn(log_request_response::<Logger>))
        .route("/ping", get(ping::get))
        .route("/healthcheck", get(health::healthcheck))
        .route("/v1/identity", post(identity::create))
        .route(
            "/v1/identity/:id",
            put(identity::update).delete(identity::delete),
        )
        .route("/v1/contract", post(contract::create))
        .route(
            "/v1/contract/:contract_id",
            put(contract::update).delete(contract::delete),
        )
        .route("/v1/contract/:contract_id/execute", post(contract::execute))
        .route(
            "/v1/contract/:contract_id/estimate_fee",
            post(contract::estimate_fee),
        )
        .route("/v1/action/transfer", post(action::transfer))
        .route("/v1/action/lock", post(action::lock))
        .route("/v1/action/unlock", post(action::unlock))
        .fallback(notfound_404)
}
