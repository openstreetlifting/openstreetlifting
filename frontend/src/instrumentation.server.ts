import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-proto';
import { resourceFromAttributes } from '@opentelemetry/resources';
import { NodeSDK } from '@opentelemetry/sdk-node';

// Vite can reload this entry point. Register the SDK once per Node process.
const runtime = globalThis as typeof globalThis & { oslTelemetry?: NodeSDK };

if (!runtime.oslTelemetry) {
  const exportEnabled = Boolean(
    process.env.OTEL_EXPORTER_OTLP_TRACES_ENDPOINT || process.env.OTEL_EXPORTER_OTLP_ENDPOINT
  );
  const sdk = new NodeSDK({
    serviceName: process.env.OTEL_SERVICE_NAME ?? 'osl-frontend',
    resource: resourceFromAttributes({
      'service.version': process.env.PUBLIC_APP_VERSION ?? 'development',
      'deployment.environment.name': process.env.PUBLIC_ENVIRONMENT || 'development',
    }),
    // Pino writes logs to stdout; no second log-export pipeline is needed.
    logRecordProcessors: [],
    metricReaders: [],
    ...(exportEnabled
      ? { traceExporter: new OTLPTraceExporter() }
      : {
          // Still create real contexts locally, without contacting a collector.
          spanProcessors: [
            { onStart() {}, onEnd() {}, async forceFlush() {}, async shutdown() {} },
          ],
        }),
  });
  sdk.start();
  runtime.oslTelemetry = sdk;
  // adapter-node emits this after draining HTTP requests.
  process.once('sveltekit:shutdown', () => {
    void sdk.shutdown().catch((err: unknown) => {
      // Load the logger after instrumentation has initialized.
      void import('#lib/server/logger.js').then(({ logger }) => {
        logger.error({ err }, 'Trace export shutdown failed');
      });
    });
  });
}
