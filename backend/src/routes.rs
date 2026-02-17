use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::{
    data::bars::BarsRepository,
    models::{ApiErrorResponse, BarsQuery, BarsResponse, HealthResponse, SeriesResponse},
};

/// Shared application state containing repositories used by route handlers.
#[derive(Clone)]
pub struct AppState {
    /// Access layer that serves chart bars for requested contracts and date ranges.
    pub bars_repository: BarsRepository,
}

impl AppState {
    /// Constructs application state with default repository instances.
    pub fn new() -> Self {
        Self {
            bars_repository: BarsRepository::new(),
        }
    }
}

/// Builds the HTTP router and wires all endpoint handlers.
pub fn app_router() -> Router {
    let state = AppState::new();

    Router::new()
        .route("/health", get(health))
        .route("/bars", get(bars))
        .route("/series", get(series))
        .with_state(state)
}

/// Returns a simple liveliness probe used by frontend boot-time checks.
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

/// Returns candlestick bars filtered by contract and date query parameters.
async fn bars(
    State(state): State<AppState>,
    Query(query): Query<BarsQuery>,
) -> Result<Json<BarsResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    let candles = state
        .bars_repository
        .load_bars(
            query.contract.as_deref(),
            query.start.as_deref(),
            query.end.as_deref(),
        )
        .map_err(|message| (StatusCode::BAD_REQUEST, Json(ApiErrorResponse { message })))?;

    Ok(Json(BarsResponse { candles }))
}

/// Returns indicator series filtered by contract and date query parameters.
async fn series(
    State(state): State<AppState>,
    Query(query): Query<BarsQuery>,
) -> Result<Json<SeriesResponse>, (StatusCode, Json<ApiErrorResponse>)> {
    let series = state
        .bars_repository
        .load_series(
            query.contract.as_deref(),
            query.start.as_deref(),
            query.end.as_deref(),
        )
        .map_err(|message| (StatusCode::BAD_REQUEST, Json(ApiErrorResponse { message })))?;

    Ok(Json(SeriesResponse { series }))
}
