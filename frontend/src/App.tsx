import { useEffect, useMemo, useState } from 'react';

import ChartView from './components/ChartView';
import { fetchBars, fetchHealth, fetchSeries, type Candle, type IndicatorSeries } from './api/client';

type StatusState =
  | { type: 'loading' }
  | { type: 'ok' }
  | { type: 'error'; message: string };

type BarsState =
  | { type: 'loading' }
  | { type: 'ready'; candles: Candle[] }
  | { type: 'error'; message: string };

type SeriesState =
  | { type: 'loading' }
  | { type: 'ready'; series: IndicatorSeries[] }
  | { type: 'error'; message: string };

/** Milestone 2 default query reads from parquet and allows all contracts by default. */
const DEFAULT_QUERY = {};

/** Coordinates backend health checks and parquet-backed bars + indicator loading. */
export default function App() {
  const [status, setStatus] = useState<StatusState>({ type: 'loading' });
  const [bars, setBars] = useState<BarsState>({ type: 'loading' });
  const [series, setSeries] = useState<SeriesState>({ type: 'loading' });

  useEffect(() => {
    fetchHealth()
      .then((health) => {
        if (health.status !== 'ok') {
          setStatus({ type: 'error', message: `Unexpected status: ${health.status}` });
          return;
        }

        setStatus({ type: 'ok' });

        fetchBars(DEFAULT_QUERY)
          .then((response) => setBars({ type: 'ready', candles: response.candles }))
          .catch((error: unknown) => {
            const message = error instanceof Error ? error.message : 'Unknown bars error';
            setBars({ type: 'error', message });
          });

        fetchSeries(DEFAULT_QUERY)
          .then((response) => setSeries({ type: 'ready', series: response.series }))
          .catch((error: unknown) => {
            const message = error instanceof Error ? error.message : 'Unknown series error';
            setSeries({ type: 'error', message });
          });
      })
      .catch((error: unknown) => {
        const message = error instanceof Error ? error.message : 'Unknown health error';
        setStatus({ type: 'error', message });
      });
  }, []);

  const statusText = useMemo(() => {
    if (status.type === 'loading') return 'Checking backend health...';
    if (status.type === 'error') return `Backend error: ${status.message}`;
    if (bars.type === 'loading') return 'Loading bars...';
    if (bars.type === 'error') return `Bars error: ${bars.message}`;
    if (series.type === 'loading') return 'Loading indicator series...';
    if (series.type === 'error') return `Series error: ${series.message}`;

    return `Loaded ${bars.candles.length} candles and ${series.series.length} indicator series from parquet`;
  }, [bars, series, status]);

  return (
    <main style={{ margin: '0 auto', maxWidth: 1024, padding: '1rem' }}>
      <h1>Trade Review UI — Milestone 2</h1>
      <p>{statusText}</p>
      {status.type === 'ok' && bars.type === 'ready' && series.type === 'ready' ? (
        <ChartView candles={bars.candles} series={series.series} />
      ) : null}
    </main>
  );
}
