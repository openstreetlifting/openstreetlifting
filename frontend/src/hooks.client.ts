import type { HandleClientError } from '@sveltejs/kit/hooks';
import posthog from 'posthog-js';
import { PUBLIC_POSTHOG_HOST, PUBLIC_POSTHOG_PROJECT_TOKEN } from '$app/env/public';

let initialized = false;

function requireConfiguration(
  variable: 'PUBLIC_POSTHOG_HOST' | 'PUBLIC_POSTHOG_PROJECT_TOKEN',
  value: string | undefined
) {
  if (!value && import.meta.env.DEV) {
    console.error(
      `${variable} variable required by PostHog is missing or un-configured, this causes events to be silently missed. This error stops appearing once ${variable} is configured`
    );
  }
  return value;
}

export async function init() {
  const token = requireConfiguration('PUBLIC_POSTHOG_PROJECT_TOKEN', PUBLIC_POSTHOG_PROJECT_TOKEN);
  const host = requireConfiguration('PUBLIC_POSTHOG_HOST', PUBLIC_POSTHOG_HOST);
  if (!token || !host) return;

  posthog.init(token, {
    api_host: host,
    ui_host: 'https://eu.posthog.com',
    defaults: '2026-08-30',
    persistence: 'memory',
    disable_session_recording: true,
    capture_exceptions: true,
  });
  initialized = true;
}

export const handleError: HandleClientError = ({ error }) => {
  if (initialized) posthog.captureException(error);
};
