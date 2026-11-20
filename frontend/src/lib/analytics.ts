import { PUBLIC_UMAMI_SCRIPT_URL, PUBLIC_UMAMI_WEBSITE_ID } from '$app/env/public';

const scriptUrl = PUBLIC_UMAMI_SCRIPT_URL ?? '';
const websiteId = PUBLIC_UMAMI_WEBSITE_ID ?? '';

export const umami = {
  enabled: Boolean(scriptUrl && websiteId),
  scriptUrl,
  websiteId,
} as const;

const AUTOMATED_USER_AGENT =
  /headless|phantomjs|puppeteer|playwright|selenium|webdriver|lighthouse|bot|crawler|spider/i;
const AUTOMATION_GLOBALS = ['_phantom', 'callPhantom', '__nightmare', 'Cypress'];

export function isAutomatedBrowser(
  nav: Pick<Navigator, 'webdriver' | 'userAgent'> = navigator,
  win: object = window
): boolean {
  return (
    nav.webdriver === true ||
    AUTOMATED_USER_AGENT.test(nav.userAgent) ||
    AUTOMATION_GLOBALS.some((name) => name in win)
  );
}

export function loadUmami(): void {
  if (!umami.enabled || isAutomatedBrowser()) return;
  const script = document.createElement('script');
  script.defer = true;
  script.src = umami.scriptUrl;
  script.dataset.websiteId = umami.websiteId;
  document.head.append(script);
}
