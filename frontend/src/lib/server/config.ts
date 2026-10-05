import { BACKEND_URL } from '$app/env/private';

export const config = { apiUrl: BACKEND_URL ?? 'http://localhost:8080' } as const;
