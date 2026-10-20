import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-proto';
import type { Attributes } from '@opentelemetry/api';

type Spans = Parameters<OTLPTraceExporter['export']>[0];
type ExportCallback = Parameters<OTLPTraceExporter['export']>[1];

const safeKeys = new Set([
  'http.route',
  'http.request.method',
  'http.response.status_code',
  'server.address',
  'error.type',
  'sveltekit.is_data_request',
  'sveltekit.is_sub_request',
]);

function safeAttributes(attributes: Attributes): Attributes {
  return Object.fromEntries(
    Object.entries(attributes).filter(
      ([key]) => safeKeys.has(key) || key.endsWith('.result.type') || key.endsWith('.result.status')
    )
  );
}

// SvelteKit adds full URLs and exception messages to its built-in spans. Send
// a sanitized copy so those values never leave the application via OTLP.
export function sanitizeSpans(spans: Spans): Spans {
  return spans.map((span) => ({
    name: span.name,
    kind: span.kind,
    spanContext: () => span.spanContext(),
    parentSpanContext: span.parentSpanContext,
    startTime: span.startTime,
    endTime: span.endTime,
    duration: span.duration,
    ended: span.ended,
    status: { code: span.status.code },
    attributes: safeAttributes(span.attributes),
    events: span.events.map((event) => ({
      ...event,
      attributes: safeAttributes(event.attributes ?? {}),
    })),
    links: span.links.map((link) => ({ ...link, attributes: {} })),
    resource: span.resource,
    instrumentationScope: span.instrumentationScope,
    droppedAttributesCount: span.droppedAttributesCount,
    droppedEventsCount: span.droppedEventsCount,
    droppedLinksCount: span.droppedLinksCount,
  }));
}

export class SafeTraceExporter extends OTLPTraceExporter {
  override export(spans: Spans, callback: ExportCallback): void {
    super.export(sanitizeSpans(spans), callback);
  }
}
