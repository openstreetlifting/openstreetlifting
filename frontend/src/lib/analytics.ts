import { env } from '$env/dynamic/public';

const scriptUrl = env.PUBLIC_UMAMI_SCRIPT_URL ?? '';
const websiteId = env.PUBLIC_UMAMI_WEBSITE_ID ?? '';

export const umami = {
  enabled: Boolean(scriptUrl && websiteId),
  scriptUrl,
  websiteId,
} as const;
