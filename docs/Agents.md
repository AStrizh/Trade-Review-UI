# Project Spec: Backtest Trade Review UI (Local)

## 1. Purpose

Create a local application that loads:

* **Market history bars** from **Parquet**
* **Precomputed indicators** from **Parquet**
* **Backtest trades** from a **CSV**

…and visualizes them on an interactive chart to **audit/debug backtests** (verify timestamps, indicator alignment, trade placement, and logic correctness).

Primary goal: **trade review and correctness**, not publishing charts or live trading.

## 2. Target User & Use Cases

**User:** A developer/quant (Alex) building a backtesting system and wanting a fast way to visually verify trades.

### Core Use Cases

1. Load a symbol + date range → see candlestick chart.
2. Overlay indicators (VWAP/EMA/RSI/etc.).
3. Load trade CSV → see entry/exit markers and position windows.
4. Click trade in a list → chart jumps to entry time and highlights the trade.
5. Validate alignment: trade timestamp lands on/near expected bar, entry price is plausible within bar range.

### Nice-to-have

* Step through trades sequentially (“Next trade”, “Previous trade”)
* Filter trades (long/short, winners/losers, tag/reason, date)
* Basic stats panel (count, win rate, avg R, total PnL)

## 3. Non-Goals

* No brokerage connectivity.
* No strategy execution inside the UI (for now).
* No complex drawing tools (trendlines, fibs, etc.).
* No multi-user auth.

## 4. Technical Approach (Recommended)

**Two-process local app:**

* **Backend service** reads Parquet/CSV and serves normalized JSON to the UI.
* **Frontend UI** renders charts using **TradingView Lightweight Charts**.

Reason: browsers + Parquet is possible but adds complexity and performance issues; a local backend keeps it clean and scalable.

### Suggested Stack

* Backend: **Rust** (`axum`) + `polars` (Parquet) + `csv` + `serde`
* Frontend: **React + TypeScript** + `lightweight-charts`
* Dev: run both locally; later optionally bundle with Tauri/Electron

## 5. Data Contracts (Canonical Models)

The backend MUST normalize all inputs into these canonical structures.

### 5.1 Bars

```json
Bar {
  "t": 1704067200,        // epoch seconds (UTC)
  "o": 100.12,
  "h": 101.00,
  "l": 99.50,
  "c": 100.80,
  "v": 12345
}
```

* Time is **UTC epoch seconds**.
* Define the meaning of `t`: **bar open time** (default). If your dataset uses close time, backend must convert or document it consistently.

### 5.2 Indicator Series

Two types:

* **overlay** (on price chart): EMA, VWAP
* **pane** (separate): RSI, ATR

```json
Series {
  "id": "vwap",
  "name": "VWAP",
  "kind": "line",                 // line | histogram
  "placement": "overlay",         // overlay | pane
  "paneId": "rsi",                // only if placement=pane
  "points": [{"t":1704067200,"y":100.55}, ...]
}
```

### 5.3 Trades (Minimum)

Trades can be stored as “round trips” (entry + exit) for v1.

```json
Trade {
  "id": "T-000123",
  "symbol": "MCLU25",
  "side": "long",                 // long | short
  "qty": 1,
  "entryTime": 1704067500,
  "entryPrice": 100.25,
  "exitTime": 1704068400,
  "exitPrice": 101.10,
  "pnl": 0.85,                    // optional if known
  "tags": ["RSI-30-cross", "tp"],  // optional
  "notes": "optional"
}
```

**Trade CSV mapping rules:**

* The backend must provide a mapping adapter that can map common CSV columns to the above fields.
* If CSV has partial fills or multiple exits: out of scope for v1; backend may collapse to one round-trip or skip.

## 6. Backend API

All endpoints return JSON. Backend reads from configured local paths.

### 6.1 Configuration

Backend reads a config file or env vars:

* `DATA_DIR` root
* `BARS_PARQUET_PATH` (or pattern)
* `INDICATORS_PARQUET_PATH` (or pattern)
* `TRADES_CSV_PATH`
* `TIMEZONE` default UTC for storage

### 6.2 Endpoints

1. Health

* `GET /health` → `{ "status": "ok" }`

2. Symbols (optional if you have multiple)

* `GET /symbols` → `["MCLU25","MNQH26",...]`

3. Dataset metadata / sanity info

* `GET /meta?symbol=...` → bar range, counts, available indicators, trade count, etc.

4. Bars

* `GET /bars?symbol=...&start=YYYY-MM-DD&end=YYYY-MM-DD&tf=1m`
* returns `Bar[]`
* If `tf != base`, backend resamples.

5. Indicators

