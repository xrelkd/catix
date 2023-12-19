mod system;

use axum::Router;

pub fn api_v1_router() -> Router {
    Router::new().nest("/api", Router::new().merge(self::system::v1()))
}
