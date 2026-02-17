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

export type IndicatorPoint = {
  time: number;
  value: number;
};

export type IndicatorSeries = {
  id: string;
  name: string;
  kind: string;
  pane: string;
  data: IndicatorPoint[];
};

export type BarsResponse = {
  candles: Candle[];
};

export type SeriesResponse = {
  series: IndicatorSeries[];
};

export type BarsQuery = {
  contract?: string;
  start?: string;
  end?: string;
};

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? 'http://localhost:8080';

function buildParams(query: BarsQuery): URLSearchParams {
  const params = new URLSearchParams();
  if (query.contract) params.set('contract', query.contract);
  if (query.start) params.set('start', query.start);
  if (query.end) params.set('end', query.end);
  return params;
}

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

/** Fetches chart candles for an optional contract and inclusive date range. */
export async function fetchBars(query: BarsQuery): Promise<BarsResponse> {
  const response = await fetch(`${API_BASE_URL}/bars?${buildParams(query).toString()}`, {
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

/** Fetches indicator series from parquet-backed columns. */
export async function fetchSeries(query: BarsQuery): Promise<SeriesResponse> {
  const response = await fetch(`${API_BASE_URL}/series?${buildParams(query).toString()}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    throw new Error(`Series request failed with ${response.status} ${response.statusText}`);
  }

  return (await response.json()) as SeriesResponse;
}
