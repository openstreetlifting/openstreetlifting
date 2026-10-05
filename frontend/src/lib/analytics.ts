import { PUBLIC_UMAMI_SCRIPT_URL, PUBLIC_UMAMI_WEBSITE_ID } from '$app/env/public';

const scriptUrl = PUBLIC_UMAMI_SCRIPT_URL ?? '';
const websiteId = PUBLIC_UMAMI_WEBSITE_ID ?? '';

export const umami = {
  enabled: Boolean(scriptUrl && websiteId),
  scriptUrl,
  websiteId,
} as const;
