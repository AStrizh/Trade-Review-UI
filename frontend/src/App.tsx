import { useEffect, useMemo, useState } from 'react';

import ChartView from './components/ChartView';
import { fetchBars, fetchHealth, type Candle } from './api/client';

type StatusState =
  | { type: 'loading' }
  | { type: 'ok' }
  | { type: 'error'; message: string };

type BarsState =
  | { type: 'loading' }
  | { type: 'ready'; candles: Candle[] }
  | { type: 'error'; message: string };

/** Defines the Milestone 1 default query so the app can load with no extra input controls yet. */
const DEFAULT_BARS_QUERY = {
  contract: 'DEMO_CONTRACT',
  start: '2024-10-24',
  end: '2024-10-24',
};

/** Coordinates backend health checks and initial bars loading for Milestone 1. */
export default function App() {
  const [status, setStatus] = useState<StatusState>({ type: 'loading' });
  const [bars, setBars] = useState<BarsState>({ type: 'loading' });

  useEffect(() => {
    fetchHealth()
      .then((health) => {
        if (health.status !== 'ok') {
          setStatus({ type: 'error', message: `Unexpected status: ${health.status}` });
          return;
        }

        setStatus({ type: 'ok' });
        return fetchBars(DEFAULT_BARS_QUERY)
          .then((response) => {
            setBars({ type: 'ready', candles: response.candles });
          })
          .catch((error: unknown) => {
            const message = error instanceof Error ? error.message : 'Unknown bars error';
            setBars({ type: 'error', message });
          });
      })
      .catch((error: unknown) => {
        const message = error instanceof Error ? error.message : 'Unknown health error';
        setStatus({ type: 'error', message });
      });
  }, []);

  const statusText = useMemo(() => {
    if (status.type === 'loading') {
      return 'Checking backend health...';
    }

    if (status.type === 'error') {
      return `Backend error: ${status.message}`;
    }

    if (bars.type === 'loading') {
      return 'Loading bars...';
    }

    if (bars.type === 'error') {
      return `Bars error: ${bars.message}`;
    }

    return `Loaded ${bars.candles.length} candles for ${DEFAULT_BARS_QUERY.contract}`;
  }, [bars, status]);

  return (
    <main style={{ margin: '0 auto', maxWidth: 1024, padding: '1rem' }}>
      <h1>Trade Review UI — Milestone 1</h1>
      <p>{statusText}</p>
      {status.type === 'ok' && bars.type === 'ready' ? <ChartView candles={bars.candles} /> : null}
    </main>
  );
}
