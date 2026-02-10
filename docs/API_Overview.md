# API Overview

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
