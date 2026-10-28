import { expect, it, vi } from 'vitest';
import { handle } from './hooks.server';
import { LOCALES } from '#lib/i18n/locale.js';

vi.mock('#lib/server/logger.js', () => ({ logger: { debug: vi.fn(), error: vi.fn() } }));

function request(path: string, acceptLanguage = 'en') {
  return {
    url: new URL(`https://openstreetlifting.org${path}`),
    request: new Request(`https://openstreetlifting.org${path}`, {
      headers: { 'Accept-Language': acceptLanguage },
    }),
    setHeaders: vi.fn(),
    locals: {},
    route: { id: '/contact' },
    isDataRequest: false,
    tracing: {
      root: {
        setAttributes: vi.fn(),
        spanContext: () => ({ traceId: '0'.repeat(32), spanId: '0'.repeat(16), traceFlags: 0 }),
      },
    },
  };
}

it.each(LOCALES)('uses the %s URL for SSR regardless of browser language', async (locale) => {
  const event = request(`/${locale}/contact`, 'fr');
  const resolve = vi.fn(
    async (_event, options) =>
      new Response(options.transformPageChunk({ html: '<html lang="%lang%">', done: true }), {
        headers: { 'Content-Type': 'text/html', Vary: 'Accept-Encoding' },
      })
  );
  const response = await handle({ event, resolve } as unknown as Parameters<typeof handle>[0]);
  expect(event.locals).toEqual({ locale });
  expect(await response.text()).toBe(`<html lang="${locale}">`);
  expect(response.headers.get('content-language')).toBe(locale);
  expect(response.headers.get('vary')).toBe('Accept-Encoding');
});

it('preserves legacy page links with a permanent English redirect', async () => {
  await expect(
    handle({
      event: request('/competitions?q=open&page=2'),
      resolve: vi.fn(),
    } as unknown as Parameters<typeof handle>[0])
  ).rejects.toMatchObject({ status: 308, location: '/en/competitions?q=open&page=2' });
});

it('negotiates only the unprefixed home page using a temporary redirect', async () => {
  const event = request('/?q=alex', 'pl-PL,pl;q=0.9,en;q=0.8');
  await expect(
    handle({ event, resolve: vi.fn() } as unknown as Parameters<typeof handle>[0])
  ).rejects.toMatchObject({ status: 307, location: '/pl?q=alex' });
  expect(event.setHeaders).toHaveBeenCalledWith({ Vary: 'Accept-Language' });
});