* `GET /series?symbol=...&start=...&end=...`
* returns `Series[]` (bars-aligned points)

6. Trades

* `GET /trades?symbol=...&start=...&end=...`
* returns `Trade[]`

### Performance requirement

Backend MUST support viewport usage:

* it should be able to efficiently return data for a subset range without loading entire history into RAM each request.
* acceptable for v1 to load per-request but must be cleanly coded with the ability to optimize.

## 7. Frontend Requirements

### 7.1 Layout

Single-page UI with:

* **Top bar**: symbol selector, date range, timeframe selector
* **Main chart**: candlesticks
* **Indicator toggles** panel (checkbox list)
* **Trades table** below or side panel

### 7.2 Chart Behavior

* Candlestick chart using TradingView Lightweight Charts.
* Overlay indicator series on main chart.
* Pane indicators show in separate panels (can be implemented as separate charts synced by time).
* Trades shown as:

  * entry markers (arrow up/down)
  * exit markers (opposite)
  * optional line between entry and exit or shaded “position window”

### 7.3 Interaction

* Clicking a trade row:

  * scroll/zoom chart to around `entryTime`
  * highlight the trade markers/region
* Filter controls:

  * long/short toggle
  * winners/losers toggle (if pnl available)
  * tag filter (optional)

## 8. Data Alignment & Correctness Rules

These are critical and must be tested.

1. **Time normalization**

* All times in UI are rendered local to user, but internal model uses UTC epoch seconds.

2. **Bar vs trade timestamps**

* For each trade, backend must compute a “nearest bar” check:

  * if `abs(tradeTime - nearestBarTime) > maxSkewSeconds` then flag the trade in the response or meta.
  * `maxSkewSeconds` default: half the timeframe duration.

3. **Price plausibility**

* If `entryPrice` is outside `[bar.low - epsilon, bar.high + epsilon]`, flag.

Flags can show as:

* `trade.flags = ["TIME_SKEW", "PRICE_OUT_OF_RANGE"]`

## 9. Project Milestones (Agent Execution Plan)

### Milestone 0 — Repo scaffold

Deliverables:

* Backend skeleton (`/backend`)
* Frontend skeleton (`/frontend`)
* Root README with run instructions
  Acceptance:
* `GET /health` returns ok
* frontend starts and shows placeholder UI

### Milestone 1 — Bars rendering

Deliverables:

* Backend can serve `/bars` from Parquet for a symbol/range
* Frontend renders candlesticks
  Acceptance:
* Chart displays correct bars for selected range

### Milestone 2 — Trades overlay

Deliverables:

* Backend loads CSV and serves `/trades`
* Frontend renders entry/exit markers
* Clicking a trade jumps to it
  Acceptance:
* You can visually confirm trades appear at the correct time/price

### Milestone 3 — Indicators

Deliverables:

* Backend serves `/series` for precomputed indicators
* Frontend overlays selected indicators
* Separate pane for RSI-like indicators
  Acceptance:
* Indicators align with bars (no off-by-one)

### Milestone 4 — Filters + sanity panel

Deliverables:

* Trade filters, meta sanity checks displayed
  Acceptance:
* Bad alignment is highlighted and easy to spot

## 10. Testing Requirements

Backend:

* Unit tests for:

  * timestamp conversion
  * CSV mapping
  * bar resampling correctness
  * alignment checks (time skew / price range)

Frontend:

* Minimal smoke test: can load bars and render chart without runtime errors.

## 11. Repo Structure (Proposed)

```
trade-review-ui/
  README.md
  backend/
    Cargo.toml
    src/
      main.rs
      config.rs
      models.rs
      routes.rs
      data/
        bars.rs
        indicators.rs
        trades.rs
      align/
        checks.rs
      resample/
        ohlcv.rs
    tests/
  frontend/
    package.json
    src/
      api/
      components/
        ChartView.tsx
        TradesTable.tsx
        ControlsBar.tsx
        IndicatorsPanel.tsx
      pages/
        App.tsx
```

## 12. “Definition of Done”

The project is done (v1) when:

* You can pick symbol + date range → see bars.
* Trades load from CSV → show entry/exit.
* At least 2 indicators display (one overlay like VWAP/EMA and one pane like RSI).
* Clicking a trade focuses the chart on it.
* Misalignment (time skew or price outside bar range) is flagged and visible.

---

## 12. Commit and Comment Requirement

Produce SMALL, REVIEWABLE COMMITS. Do not dump everything into one commit.

Leave a comment for every method, class, and data object that clearly explains what it does and why it is necessary.

Logically space out work items so that they may be easily reviewed.

If a method, function, variable, etc. is no longer needed, delete it. Do not commit dead code into the repo.
