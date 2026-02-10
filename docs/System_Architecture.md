# System Architecture

## 1. Overview

This project is a **local trade review and visualization system** designed to support the analysis and debugging of quantitative trading backtests.

The system ingests:

* Historical market data and precomputed technical indicators stored in **Parquet** format
* Backtest trade results stored in **CSV** format

It presents this data through an interactive charting user interface that allows users to visually inspect price action, indicators, and executed trades in a unified timeline.

The application is architected as **two cooperating components**:

1. A **Backend Data Service** responsible for data ingestion, normalization, and delivery
2. A **Frontend User Interface** responsible for visualization and user interaction

These components communicate over a local HTTP API and are developed together within a single repository.

---

## 2. Architectural Goals

The system is designed to meet the following goals:

* **Correctness first**
  Ensure that bars, indicators, and trades are aligned in time and price so that visual output can be trusted for debugging and validation.

* **Separation of concerns**
  Keep data access and transformation logic out of the UI, and visualization logic out of the backend.

* **Iterative development**
  Allow the frontend and backend to evolve together without premature API versioning or deployment complexity.

* **Local-first workflow**
  The system runs entirely on a developer’s machine and operates on local data files.

* **AI-assisted development**
  Provide clear boundaries and responsibilities so that AI agents can work effectively on isolated parts of the system or across vertical slices.

---

## 3. High-Level System Design

```
+---------------------+          HTTP / JSON          +------------------------+
|                     |  <------------------------>  |                        |
|   Frontend UI       |                               |   Backend Data Service |
|   (React + TS)      |                               |   (Rust + Axum)        |
|                     |                               |                        |
+---------------------+                               +------------------------+
                                                              |
                                                              |
                                                   Local File System
                                                              |
                                             Parquet (OHLCV + Indicators)
                                             CSV (Backtest Trades)
```

---

## 4. Backend Architecture

### 4.1 Responsibilities

The backend is a **local data service**, not a trading engine and not a visualization layer.

Its responsibilities are to:

* Load OHLCV and indicator data from Parquet files
* Load trade results from CSV files
* Normalize all data into canonical, frontend-friendly models
* Enforce consistent timestamp handling (UTC, epoch-based)
* Perform basic validation and alignment checks
* Serve data efficiently via HTTP endpoints

The backend **does not**:

* Execute strategies
* Modify data
* Perform UI-specific formatting
* Manage authentication or persistence beyond local files

---

### 4.2 Technology Stack

* **Language:** Rust
* **Web Framework:** `axum`
* **Data Processing:** `polars` (Parquet)
* **CSV Parsing:** `csv`
* **Serialization:** `serde`, `serde_json`

This stack is chosen for performance, strong typing, and compatibility with existing Rust-based backtesting infrastructure.

---

### 4.3 Data Ingestion Model

The backend treats Parquet and CSV files as **read-only sources of truth**.

#### Parquet Input (Market Data + Indicators)

The Parquet schema includes:

* Time series market data (OHLCV)
* Precomputed indicators
* Contract/symbol identifiers

Example columns:

```
timestamp
open, high, low, close, volume
vwap, ema_9, ema_14, ema_21
rsi_14_ema, rsi_14_wilder
atr_14
contract
```

The backend is responsible for:

* Converting timestamps into a single canonical representation (UTC epoch seconds)
* Treating each indicator column as a distinct logical time series
* Ensuring indicator values align correctly with their corresponding bars

**Design note:**
At this stage, the schema is documented for context only. Internal data structures should remain flexible to accommodate future schema changes.

---

#### CSV Input (Trades)

Trade CSV files are expected to contain backtest results such as:

* Entry and exit timestamps
* Prices
* Side (long/short)
* Quantity
* Optional metadata (PnL, tags, reason codes)

The backend maps CSV columns into a **canonical Trade model**, which the frontend can consume without knowledge of the original CSV structure.

---

### 4.4 Backend API Role

The backend exposes a **query-oriented API**, not a streaming or event-driven API.

Typical responsibilities of endpoints:

* Filter by symbol/contract
* Filter by date range
* Return only the data required for the current viewport
* Provide metadata and sanity-check information

The API is designed to support:

* Incremental rendering in the UI
* Fast iteration during development
* Strong coupling with the frontend (within the same repository)

---

## 5. Frontend Architecture

### 5.1 Responsibilities

The frontend is a **pure visualization and interaction layer**.

Its responsibilities are to:

* Request data from the backend
* Render candlestick charts
* Overlay indicators and trade markers
* Provide intuitive controls for filtering and navigation
* Enable trade-focused workflows (jump to trade, step through trades)

The frontend **does not**:

* Read Parquet or CSV files directly
* Perform financial calculations
* Perform data alignment or validation logic
* Persist or modify trading data

---

### 5.2 Technology Stack

* **Framework:** React
* **Language:** TypeScript
* **Charting Library:** TradingView Lightweight Charts

This combination provides:

* Strong typing for API contracts
* Excellent time-series chart performance
* A familiar UI development model

---

### 5.3 Charting Model

The frontend renders:

* **Primary price chart**

  * Candlestick bars (OHLCV)
  * Overlay indicators (EMA, VWAP, etc.)

* **Secondary indicator panes**

  * RSI, ATR, and similar oscillators

* **Trade annotations**

  * Entry and exit markers
  * Optional position-duration visualization
  * Highlighting for selected trades

Multiple charts or panes may be synchronized by timestamp to support crosshair movement and coordinated navigation.

---

### 5.4 User Interaction Flow

1. User selects:

   * Symbol / contract
   * Date range
   * Timeframe

2. Frontend requests:

   * Bars
   * Indicator series
   * Trades

3. UI renders:

   * Chart data
   * Overlays and markers

4. User interacts:

   * Clicking a trade highlights it on the chart
   * Navigating between trades updates the viewport
   * Filters adjust visible trades and indicators

---

## 6. Repository Structure Strategy

The project is maintained as a **single repository (monorepo)** with clear internal boundaries:

```
trade-review-ui/
  backend/
  frontend/
  docs/
    System_Architecture.md
    Data_Contracts.md
    API_Overview.md
```

This approach:

* Allows the frontend and backend to evolve together
* Simplifies AI-assisted development
* Avoids premature versioning and deployment complexity

Despite sharing a repository, the frontend and backend are treated as **logically independent applications**.

---

## 7. Development & AI Agent Considerations

This architecture is intentionally designed to support AI-driven development by:

* Defining clear ownership of responsibilities
* Providing explicit data boundaries
* Encouraging vertical feature slices that touch both frontend and backend
* Minimizing implicit assumptions between components

AI agents should be assigned work in terms of:

* Backend endpoints + normalization logic
* Frontend features consuming existing endpoints
* End-to-end vertical slices (e.g., “render trades on chart”)

---

## 8. Future Evolution (Out of Scope for v1)

Potential future enhancements include:

* Multi-symbol comparison
* Equity curve visualization
* Trade replay / bar-by-bar stepping
* Exportable reports
* Packaging as a desktop application (Tauri/Electron)

These are explicitly **not required** for the initial implementation and should not influence early design decisions.

---

If you want, the next *very natural* document to write (and very helpful for an AI agent) would be:

* **`Data_Contracts.md`** – canonical JSON models and invariants
* **`API_Overview.md`** – endpoint list + example responses

Those two, together with this architecture doc, essentially give an AI agent everything it needs to start building with confidence.
