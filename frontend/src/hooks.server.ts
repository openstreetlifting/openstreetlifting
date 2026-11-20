import { SpanStatusCode, isSpanContextValid } from '@opentelemetry/api';
import type { Handle, HandleServerError } from '@sveltejs/kit/hooks';
import { DEFAULT_LOCALE, negotiateLocale } from '#lib/i18n/locale.js';
import { redirect } from '@sveltejs/kit';
import { legacyRedirect, localeHref, pathLocale } from '#lib/i18n/paths.js';
import { logger } from '#lib/server/logger.js';
import { emitPostHogLog } from '#lib/server/posthog-logger.js';

export const handle: Handle = async ({ event, resolve }) => {
  const started = performance.now();
  const legacy = legacyRedirect(event.url.pathname, event.url.search);
  if (legacy) {
    if (event.url.pathname === '/') {
      event.setHeaders({ Vary: 'Accept-Language' });
      redirect(
        307,
        localeHref(
          '/' + event.url.search,
          negotiateLocale(event.request.headers.get('accept-language'))
        )
      );
    }
    redirect(308, legacy);
  }
  event.locals.locale = pathLocale(event.url.pathname) ?? DEFAULT_LOCALE;
  const response = await resolve(event, {
    transformPageChunk: ({ html }) => html.replace('%lang%', event.locals.locale),
  });
  if (
    event.isDataRequest ||
    response.headers.get('content-type')?.includes('text/html') ||
    event.route.id === '/manifest.webmanifest'
  ) {
    response.headers.set('Content-Language', event.locals.locale);
  }
  event.tracing.root.setAttributes({
    'http.request.method': event.request.method,
    'http.route': event.route.id ?? 'unmatched',
    'http.response.status_code': response.status,
  });
  const active = event.tracing.root.spanContext();
  if (isSpanContextValid(active)) {
    response.headers.set(
      'traceparent',
      `00-${active.traceId}-${active.spanId}-${active.traceFlags.toString(16).padStart(2, '0')}`
    );
  }
  const attributes = {
    'http.request.method': event.request.method,
    'http.route': event.route.id ?? 'unmatched',
    'http.response.status_code': response.status,
    'osl.http.duration_ms': performance.now() - started,
  };
  if (response.status >= 500) {
    event.tracing.root.setAttribute('error.type', String(response.status));
    event.tracing.root.setStatus({ code: SpanStatusCode.ERROR });
    logger.error(attributes, 'Frontend HTTP request failed');
  } else {
    logger.debug(attributes, 'Frontend HTTP request completed');
  }
  emitPostHogLog(response.status >= 500 ? 'ERROR' : 'INFO', 'SvelteKit request completed', {
    event: 'server_request_completed',
    ...attributes,
  });
  return response;
};

export const handleError: HandleServerError = ({ kind, error, event }) => {
  if (kind !== 'unknown') return error;
  logger.error(
    { err: error, 'http.route': event.route.id ?? 'unmatched' },
    'Unhandled frontend server error occurred'
  );
  emitPostHogLog('ERROR', 'Unhandled SvelteKit server error', {
    event: 'server_error',
    'http.route': event.route.id ?? 'unmatched',
    'error.kind': kind,
  });
  return { message: 'An unexpected error occurred' };
};
