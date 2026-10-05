import { defineEnvVars } from '@sveltejs/kit/env';

export const variables = defineEnvVars({
  PUBLIC_UMAMI_SCRIPT_URL: { public: true, schema: (input) => input ?? '' },
  PUBLIC_UMAMI_WEBSITE_ID: { public: true, schema: (input) => input ?? '' },
  PUBLIC_APP_VERSION: { public: true, schema: (input) => input ?? '' },
  PUBLIC_GIT_SHA: { public: true, schema: (input) => input ?? '' },
  PUBLIC_ENVIRONMENT: { public: true, schema: (input) => input ?? '' },
  BACKEND_URL: { schema: (input) => input ?? 'http://localhost:8080' },
  PUBLIC_SITE_URL: { public: true, schema: (input) => input ?? 'https://openstreetlifting.org' },
});
