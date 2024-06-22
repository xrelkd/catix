mod proxy;
mod system;

use axum::{routing, Router};

pub fn new_router() -> Router {
    Router::new()
        .nest("/api", Router::new().merge(system::v1()))
        .fallback(routing::any(proxy::execute))
}
