import { useEffect, useState } from 'react';

import { fetchHealth } from './api/client';

type StatusState =
  | { type: 'loading' }
  | { type: 'ok' }
  | { type: 'error'; message: string };

export default function App() {
  const [status, setStatus] = useState<StatusState>({ type: 'loading' });

  useEffect(() => {
    fetchHealth()
      .then((health) => {
        if (health.status === 'ok') {
          setStatus({ type: 'ok' });
        } else {
          setStatus({ type: 'error', message: `Unexpected status: ${health.status}` });
        }
      })
      .catch((error: unknown) => {
        const message = error instanceof Error ? error.message : 'Unknown error';
        setStatus({ type: 'error', message });
      });
  }, []);

  if (status.type === 'loading') {
    return <p>Checking backend health...</p>;
  }

  if (status.type === 'error') {
    return <p>Backend error: {status.message}</p>;
  }

  return <p>Backend: OK</p>;
}
