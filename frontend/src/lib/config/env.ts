import { env } from '$env/dynamic/public';

export const config = {
	apiUrl: env.PUBLIC_OPENSTREETLIFTING_API_URL || 'http://localhost:8080'
} as const;
