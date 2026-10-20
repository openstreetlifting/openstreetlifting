import { defaultResource, resourceFromAttributes } from '@opentelemetry/resources';
import { NodeSDK } from '@opentelemetry/sdk-node';
import { logger } from '#lib/server/logger.js';
import { SafeTraceExporter } from '#lib/server/trace-exporter.js';

const runtime = globalThis as typeof globalThis & { oslTelemetry?: NodeSDK };

if (!runtime.oslTelemetry) {
  const exportEnabled = Boolean(
    process.env.OTEL_EXPORTER_OTLP_TRACES_ENDPOINT || process.env.OTEL_EXPORTER_OTLP_ENDPOINT
  );
  const sdk = new NodeSDK({
    serviceName: process.env.OTEL_SERVICE_NAME ?? 'osl-frontend',
    autoDetectResources: false,
    resource: defaultResource().merge(
      resourceFromAttributes({
        'service.version': process.env.PUBLIC_APP_VERSION ?? 'development',
        'deployment.environment.name': process.env.PUBLIC_ENVIRONMENT || 'development',
      })
    ),

    logRecordProcessors: [],
    metricReaders: [],
    ...(exportEnabled
      ? { traceExporter: new SafeTraceExporter() }
      : {

          spanProcessors: [
            { onStart() {}, onEnd() {}, async forceFlush() {}, async shutdown() {} },
          ],
        }),
  });
  sdk.start();
  process.on('uncaughtExceptionMonitor', (err) => {
    logger.fatal({ err }, 'Frontend process terminated');
  });
  runtime.oslTelemetry = sdk;

  process.once('sveltekit:shutdown', () => {
    void sdk.shutdown().catch((err: unknown) => {
      logger.error({ err }, 'Frontend trace exporter shutdown failed');
    });
  });
}
