use axum::{routing::get, Json, Router};

use crate::models::HealthResponse;

pub fn app_router() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
