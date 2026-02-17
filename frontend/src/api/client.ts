export type HealthResponse = {
  status: string;
};

export type Candle = {
  time: number;
  open: number;
  high: number;
  low: number;
  close: number;
};

export type BarsResponse = {
  candles: Candle[];
};

export type BarsQuery = {
  contract: string;
  start: string;
  end: string;
};

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? 'http://localhost:8080';

/** Fetches backend health so the UI can fail fast when the API is unavailable. */
export async function fetchHealth(): Promise<HealthResponse> {
  const response = await fetch(`${API_BASE_URL}/health`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    throw new Error(`Backend returned ${response.status} ${response.statusText}`);
  }

  return (await response.json()) as HealthResponse;
}

/** Fetches chart candles for a contract and inclusive date range. */
export async function fetchBars(query: BarsQuery): Promise<BarsResponse> {
  const params = new URLSearchParams({
    contract: query.contract,
    start: query.start,
    end: query.end,
  });

  const response = await fetch(`${API_BASE_URL}/bars?${params.toString()}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    throw new Error(`Bars request failed with ${response.status} ${response.statusText}`);
  }

  return (await response.json()) as BarsResponse;
}
