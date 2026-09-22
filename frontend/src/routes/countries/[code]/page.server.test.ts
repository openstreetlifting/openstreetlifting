import { beforeEach, expect, it, vi } from 'vitest';
import { competitionsService, rankingsService } from '$lib/server/api';
import { load } from './+page.server';

vi.mock('$lib/server/api', () => ({
  athletesService: { getAll: vi.fn() },
  competitionsService: { getAll: vi.fn() },
  rankingsService: { getGlobalRankings: vi.fn(), getRankingCountries: vi.fn() },
}));

const fnsl = { federation_id: 'f', name: 'FNSL', abbreviation: null, country: 'FR' };

const request = (code: string, query = '') =>
  ({
    params: { code },
    url: new URL(`http://localhost/countries/${code}?${query}`),
  }) as Parameters<typeof load>[0];

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(competitionsService.getAll).mockResolvedValue({
    data: [
      {
        slug: 'cnsl-2024',
        status: 'completed',
        country: 'FR',
        start_date: '2024-06-23',
        federation: fnsl,
      },
      {
        slug: 'cnsl-2025',
        status: 'completed',
        country: 'FR',
        start_date: '2025-06-07',
        federation: fnsl,
      },
      {
        slug: 'elite-2027',
        status: 'upcoming',
        country: 'FR',
        start_date: '2027-06-12',
        federation: fnsl,
      },
    ],
    pagination: { page: 1, page_size: 100, total_items: 3, total_pages: 1 },
  } as unknown as Awaited<ReturnType<typeof competitionsService.getAll>>);
  vi.mocked(rankingsService.getRankingCountries).mockResolvedValue(['FR', 'IS']);
  vi.mocked(rankingsService.getGlobalRankings).mockResolvedValue({
    data: [],
    pagination: { page: 1, page_size: 50, total_items: 355, total_pages: 8 },
  });
});

it('loads the meets held in a country and its athletes ranked by RIS', async () => {
  const result = await load(request('fr', 'page=3'));

  expect(result).toMatchObject({
    code: 'FR',
    federations: ['FNSL'],
    results: [{ slug: 'cnsl-2025' }, { slug: 'cnsl-2024' }],
    upcoming: [{ slug: 'elite-2027' }],
  });
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledWith({
    page: 3,
    page_size: 50,
    movement: 'ris',
    country: 'FR',
  });
});

it('keeps a country with ranked athletes but no meets', async () => {
  await expect(load(request('is'))).resolves.toMatchObject({ code: 'IS', results: [] });
});

it('redirects an uppercase code to the lowercase URL', async () => {
  await expect(load(request('FR', 'page=2'))).rejects.toMatchObject({
    status: 308,
    location: '/countries/fr?page=2',
  });
});

it('answers a country with no athletes and no meets with a 404', async () => {
  await expect(load(request('zz'))).rejects.toMatchObject({ status: 404 });
});
