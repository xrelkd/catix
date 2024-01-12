mod proxy;
mod system;

use axum::{routing, Router};

pub fn new_router() -> Router {
    Router::new()
        .nest("/api", Router::new().merge(self::system::v1()))
        .fallback(routing::any(self::proxy::execute))
}
