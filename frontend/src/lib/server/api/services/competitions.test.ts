import { afterEach, expect, it, vi } from 'vitest';
import { competitionsService } from './competitions';

vi.mock('#lib/server/config.js', () => ({
  config: { apiUrl: 'http://backend.test' },
}));

afterEach(() => vi.unstubAllGlobals());

it('requests editions with competition details and preserves their order', async () => {
  const competition = {
    slug: 'current',
    federation: { name: 'Organizer' },
    movements: [],
    categories: [],
    editions: [
      { slug: 'original-2023', name: 'Original edition', start_date: '2023-06-01' },
      { slug: 'renamed-2025', name: 'Renamed edition', start_date: '2025-06-01' },
    ],
  };
  const fetch = vi.fn().mockResolvedValue(Response.json(competition));
  vi.stubGlobal('fetch', fetch);

  expect(await competitionsService.getById('current')).toEqual(competition);
  const url = new URL((fetch.mock.calls[0][0] as Request).url);
  expect(url.pathname).toBe('/api/v1/competitions/current');
  expect(url.searchParams.get('include')).toBe('federation,results,movements,editions');
});

it('accepts an empty editions list for an unassigned competition', async () => {
  vi.stubGlobal(
    'fetch',
    vi.fn().mockResolvedValue(
      Response.json({
        federation: { name: 'Organizer' },
        movements: [],
        categories: [],
        editions: [],
      })
    )
  );
  expect((await competitionsService.getById('first')).editions).toEqual([]);
});
