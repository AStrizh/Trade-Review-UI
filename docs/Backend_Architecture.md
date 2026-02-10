# Backend Architecture

## 1. Purpose

This document describes the **technical architecture of the backend data service** for the Trade Review UI system.

The backend is responsible for ingesting local market and trade data, normalizing it into canonical time-series models, validating alignment and correctness, and exposing this data via a local HTTP API for consumption by a frontend visualization layer.

This document focuses specifically on **Rust tooling, data flow, and backend design decisions**, and complements the higher-level system overview described in `System_Architecture.md`.

---

## 2. Backend Role in the System

The backend acts as a **local, read-only data adapter** between raw backtesting outputs and a chart-oriented frontend.

It is **not**:

* a strategy engine
* a calculation engine for new indicators
* a persistent data store
* a visualization layer

It **is**:

* a Parquet and CSV reader
* a time-series normalization layer
* a correctness and alignment gatekeeper
* a chart-data provider

The backend’s primary objective is to **reduce frontend complexity** by ensuring that all data returned is:

* correctly aligned in time
* correctly typed
* shaped to match the expectations of the charting library

---

## 3. Technology Stack

### 3.1 Core Libraries

* **Language:** Rust
* **Web Framework:** `axum`
* **Async Runtime:** `tokio`
* **Serialization:** `serde`, `serde_json`
* **Parquet Processing:** `polars`
* **CSV Parsing:** `csv`
* **Time Handling:** `chrono`
* **Error Handling:** `thiserror` or `anyhow`
* **HTTP Middleware:** `tower`, `tower-http` (CORS, tracing)

This stack is chosen to:

* efficiently handle large Parquet datasets
* provide strong typing and explicit error handling
* integrate cleanly with a Rust-based backtesting ecosystem

---

## 4. Data Ingestion Strategy

### 4.1 Parquet Input (Market Data + Indicators)

The backend ingests Parquet files containing **wide-format time-series data**, where each row represents one bar for one contract.

Example columns include:

* Core OHLCV:

  * `timestamp`
  * `open`, `high`, `low`, `close`, `volume`
* Indicators:

  * `vwap`, `ema_9`, `ema_14`, `ema_21`
  * `rsi_14_ema`, `rsi_14_wilder`
  * `atr_14`
* Identifier:

  * `contract`

#### Interpretation Rules

* Each row corresponds to **one bar** for **one contract**
* Indicator values are **bar-aligned**
* `timestamp` is the authoritative temporal reference

### 4.2 Timestamp Normalization

The backend must convert timestamps into a **single canonical representation**:

* **UTC**
* **Epoch seconds**
* Representing **bar open time** (unless explicitly documented otherwise)

This normalization happens **once**, at ingestion time, and all downstream logic relies on it.

### 4.3 NaN and Missing Data Handling

Because Parquet supports `NaN` but JSON does not:

* Indicator values that are `NaN` must be:

  * either omitted from the resulting series
  * or converted to `null` (only if required)

**Default policy:** omit points with missing values.

This policy is chosen to align with TradingView Lightweight Charts’ expectation that indicator series may have gaps.

---

## 5. Internal Data Model (Conceptual)

Internally, the backend operates on three conceptual models:

1. **Bars**

   * Time-indexed OHLCV data
2. **Indicator Series**

   * One logical series per indicator column
3. **Trades**

   * Entry/exit events with metadata

These internal models are not necessarily exposed directly; instead, they are transformed into **chart-ready API models**.

---

## 6. Trade CSV Ingestion

### 6.1 Trade Mapping Responsibility

The backend maps trade CSV columns into a **canonical Trade model** regardless of CSV structure.

This allows:

* changes in backtester output format
* multiple backtest versions
* future adapters for other systems

The frontend must not need to understand CSV column semantics.

### 6.2 Trade Validation

For each trade, the backend performs basic validation checks:

* **Time alignment**

  * Entry/exit timestamps must be close to a bar timestamp
* **Price plausibility**

  * Entry/exit prices should fall within or near the corresponding bar’s high/low

Detected issues are returned as **flags**, not errors.

---

## 7. Query & Filtering Model

The backend supports **query-oriented access**:

* Filter by:

  * contract / symbol
  * start and end date
* Optionally:

  * resample bars (future enhancement)

Data access should be efficient and scoped to the request. Large datasets should not be fully loaded unless required.

Polars’ lazy scanning and column selection should be used where possible.

---

## 8. API-Oriented Output Philosophy

The backend’s API responses are intentionally shaped to:

* minimize transformation logic in the frontend
* closely match the input formats expected by TradingView Lightweight Charts

This means:

* candle bars are already `{time, open, high, low, close}`
* indicator series are already `{time, value}`
* trade markers are pre-classified (entry vs exit, long vs short)

---

## 9. Correctness as a First-Class Concern

The backend treats correctness issues as **data**, not failures.

Instead of rejecting misaligned data:

* it annotates trades with flags
* it exposes metadata to help the UI highlight issues visually

This design supports the project’s primary goal: **debugging and validation of backtests**.

---

## 10. Extensibility Considerations

