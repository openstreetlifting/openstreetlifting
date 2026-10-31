import { afterEach, expect, it, vi } from 'vitest';
import { competitionsService } from './competitions';

vi.mock('#lib/server/config.js', () => ({
  config: { apiUrl: 'http://backend.test' },
}));

afterEach(() => vi.unstubAllGlobals());

it('requests previous editions with competition details and preserves their order', async () => {
  const competition = {
    slug: 'current',
    federation: { name: 'Organizer' },
    movements: [],
    categories: [],
    previous_editions: [
      { slug: 'renamed-2025', name: 'Renamed edition', start_date: '2025-06-01' },
      { slug: 'original-2023', name: 'Original edition', start_date: '2023-06-01' },
    ],
  };
  const fetch = vi.fn().mockResolvedValue(Response.json(competition));
  vi.stubGlobal('fetch', fetch);

  expect(await competitionsService.getById('current')).toEqual(competition);
  const url = new URL((fetch.mock.calls[0][0] as Request).url);
  expect(url.pathname).toBe('/api/v1/competitions/current');
  expect(url.searchParams.get('include')).toBe('federation,results,movements,previous_editions');
});

it('accepts an empty previous-editions list for a first edition', async () => {
  vi.stubGlobal(
    'fetch',
    vi.fn().mockResolvedValue(
      Response.json({
        federation: { name: 'Organizer' },
        movements: [],
        categories: [],
        previous_editions: [],
      })
    )
  );
  expect((await competitionsService.getById('first')).previous_editions).toEqual([]);
});
