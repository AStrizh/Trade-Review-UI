use serde::{Deserialize, Serialize};

/// Represents the payload returned by the backend health endpoint.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Indicates whether the backend service is alive and accepting requests.
    pub status: &'static str,
}

/// Represents one OHLC candle in chart-ready shape for Lightweight Charts.
#[derive(Clone, Debug, Serialize)]
pub struct Candle {
    /// Candle open time represented as UTC epoch seconds.
    pub time: i64,
    /// Open price for the interval.
    pub open: f64,
    /// High price for the interval.
    pub high: f64,
    /// Low price for the interval.
    pub low: f64,
    /// Close price for the interval.
    pub close: f64,
}

/// Wraps candle output so the backend can extend response metadata later without breaking clients.
#[derive(Debug, Serialize)]
pub struct BarsResponse {
    /// Ordered candles in ascending timestamp order.
    pub candles: Vec<Candle>,
}

/// Represents query parameters accepted by the bars endpoint.
#[derive(Debug, Deserialize)]
pub struct BarsQuery {
    /// Contract or symbol identifier used to choose a dataset.
    pub contract: Option<String>,
    /// Inclusive lower date bound using YYYY-MM-DD format.
    pub start: Option<String>,
    /// Inclusive upper date bound using YYYY-MM-DD format.
    pub end: Option<String>,
}

/// Represents a structured API error payload used for bad requests.
#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    /// Short, user-readable description of the request error.
    pub message: String,
}