The backend architecture allows future extensions without major redesign:

* additional indicators (new Parquet columns)
* additional trade metadata
* additional endpoints (equity curve, metrics)
* caching layers
* alternative data sources

None of these are required for the initial implementation.

---

## 11. Non-Goals (Backend)

The backend explicitly avoids:

* computing indicators dynamically
* modifying input data
* enforcing trading rules
* providing authentication or authorization
* supporting multi-user access

---

# `docs/API_Overview.md`

## 1. Purpose

This document defines the **HTTP API exposed by the backend data service** and the **data shapes returned**.

The API is designed to:

* serve chart-ready time-series data
* minimize frontend transformation logic
* align with TradingView Lightweight Charts’ expected input formats

All endpoints return **JSON** and are intended for **local use only**.

---

## 2. General Conventions

### 2.1 Time Representation

* All timestamps are:

  * UTC
  * UNIX epoch seconds
* Bars are sorted in ascending time order
* Indicator points share bar timestamps unless omitted

### 2.2 Numeric Values

* All numeric values are JSON numbers
* `NaN` values are omitted from series
* Missing data is represented by absence, not placeholders

---

## 3. Endpoints

### 3.1 Health Check

**GET** `/health`

```json
{ "status": "ok" }
```

Used to confirm backend availability.

---

### 3.2 Metadata

**GET** `/meta?contract=CLZ4_ohlcv1m`

Returns high-level information about the dataset.

```json
{
  "contract": "CLZ4_ohlcv1m",
  "barCount": 152340,
  "startTime": 1704067200,
  "endTime": 1706812800,
  "availableIndicators": [
    "vwap",
    "ema_9",
    "ema_14",
    "ema_21",
    "rsi_14_ema",
    "rsi_14_wilder",
    "atr_14"
  ],
  "tradeCount": 312
}
```

---

### 3.3 Bars (Candlesticks)

**GET** `/bars?contract=CLZ4_ohlcv1m&start=2024-10-01&end=2024-10-31`

Returns data shaped for a TradingView candlestick series.

```json
{
  "candles": [
    {
      "time": 1729771800,
      "open": 71.22,
      "high": 71.32,
      "low": 71.21,
      "close": 71.25
    },
    {
      "time": 1729772100,
      "open": 71.21,
      "high": 71.28,
      "low": 71.12,
      "close": 71.22
    }
  ]
}
```

**Frontend usage:**
Pass `candles` directly to `candlestickSeries.setData()`.

---

### 3.4 Indicator Series

**GET** `/series?contract=CLZ4_ohlcv1m&start=2024-10-01&end=2024-10-31`

Returns one or more indicator series.

```json
{
  "series": [
    {
      "id": "ema_21",
      "name": "EMA 21",
      "kind": "line",
      "pane": "price",
      "data": [
        { "time": 1729771800, "value": 71.41 },
        { "time": 1729772100, "value": 71.43 }
      ]
    },
    {
      "id": "rsi_14",
      "name": "RSI 14",
      "kind": "line",
      "pane": "rsi",
      "data": [
        { "time": 1729771800, "value": 44.67 },
        { "time": 1729772100, "value": 41.88 }
      ]
    }
  ]
}
```

**Pane semantics:**

* `"price"` → overlay on candlestick chart
* any other pane name → rendered in a sub-pane

**Frontend usage:**
Create one series per entry and call `setData()` with `data`.

---

### 3.5 Trades

**GET** `/trades?contract=CLZ4_ohlcv1m&start=2024-10-01&end=2024-10-31`

Returns both:

* high-level trade objects (for tables)
* chart markers (for rendering)

```json
{
  "trades": [
    {
      "id": "T-00123",
      "side": "long",
      "entryTime": 1729771950,
      "entryPrice": 71.24,
      "exitTime": 1729772400,
      "exitPrice": 71.85,
      "pnl": 0.61,
      "flags": []
    }
  ],
  "markers": [
    {
      "time": 1729771950,
      "position": "belowBar",
      "shape": "arrowUp",
      "color": "green",
      "text": "Entry"
    },
    {
      "time": 1729772400,
      "position": "aboveBar",
      "shape": "arrowDown",
      "color": "red",
      "text": "Exit"
    }
  ]
}
```

**Frontend usage:**
Pass `markers` to `series.setMarkers()`.

---

## 4. Error Handling

* API returns HTTP errors only for:

  * malformed requests
  * missing data
* Data issues (misalignment, implausible prices) are returned as **flags** inside responses

---

## 5. Contract Between Backend and Frontend

The frontend assumes:

* bars are sorted
* times are epoch seconds
* indicator series are already aligned
* markers are already classified

Any deviation from these rules must be handled in the backend.

---

If you want, the **next logical step** would be a very small but powerful doc:

**`Data_Contracts.md`** (1–2 pages) that simply restates:

* candle model
* indicator series model
* trade model
* invariants

That document becomes gold when you start letting AI agents implement slices without re-explaining assumptions.

If you’d like, I can generate that next — or we can move straight into a **repo bootstrap prompt** you can paste directly into Codex.
