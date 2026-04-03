import { config } from '$lib/config/env';

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
	params?: Record<string, string | number | boolean>;
}

class ApiClient {
	private baseUrl: string;

	constructor(baseUrl: string) {
		this.baseUrl = baseUrl.replace(/\/$/, ''); // Remove trailing slash
	}

	private buildUrl(path: string, params?: Record<string, string | number | boolean>): string {
		const url = new URL(`${this.baseUrl}${path}`);

		if (params) {
			Object.entries(params).forEach(([key, value]) => {
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
				// If parsing JSON fails, use the default error message
			}

			throw new ApiError(response.status, response.statusText, errorMessage);
		}

		// Handle 204 No Content
		if (response.status === 204) {
			return null as T;
		}

		return response.json();
	}

	async get<T>(path: string, options?: RequestOptions): Promise<T> {
		const url = this.buildUrl(path, options?.params);
		const response = await fetch(url, {
			...options,
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				...options?.headers
			}
		});

		return this.handleResponse<T>(response);
	}

	async post<T>(path: string, body?: unknown, options?: RequestOptions): Promise<T> {
		const url = this.buildUrl(path, options?.params);
		const response = await fetch(url, {
			...options,
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				...options?.headers
			},
			body: body ? JSON.stringify(body) : undefined
		});

		return this.handleResponse<T>(response);
	}

	async put<T>(path: string, body?: unknown, options?: RequestOptions): Promise<T> {
		const url = this.buildUrl(path, options?.params);
		const response = await fetch(url, {
			...options,
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				...options?.headers
			},
			body: body ? JSON.stringify(body) : undefined
		});

		return this.handleResponse<T>(response);
	}

	async patch<T>(path: string, body?: unknown, options?: RequestOptions): Promise<T> {
		const url = this.buildUrl(path, options?.params);
		const response = await fetch(url, {
			...options,
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json',
				...options?.headers
			},
			body: body ? JSON.stringify(body) : undefined
		});

		return this.handleResponse<T>(response);
	}

	async delete<T>(path: string, options?: RequestOptions): Promise<T> {
		const url = this.buildUrl(path, options?.params);
		const response = await fetch(url, {
			...options,
			method: 'DELETE',
			headers: {
				'Content-Type': 'application/json',
				...options?.headers
			}
		});

		return this.handleResponse<T>(response);
	}
}

// Export a singleton instance
export const apiClient = new ApiClient(config.apiUrl);
