import { config } from '#lib/server/config.js';
import { context, propagation, SpanKind, SpanStatusCode, trace } from '@opentelemetry/api';

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

export interface RequestOptions extends RequestInit {
  params?: Record<string, string | number | boolean | null | undefined>;
}

class ApiClient {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl.replace(/\/$/, '');
  }

  private buildUrl(
    path: string,
    params?: Record<string, string | number | boolean | null | undefined>
  ): string {
    const url = new URL(`${this.baseUrl}${path}`);

    if (params) {
      Object.entries(params).forEach(([key, value]) => {
        // An unset filter must be left out of the query string, since the API
        // reads a literal "undefined" as a value it does not know.
        if (value === null || value === undefined) {
          return;
        }

        url.searchParams.append(key, String(value));
      });
    }

    return url.toString();
  }

  private async handleResponse<T>(response: Response): Promise<T> {
    if (!response.ok) {
      let errorMessage = `HTTP ${response.status}: ${response.statusText}`;

      try {
        const errorData = await response.json();
        errorMessage = errorData.message || errorData.error || errorMessage;
      } catch {
        // Body was not JSON, keep the status line as the message.
      }

      throw new ApiError(response.status, response.statusText, errorMessage);
    }

    if (response.status === 204) {
      return null as T;
    }

    return response.json();
  }

  async get<T>(path: string, options?: RequestOptions): Promise<T> {
    const url = this.buildUrl(path, options?.params);
    return trace.getTracer('osl.frontend.api').startActiveSpan(
      'GET backend',
      {
        kind: SpanKind.CLIENT,
        attributes: { 'http.request.method': 'GET', 'server.address': new URL(url).hostname },
      },
      async (span) => {
        try {
          const headers = new Headers(options?.headers);
          headers.set('Content-Type', 'application/json');
          const carrier: Record<string, string> = {};
          propagation.inject(context.active(), carrier);
          for (const [key, value] of Object.entries(carrier)) headers.set(key, value);
          const response = await fetch(url, { ...options, method: 'GET', headers });
          span.setAttribute('http.response.status_code', response.status);
          return await this.handleResponse<T>(response);
        } catch (error) {
          span.setStatus({ code: SpanStatusCode.ERROR });
          span.setAttribute('error.type', error instanceof Error ? error.name : 'Error');
          throw error;
        } finally {
          span.end();
        }
      }
    );
  }
}

export const apiClient = new ApiClient(config.apiUrl);
