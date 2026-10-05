import { PUBLIC_APP_VERSION, PUBLIC_GIT_SHA, PUBLIC_ENVIRONMENT } from '$app/env/public';

const appVersion = PUBLIC_APP_VERSION ?? '';
const gitSha = PUBLIC_GIT_SHA ?? '';
const environment = PUBLIC_ENVIRONMENT ?? '';

// Outside production the chart's appVersion is the last release, not what is
// running, so only the commit is shown.
const isProduction = environment === 'production';

export const buildInfo = {
  label: isProduction ? '' : environment,
  version: isProduction && appVersion ? appVersion : '',
  sha: gitSha,
  shortSha: gitSha.slice(0, 7),
} as const;
