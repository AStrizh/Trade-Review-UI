# Trade Review UI

> **Status:** Early development (Milestone 1 in progress)

## Overview

**Trade Review UI** is a local, developer-focused application for visually reviewing and validating quantitative trading backtests.

The project is designed to:

* load historical market data and precomputed indicators
* load backtest trade results
* visualize price action, indicators, and trades on an interactive chart
* help identify alignment issues, logic errors, and unexpected behavior in backtesting systems

This tool is intentionally **local-first**, **read-only**, and optimized for **debugging and inspection**, not live trading or signal generation.

---

### 📊 Charting Library Attribution

This project uses **TradingView Lightweight Charts**,  
an open-source charting library for displaying financial time series data.
  
**Lightweight Charts™ is licensed under the Apache License, Version 2.0**,  
and requires attribution to TradingView as the product creator. ([GitHub][1])  

TradingView Lightweight Charts™  
Copyright (с) 2025 TradingView, Inc. https://www.tradingview.com/  


To Learn more about this library visit the folowing link: [TradingView Lightweight Charts][2]  


[1]: https://github.com/tradingview/lightweight-charts/blob/master/LICENSE?utm_source=chatgpt.com "license - tradingview/lightweight-charts"
[2]: https://tradingview.github.io/lightweight-charts/docs?utm_source=chatgpt.com "Getting started | Lightweight Charts"

---

## Development Approach

This project is being developed with the assistance of **OpenAI tools**, including **ChatGPT** and **Codex-style AI agents**.

AI is used to:

* generate architecture and design documentation
* scaffold code and infrastructure
* implement small, reviewable development milestones

All generated code is reviewed, understood, and iterated on manually.
The goal is **high code quality, correctness, and clarity**, not rapid, opaque generation.

---

## Current Status (Milestone 0)

Milestone 0 establishes the **project foundation**:

* A monorepo structure containing:

  * a Rust backend service
  * a React + TypeScript frontend
* A running backend with a `/health` endpoint
* A frontend that successfully communicates with the backend
* Development-time CORS configuration
* Core project documentation

At this stage:

* A Milestone 1 candlestick prototype is available using backend-served demo bars
* Parquet-backed ingestion is still pending
* Trade and indicator overlays are not implemented yet

This milestone exists solely to ensure that:

* the repo structure is sound
* tooling works end-to-end
* future development can proceed in small, reviewable steps

---

## Architecture Summary

The system is intentionally split into two cooperating applications:

### Backend (Data Service)

* **Language:** Rust
* **Framework:** Axum
* **Role:**

  * Load and normalize market data and trades
  * Perform validation and alignment checks
  * Serve chart-ready JSON to the frontend

### Frontend (User Interface)

* **Framework:** React
* **Language:** TypeScript
* **Role:**

  * Request data from the backend
  * Render interactive charts
  * Provide trade-focused review workflows

The two applications communicate over a **local HTTP API**.

Full details are available in the `docs/` directory.

---

## Repository Structure

```
trade-review-ui/
├── README.md
├── docs/
│   ├── System_Architecture.md
│   ├── Backend_Architecture.md
│   ├── API_Overview.md
│   └── Data_Contracts.md
├── backend/
│   └── (Rust Axum service)
└── frontend/
    └── (React + TypeScript app)
```

Each directory is intentionally scoped and documented to support AI-assisted and human development.

---

## How to Run (Development)

### Prerequisites

* **Rust** (stable toolchain)
* **Node.js** (18+ recommended)
* **npm** or **pnpm**

---

### Backend

```bash
cd backend
cargo run
```

The backend will start on:

```
http://localhost:8080
```

Health check:

```
http://localhost:8080/health
```

---

### Frontend

```bash
cd frontend
npm install
npm run dev
```

The frontend will start on:

```
http://localhost:5173
```

On load, the frontend will call the backend `/health` endpoint and display the result.

---

## Documentation

The `docs/` directory contains the authoritative design documents for this project:

* **System_Architecture.md**
  High-level system overview and design goals

* **Backend_Architecture.md**
  Technical details of the Rust backend, data ingestion strategy, and responsibilities

* **API_Overview.md**
  HTTP endpoints and example responses

* **Data_Contracts.md**
  Canonical data models and invariants shared between backend and frontend

These documents are considered **source of truth** and should be updated alongside code changes.

---

## Development Roadmap

Planned milestones (subject to iteration):

* **Milestone 1:**
  Render candlestick charts using mock or real bar data

* **Milestone 2:**
  Ingest OHLCV and indicator data from Parquet files

* **Milestone 3:**
  Load backtest trades from CSV and render entry/exit markers

* **Milestone 4:**
  Indicator panes (EMA, VWAP overlays; RSI/ATR subcharts)

* **Milestone 5:**
  Trade navigation, filtering, and validation highlighting

Each milestone is intentionally scoped to allow for careful code review and validation.

---

## Non-Goals

This project does **not** aim to:

* perform live trading
* compute indicators dynamically (initially)
* replace professional charting platforms
* act as a multi-user or hosted service

It exists to support **transparent, inspectable backtest analysis**.

---

## License

Apache 2.0

---
