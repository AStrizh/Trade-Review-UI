import { useEffect, useRef } from 'react';
import {
  CandlestickSeries,
  LineSeries,
  createChart,
  type IChartApi,
  type ISeriesApi,
  type UTCTimestamp,
} from 'lightweight-charts';

import type { Candle, IndicatorSeries } from '../api/client';

type ChartViewProps = {
  candles: Candle[];
  series: IndicatorSeries[];
};

/** Renders a responsive candlestick chart with overlay indicator lines. */
export default function ChartView({ candles, series }: ChartViewProps) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const chartRef = useRef<IChartApi | null>(null);
  const candleSeriesRef = useRef<ISeriesApi<'Candlestick'> | null>(null);
  const indicatorRefs = useRef<Map<string, ISeriesApi<'Line'>>>(new Map());

  useEffect(() => {
    const container = containerRef.current;
    if (!container || chartRef.current) {
      return;
    }

    const chart = createChart(container, {
      autoSize: true,
      layout: {
        background: { color: '#0f172a' },
        textColor: '#e2e8f0',
      },
      grid: {
        vertLines: { color: '#334155' },
        horzLines: { color: '#334155' },
      },
      rightPriceScale: {
        borderColor: '#334155',
      },
      timeScale: {
        borderColor: '#334155',
      },
    });

    const candleSeries = chart.addSeries(CandlestickSeries, {
      upColor: '#16a34a',
      downColor: '#dc2626',
      borderVisible: false,
      wickUpColor: '#22c55e',
      wickDownColor: '#ef4444',
    });

    chartRef.current = chart;
    candleSeriesRef.current = candleSeries;

    return () => {
      chart.remove();
      chartRef.current = null;
      candleSeriesRef.current = null;
      indicatorRefs.current.clear();
    };
  }, []);

  useEffect(() => {
    if (!candleSeriesRef.current) {
      return;
    }

    candleSeriesRef.current.setData(
      candles.map((candle) => ({
        time: candle.time as UTCTimestamp,
        open: candle.open,
        high: candle.high,
        low: candle.low,
        close: candle.close,
      })),
    );
  }, [candles]);

  useEffect(() => {
    const chart = chartRef.current;
    if (!chart) {
      return;
    }

    const overlaySeries = series.filter((item) => item.pane === 'price');
    const seen = new Set<string>();

    for (const indicator of overlaySeries) {
      seen.add(indicator.id);
      let lineSeries = indicatorRefs.current.get(indicator.id);
      if (!lineSeries) {
        lineSeries = chart.addSeries(LineSeries, {
          lineWidth: 2,
          priceLineVisible: false,
          lastValueVisible: false,
        });
        indicatorRefs.current.set(indicator.id, lineSeries);
      }

      lineSeries.setData(
        indicator.data.map((point) => ({
          time: point.time as UTCTimestamp,
          value: point.value,
        })),
      );
    }

    for (const [id, line] of indicatorRefs.current.entries()) {
      if (!seen.has(id)) {
        chart.removeSeries(line);
        indicatorRefs.current.delete(id);
      }
    }

    chart.timeScale().fitContent();
  }, [series]);

  return <div ref={containerRef} style={{ height: 420, width: '100%' }} />;
}
