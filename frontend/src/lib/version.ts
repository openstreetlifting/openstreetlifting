import { env } from '$env/dynamic/public';

const appVersion = env.PUBLIC_APP_VERSION ?? '';
const gitSha = env.PUBLIC_GIT_SHA ?? '';
const environment = env.PUBLIC_ENVIRONMENT ?? '';

// Outside production the chart's appVersion is the last release, not what is
// running, so only the commit is shown.
const isProduction = environment === 'production';

export const buildInfo = {
  label: isProduction ? '' : environment,
  version: isProduction && appVersion ? appVersion : '',
  sha: gitSha,
  shortSha: gitSha.slice(0, 7),
} as const;
