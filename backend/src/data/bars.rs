use chrono::{NaiveDate, TimeZone, Utc};

use crate::models::Candle;

/// Stores available candle datasets keyed by contract name.
#[derive(Clone, Copy)]
pub struct BarsRepository;

impl BarsRepository {
    /// Creates a new bars repository instance used by request handlers.
    pub fn new() -> Self {
        Self
    }

    /// Returns candles for a contract filtered to an optional inclusive date window.
    pub fn load_bars(
        &self,
        contract: Option<&str>,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<Vec<Candle>, String> {
        let all_candles = match contract.unwrap_or("DEMO_CONTRACT") {
            "DEMO_CONTRACT" | "CLZ4_ohlcv1m" => seed_demo_candles(),
            other => return Err(format!("Unknown contract '{other}'")),
        };

        let start_time = parse_start_date(start)?;
        let end_time = parse_end_date(end)?;

        Ok(all_candles
            .into_iter()
            .filter(|candle| start_time.is_none_or(|min| candle.time >= min))
            .filter(|candle| end_time.is_none_or(|max| candle.time <= max))
            .collect())
    }
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

/// Produces deterministic sample candles used for Milestone 1 charting integration.
fn seed_demo_candles() -> Vec<Candle> {
    vec![
        Candle {
            time: 1729771800,
            open: 71.22,
            high: 71.32,
            low: 71.21,
            close: 71.25,
        },
        Candle {
            time: 1729772100,
            open: 71.25,
            high: 71.28,
            low: 71.12,
            close: 71.22,
        },
        Candle {
            time: 1729772400,
            open: 71.22,
            high: 71.40,
            low: 71.20,
            close: 71.36,
        },
        Candle {
            time: 1729772700,
            open: 71.36,
            high: 71.49,
            low: 71.35,
            close: 71.47,
        },
        Candle {
            time: 1729773000,
            open: 71.47,
            high: 71.55,
            low: 71.32,
            close: 71.38,
        },
        Candle {
            time: 1729773300,
            open: 71.38,
            high: 71.44,
            low: 71.22,
            close: 71.27,
        },
        Candle {
            time: 1729773600,
            open: 71.27,
            high: 71.29,
            low: 71.08,
            close: 71.15,
        },
        Candle {
            time: 1729773900,
            open: 71.15,
            high: 71.31,
            low: 71.14,
            close: 71.28,
        },
        Candle {
            time: 1729774200,
            open: 71.28,
            high: 71.34,
            low: 71.16,
            close: 71.21,
        },
        Candle {
            time: 1729774500,
            open: 71.21,
            high: 71.23,
            low: 71.00,
            close: 71.05,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::BarsRepository;

    /// Verifies that date filtering keeps only candles inside the inclusive date window.
    #[test]
    fn filters_bars_to_requested_range() {
        let repository = BarsRepository::new();
        let bars = repository
            .load_bars(
                Some("DEMO_CONTRACT"),
                Some("2024-10-24"),
                Some("2024-10-24"),
            )
            .expect("date query should be valid");

        assert_eq!(bars.len(), 10);
    }

    /// Verifies that malformed dates are rejected with a useful error message.
    #[test]
    fn rejects_bad_dates() {
        let repository = BarsRepository::new();
        let err = repository
            .load_bars(Some("DEMO_CONTRACT"), Some("10/24/2024"), None)
            .expect_err("invalid date must fail");

        assert!(err.contains("Expected YYYY-MM-DD"));
    }
}
