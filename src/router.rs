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
        .route("/identity", post(identity::create))
        .route("/identity/:id", put(identity::update))
        .route("/contract", post(contract::create))
        .route(
            "/contract/:id",
            put(contract::update).delete(contract::delete),
        )
        .route("/contract/:id/execute", post(contract::execute))
        .route("/action/transfer", post(action::transfer))
        .route("/action/lock", post(action::lock))
        .route("/action/unlock", post(action::unlock))
        .fallback(notfound_404)
}
