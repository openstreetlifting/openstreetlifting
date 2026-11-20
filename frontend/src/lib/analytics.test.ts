import { expect, it } from 'vitest';
import { isAutomatedBrowser } from './analytics';

const chrome =
  'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36';

it('lets a normal browser through', () => {
  expect(isAutomatedBrowser({ webdriver: false, userAgent: chrome }, {})).toBe(false);
});

it('flags a browser driven by webdriver', () => {
  expect(isAutomatedBrowser({ webdriver: true, userAgent: chrome }, {})).toBe(true);
});

it.each([
  'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/131.0.0.0 Safari/537.36',
  'Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)',
  'Mozilla/5.0 (Linux; Android 11; moto g power (2022)) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36 Chrome-Lighthouse',
])('flags the automated user agent %s', (userAgent) => {
  expect(isAutomatedBrowser({ webdriver: false, userAgent }, {})).toBe(true);
});

it('flags automation globals', () => {
  expect(isAutomatedBrowser({ webdriver: false, userAgent: chrome }, { Cypress: {} })).toBe(true);
});
