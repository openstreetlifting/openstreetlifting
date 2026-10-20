import { context, isSpanContextValid, trace } from '@opentelemetry/api';
import pino from 'pino';

const severityNumbers: Record<string, number> = {
  trace: 1,
  debug: 5,
  info: 9,
  warn: 13,
  error: 17,
  fatal: 21,
};

export function createLogger(destination: pino.DestinationStream = process.stdout) {
  return pino(
    {
      level: process.env.LOG_LEVEL ?? 'info',
      messageKey: 'message',
      timestamp: () => `,"timestamp":"${new Date().toISOString()}"`,
      base: {
        logger: 'osl.frontend',
        resource: {
          'service.name': process.env.OTEL_SERVICE_NAME ?? 'osl-frontend',
          'service.version': process.env.PUBLIC_APP_VERSION ?? 'development',
          'deployment.environment.name': process.env.PUBLIC_ENVIRONMENT || 'development',
        },
      },
      formatters: {
        level: (label, level) => ({
          level,
          severity_text: label.toUpperCase(),
          severity_number: severityNumbers[label],
        }),
        log: ({ err, trace_id, span_id, trace_flags, ...attributes }) => ({
          trace_id,
          span_id,
          trace_flags,
          attributes: {
            ...attributes,
            ...(err instanceof Error
              ? {
                  'error.type': err.name,
                  'exception.message': err.message,
                  'exception.stacktrace': err.stack,
                }
              : err === undefined
                ? {}
                : { 'exception.message': String(err) }),
          },
        }),
      },
      mixin() {
        const active = trace.getSpan(context.active())?.spanContext();
        return active && isSpanContextValid(active)
          ? { trace_id: active.traceId, span_id: active.spanId, trace_flags: active.traceFlags }
          : {};
      },
    },
    destination
  );
}

export const logger = createLogger();
