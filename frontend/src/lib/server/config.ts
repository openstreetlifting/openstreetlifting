import { env } from '$env/dynamic/private';

export const config = {
  apiUrl: env.BACKEND_URL ?? 'http://localhost:8080',
} as const;
