import { config } from '#lib/server/config.js';
import type { paths } from '#lib/api/schema.generated.js';
import { context, propagation, SpanKind, SpanStatusCode, trace } from '@opentelemetry/api';
import createClient from 'openapi-fetch';

export class ApiError extends Error {
  constructor(
    public status: number,
    public statusText: string,
    message: string
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

export const apiClient = createClient<paths>({
  baseUrl: config.apiUrl,
  fetch: (request) =>
    trace.getTracer('osl.frontend.api').startActiveSpan(
      `${request.method} backend`,
      {
        kind: SpanKind.CLIENT,
        attributes: {
          'http.request.method': request.method,
          'server.address': new URL(request.url).hostname,
        },
      },
      async (span) => {
        try {
          const carrier: Record<string, string> = {};
          propagation.inject(context.active(), carrier);
          for (const [key, value] of Object.entries(carrier)) request.headers.set(key, value);
          const response = await fetch(request);
          span.setAttribute('http.response.status_code', response.status);

          if (!response.ok) {
            const body = await response.json().catch(() => null);
            throw new ApiError(
              response.status,
              response.statusText,
              body?.message || body?.error || `HTTP ${response.status}: ${response.statusText}`
            );
          }

          return response;
        } catch (error) {
          span.setStatus({ code: SpanStatusCode.ERROR });
          span.setAttribute('error.type', error instanceof Error ? error.name : 'Error');
          throw error;
        } finally {
          span.end();
        }
      }
    ),
});

export async function apiData<T>(request: Promise<{ data?: T }>): Promise<T> {
  const { data } = await request;
  if (data === undefined) throw new Error('API response has no data');
  return data;
}
