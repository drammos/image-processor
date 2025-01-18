use axum::Router;
pub mod handler;
pub fn routes() -> Router {
    Router::new().merge(handler::routes()) // Κοινά endpoints
}
