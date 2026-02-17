use std::path::PathBuf;

use chrono::{NaiveDate, TimeZone, Utc};
use polars::lazy::dsl::{col, lit};
use polars::prelude::*;

use crate::models::{Candle, IndicatorPoint, IndicatorSeries};

const DEFAULT_PARQUET_PATH: &str = "../sample.parquet";
const INDICATOR_COLUMNS: [&str; 9] = [
    "vwap",
    "vwapn",
    "vwapd",
    "ema_9",
    "ema_14",
    "ema_21",
    "rsi_14_ema",
    "rsi_14_wilder",
    "atr_14",
];

/// Reads market bars and indicator values from a configured Parquet source.
#[derive(Clone)]
pub struct BarsRepository {
    parquet_path: PathBuf,
}

impl BarsRepository {
    /// Creates a repository using an environment-configurable parquet file path.
    pub fn new() -> Self {
        let parquet_path = std::env::var("BARS_PARQUET_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(DEFAULT_PARQUET_PATH));

        Self { parquet_path }
    }

    /// Returns candles for an optional contract and optional inclusive date window.
    pub fn load_bars(
        &self,
        contract: Option<&str>,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<Vec<Candle>, String> {
        let start_time = parse_start_date(start)?;
        let end_time = parse_end_date(end)?;

        let mut query = self.base_query()?;

        if let Some(contract) = contract {
            query = query.filter(col("contract").eq(lit(contract)));
        }

        if let Some(start_time) = start_time {
            query = query.filter(
                col("timestamp")
                    .cast(DataType::Int64)
                    .gt_eq(lit(start_time * 1000)),
            );
        }

        if let Some(end_time) = end_time {
            query = query.filter(
                col("timestamp")
                    .cast(DataType::Int64)
                    .lt_eq(lit(end_time * 1000)),
            );
        }

        let df = query
            .select([
                col("timestamp"),
                col("open"),
                col("high"),
                col("low"),
                col("close"),
            ])
            .sort(["timestamp"], Default::default())
            .collect()
            .map_err(|error| format!("Failed to load bars from parquet: {error}"))?;

        map_dataframe_to_candles(df)
    }

    /// Returns chart-ready indicator series for available indicator columns.
    pub fn load_series(
        &self,
        contract: Option<&str>,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<Vec<IndicatorSeries>, String> {
        let start_time = parse_start_date(start)?;
        let end_time = parse_end_date(end)?;

        let mut query = self.base_query()?;

        if let Some(contract) = contract {
            query = query.filter(col("contract").eq(lit(contract)));
        }

        if let Some(start_time) = start_time {
            query = query.filter(
                col("timestamp")
                    .cast(DataType::Int64)
                    .gt_eq(lit(start_time * 1000)),
            );
        }

        if let Some(end_time) = end_time {
            query = query.filter(
                col("timestamp")
                    .cast(DataType::Int64)
                    .lt_eq(lit(end_time * 1000)),
            );
        }

        let select_exprs = std::iter::once(col("timestamp"))
            .chain(INDICATOR_COLUMNS.into_iter().map(col))
            .collect::<Vec<_>>();

        let df = query
            .select(select_exprs)
            .sort(["timestamp"], Default::default())
            .collect()
            .map_err(|error| format!("Failed to load series from parquet: {error}"))?;

        map_dataframe_to_series(df)
    }

    fn base_query(&self) -> Result<LazyFrame, String> {
        if !self.parquet_path.exists() {
            return Err(format!(
                "Parquet file not found at '{}'. Set BARS_PARQUET_PATH to override.",
                self.parquet_path.display()
            ));
        }

        LazyFrame::scan_parquet(self.parquet_path.clone(), ScanArgsParquet::default()).map_err(
            |error| {
                format!(
                    "Unable to open parquet file '{}': {error}",
                    self.parquet_path.display()
                )
            },
        )
    }
}

fn map_dataframe_to_candles(df: DataFrame) -> Result<Vec<Candle>, String> {
    let timestamps = df
        .column("timestamp")
        .map_err(|error| format!("Missing timestamp column: {error}"))?
        .cast(&DataType::Int64)
        .map_err(|error| format!("Timestamp conversion failed: {error}"))?;
    let opens = df
        .column("open")
        .map_err(|error| format!("Missing open column: {error}"))?
        .f64()
        .map_err(|error| format!("Open conversion failed: {error}"))?;
    let highs = df
        .column("high")
        .map_err(|error| format!("Missing high column: {error}"))?
        .f64()
        .map_err(|error| format!("High conversion failed: {error}"))?;
    let lows = df
        .column("low")
        .map_err(|error| format!("Missing low column: {error}"))?
        .f64()
        .map_err(|error| format!("Low conversion failed: {error}"))?;
    let closes = df
        .column("close")
        .map_err(|error| format!("Missing close column: {error}"))?
        .f64()
        .map_err(|error| format!("Close conversion failed: {error}"))?;
    let timestamp_values = timestamps
        .i64()
        .map_err(|error| format!("Timestamp conversion failed: {error}"))?;

    let mut candles = Vec::with_capacity(df.height());

    for idx in 0..df.height() {
        let Some(timestamp_ms) = timestamp_values.get(idx) else {
            continue;
        };

        let (Some(open), Some(high), Some(low), Some(close)) = (
            opens.get(idx),
            highs.get(idx),
            lows.get(idx),
            closes.get(idx),
        ) else {
            continue;
        };

        candles.push(Candle {
            time: timestamp_ms / 1000,
            open,
            high,
            low,
            close,
        });
    }

    Ok(candles)
}

fn map_dataframe_to_series(df: DataFrame) -> Result<Vec<IndicatorSeries>, String> {
    let timestamps = df
        .column("timestamp")
        .map_err(|error| format!("Missing timestamp column: {error}"))?
        .cast(&DataType::Int64)
        .map_err(|error| format!("Timestamp conversion failed: {error}"))?;
    let timestamp_values = timestamps
        .i64()
        .map_err(|error| format!("Timestamp conversion failed: {error}"))?;

    let mut series = Vec::with_capacity(INDICATOR_COLUMNS.len());

    for indicator in INDICATOR_COLUMNS {
        let values = match df.column(indicator) {
            Ok(column) => column
                .f64()
                .map_err(|error| format!("{indicator} conversion failed: {error}"))?,
            Err(_) => continue,
        };

        let mut points = Vec::new();

        for idx in 0..df.height() {
            let Some(timestamp_ms) = timestamp_values.get(idx) else {
                continue;
            };
            let Some(value) = values.get(idx) else {
                continue;
            };

            if value.is_nan() {
                continue;
            }

            points.push(IndicatorPoint {
                time: timestamp_ms / 1000,
                value,
            });
        }

        let pane = if indicator.starts_with("rsi") {
            "rsi"
        } else if indicator.starts_with("atr") {
            "atr"
        } else {
            "price"
        };

        series.push(IndicatorSeries {
            id: indicator.to_string(),
            name: indicator.replace('_', " ").to_uppercase(),
            kind: "line".to_string(),
            pane: pane.to_string(),
            data: points,
        });
    }

    Ok(series)
}

/// Converts a date string into the first UTC second included in the requested range.
fn parse_start_date(value: Option<&str>) -> Result<Option<i64>, String> {
    let Some(value) = value else {
        return Ok(None);
    };

    let date = parse_yyyy_mm_dd(value)?;
    Ok(Some(
        Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).expect("valid midnight"))
            .timestamp(),
    ))
}

/// Converts a date string into the final UTC second included in the requested range.
fn parse_end_date(value: Option<&str>) -> Result<Option<i64>, String> {
    let Some(value) = value else {
        return Ok(None);
    };

    let date = parse_yyyy_mm_dd(value)?;
    Ok(Some(
        Utc.from_utc_datetime(&date.and_hms_opt(23, 59, 59).expect("valid day end"))
            .timestamp(),
    ))
}

/// Parses a YYYY-MM-DD date string into a chrono date value.
fn parse_yyyy_mm_dd(value: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| format!("Invalid date '{value}'. Expected YYYY-MM-DD."))
}

#[cfg(test)]
mod tests {
    use super::BarsRepository;

    #[test]
    fn reads_sample_parquet_bars() {
        let repository = BarsRepository::new();
        let bars = repository
            .load_bars(None, None, None)
            .expect("parquet should load");

        assert!(!bars.is_empty());
    }

    #[test]
    fn rejects_bad_dates() {
        let repository = BarsRepository::new();
        let err = repository
            .load_bars(None, Some("10/24/2024"), None)
            .expect_err("invalid date must fail");

        assert!(err.contains("Expected YYYY-MM-DD"));
    }
}
