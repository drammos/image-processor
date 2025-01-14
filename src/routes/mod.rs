use axum::Router;
pub mod handler;
// use crate::routes::{handler, user, restaurant};

pub fn routes() -> Router {
    Router::new().merge(handler::routes()) // Κοινά endpoints
}
