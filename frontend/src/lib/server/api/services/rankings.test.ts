import { afterEach, expect, it, vi } from 'vitest';
import { rankingsService } from './rankings';

vi.mock('#lib/server/config.js', () => ({
  config: { apiUrl: 'http://backend.test' },
}));

afterEach(() => vi.unstubAllGlobals());

it('sends event and ranking filters through the API client without unset values', async () => {
  const response = {
    data: [],
    pagination: { page: 2, page_size: 100, total_items: 0, total_pages: 0 },
  };
  const fetch = vi.fn().mockResolvedValue(Response.json(response));
  vi.stubGlobal('fetch', fetch);

  expect(
    await rankingsService.getGlobalRankings({
      page: 2,
      page_size: 100,
      movement: 'total',
      event: 'PD',
      direction: 'asc',
      gender: 'F',
      country: null,
      federation: 'Street Lifting & Co',
      category: '-63kg',
      year: 2026,
      q: 'Anne & Léa',
      competition_id: undefined,
    })
  ).toEqual(response);

  const url = new URL(fetch.mock.calls[0][0]);
  expect(url.origin + url.pathname).toBe('http://backend.test/api/v1/rankings');
  expect(Object.fromEntries(url.searchParams)).toEqual({
    page: '2',
    page_size: '100',
    movement: 'total',
    event: 'PD',
    direction: 'asc',
    gender: 'F',
    federation: 'Street Lifting & Co',
    category: '-63kg',
    year: '2026',
    q: 'Anne & Léa',
  });
});
