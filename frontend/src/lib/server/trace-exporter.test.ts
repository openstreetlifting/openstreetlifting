import { expect, it } from 'vitest';
import { defaultResource } from '@opentelemetry/resources';
import { SpanKind, SpanStatusCode } from '@opentelemetry/api';
import { sanitizeSpans } from './trace-exporter';

it('removes framework URLs and exception details without mutating the original span', () => {
  const span: Parameters<typeof sanitizeSpans>[0][number] = {
    name: 'sveltekit.handle.root',
    kind: SpanKind.SERVER,
    spanContext: () => ({ traceId: '1'.repeat(32), spanId: '2'.repeat(16), traceFlags: 1 }),
    startTime: [0, 0],
    endTime: [1, 0],
    duration: [1, 0],
    ended: true,
    status: { code: SpanStatusCode.ERROR, message: 'private@example.com' },
    attributes: {
      'http.url': 'https://example.com/athletes/private-name?q=private@example.com',
      'http.route': '/athletes/[slug]',
      'sveltekit.load.result.message': 'private@example.com',
    },
    events: [
      {
        name: 'exception',
        time: [0, 0],
        attributes: { 'exception.message': 'private@example.com' },
      },
    ],
    links: [],
    resource: defaultResource(),
    instrumentationScope: { name: 'sveltekit' },
    droppedAttributesCount: 0,
    droppedEventsCount: 0,
    droppedLinksCount: 0,
  };
  const [safe] = sanitizeSpans([span]);
  expect(JSON.stringify(safe)).not.toContain('private');
  expect(safe.attributes['http.route']).toBe('/athletes/[slug]');
  expect(safe.spanContext()).toEqual(span.spanContext());
  expect(span.status.message).toBe('private@example.com');
});
