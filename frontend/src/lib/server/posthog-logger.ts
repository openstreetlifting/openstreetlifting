import type { Logger } from '@opentelemetry/api-logs';
import { OTLPLogExporter } from '@opentelemetry/exporter-logs-otlp-http';
import { resourceFromAttributes } from '@opentelemetry/resources';
import { BatchLogRecordProcessor, LoggerProvider } from '@opentelemetry/sdk-logs';

let provider: LoggerProvider | undefined;
let posthogLogger: Logger | undefined;

type LogAttributes = Record<string, boolean | number | string>;

function warnMissingConfiguration(variable: string) {
  if (process.env.NODE_ENV !== 'production') {
    console.warn(
      `${variable} variable required by PostHog is missing or un-configured, this causes events to be silently missed. This error stops appearing once ${variable} is configured`
    );
  }
}

export function configurePostHogLogger() {
  if (provider) return;

  const token = process.env.PUBLIC_POSTHOG_PROJECT_TOKEN;
  const host = process.env.PUBLIC_POSTHOG_HOST;
  if (!token) {
    warnMissingConfiguration('PUBLIC_POSTHOG_PROJECT_TOKEN');
    return;
  }
  if (!host) {
    warnMissingConfiguration('PUBLIC_POSTHOG_HOST');
    return;
  }

  provider = new LoggerProvider({
    resource: resourceFromAttributes({
      'service.name': process.env.OTEL_SERVICE_NAME ?? 'osl-frontend',
      'service.version': process.env.PUBLIC_APP_VERSION ?? 'development',
      'deployment.environment.name': process.env.PUBLIC_ENVIRONMENT || 'development',
    }),
    processors: [
      new BatchLogRecordProcessor({
        exporter: new OTLPLogExporter({
          url: new URL('/i/v1/logs', host).toString(),
          headers: { Authorization: `Bearer ${token}` },
          timeoutMillis: 2_000,
        }),
        exportTimeoutMillis: 2_000,
      }),
    ],
  });
  posthogLogger = provider.getLogger('osl.frontend.posthog');
}

export function emitPostHogLog(
  severityText: 'ERROR' | 'INFO',
  body: string,
  attributes: LogAttributes
) {
  posthogLogger?.emit({ severityText, body, attributes });
}

export async function shutdownPostHogLogger() {
  if (!provider) return;

  const shutdown = provider.shutdown().catch(() => {});
  await Promise.race([shutdown, new Promise((resolve) => setTimeout(resolve, 2_000))]);
}
