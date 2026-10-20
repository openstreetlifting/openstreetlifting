import '../../instrumentation.server';
import { context, trace, SpanKind } from '@opentelemetry/api';
import { afterEach, expect, it, vi } from 'vitest';
import { createLogger } from './logger';
import { apiClient } from './api/client';

vi.mock('#lib/server/config.js', () => ({ config: { apiUrl: 'http://backend.test' } }));

afterEach(() => vi.unstubAllGlobals());

it('keeps Pino error details and active trace context in separate fields', async () => {
  const lines: string[] = [];
  const logger = createLogger({
    write: (line) => {
      lines.push(line);
    },
  });
  await trace.getTracer('test').startActiveSpan('page', async (span) => {
    const expected = span.spanContext();
    logger.error({ err: new TypeError('unavailable'), attempt: 2 }, 'Page load failed');
    const log = JSON.parse(lines[0]);
    expect(lines[0]).not.toContain('unavailable');
    expect(log.trace_id).toBe(expected.traceId);
    expect(log.span_id).toBe(expected.spanId);
    expect(log.attributes).toMatchObject({
      'error.type': 'TypeError',

      attempt: 2,
    });
    expect(log).toMatchObject({
      message: 'Page load failed',
      severity_text: 'ERROR',
      severity_number: 17,
      logger: 'osl.frontend',
    });
    expect(log.timestamp).toMatch(/Z$/);
    span.end();
  });
});

it('propagates a distinct backend client span and preserves caller headers', async () => {
  const fetch = vi.fn().mockResolvedValue(Response.json({ ok: true }));
  vi.stubGlobal('fetch', fetch);
  await trace.getTracer('test').startActiveSpan('page', async (span) => {
    await apiClient.get('/api/v1/athletes/private-slug', {
      headers: new Headers({ 'x-custom': 'kept' }),
      params: { q: 'private search' },
    });
    const headers = fetch.mock.calls[0][1].headers as Headers;
    const parent = headers.get('traceparent')!.split('-');
    expect(parent[1]).toBe(span.spanContext().traceId);
    expect(parent[2]).not.toBe(span.spanContext().spanId);
    expect(headers.get('x-custom')).toBe('kept');
    expect(trace.getSpan(context.active())?.spanContext().spanId).toBe(span.spanContext().spanId);
    span.end();
  });
});

it('keeps parallel requests in their own traces', async () => {
  const traces: string[] = [];
  vi.stubGlobal(
    'fetch',
    vi.fn(async (_url, options: RequestInit) => {
      traces.push(new Headers(options.headers).get('traceparent')!.split('-')[1]);
      await Promise.resolve();
      return Response.json({ ok: true });
    })
  );
  const expected = await Promise.all(
    [1, 2].map(() =>
      trace.getTracer('test').startActiveSpan('page', { kind: SpanKind.SERVER }, async (span) => {
        await apiClient.get('/api/v1/rankings');
        span.end();
        return span.spanContext().traceId;
      })
    )
  );
  expect(new Set(traces).size).toBe(2);
  expect(traces.sort()).toEqual(expected.sort());
});
