import { expect, it, vi } from 'vitest';
import { handle } from './hooks.server';

vi.mock('#lib/server/logger.js', () => ({ logger: { debug: vi.fn(), error: vi.fn() } }));

it('uses the negotiated locale for SSR HTML and data, with private response caching', async () => {
  const event = {
    cookies: { get: () => undefined },
    request: new Request('https://openstreetlifting.org/contact', {
      headers: { 'Accept-Language': 'fr-CA,fr;q=0.9' },
    }),
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
  const resolve = vi.fn(
    async (_event, options) =>
      new Response(options.transformPageChunk({ html: '<html lang="%lang%">', done: true }), {
        headers: { 'Content-Type': 'text/html', Vary: 'Accept-Encoding' },
      })
  );
  const response = await handle({ event, resolve } as unknown as Parameters<typeof handle>[0]);
  expect(event.locals).toEqual({ locale: 'fr' });
  expect(await response.text()).toBe('<html lang="fr">');
  expect(response.headers.get('content-language')).toBe('fr');
  expect(response.headers.get('vary')).toBe('Accept-Encoding, Accept-Language, Cookie');
  expect(response.headers.get('cache-control')).toBe('private, no-cache');
});
