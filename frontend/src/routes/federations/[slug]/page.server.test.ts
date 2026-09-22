import { beforeEach, expect, it, vi } from 'vitest';
import { competitionsService, rankingsService } from '$lib/server/api';
import { load } from './+page.server';

vi.mock('$lib/server/api', () => ({
  competitionsService: { getAll: vi.fn() },
  rankingsService: { getGlobalRankings: vi.fn() },
}));

const federation = { federation_id: 'f', name: 'FinalRep', abbreviation: null, country: null };

const request = (slug: string, query = '') =>
  ({
    params: { slug },
    url: new URL(`http://localhost/federations/${slug}?${query}`),
  }) as Parameters<typeof load>[0];

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(competitionsService.getAll).mockResolvedValue({
    data: [
      { slug: 'euros-2024', status: 'completed', start_date: '2024-11-02', federation },
      { slug: 'worlds-2025', status: 'completed', start_date: '2025-10-04', federation },
      { slug: 'worlds-2026', status: 'upcoming', start_date: '2026-10-03', federation },
      { slug: 'draft-meet', status: 'draft', start_date: '2026-01-01', federation },
    ],
    pagination: { page: 1, page_size: 100, total_items: 4, total_pages: 1 },
  } as unknown as Awaited<ReturnType<typeof competitionsService.getAll>>);
  vi.mocked(rankingsService.getGlobalRankings).mockResolvedValue({
    data: [],
    pagination: { page: 2, page_size: 50, total_items: 60, total_pages: 2 },
  });
});

it('loads a federation by the slug of its name, newest result first', async () => {
  const result = await load(request('finalrep', 'page=2'));

  expect(result).toMatchObject({
    federation: { name: 'FinalRep', competitions: 2, upcoming: 1 },
    results: [{ slug: 'worlds-2025' }, { slug: 'euros-2024' }],
    upcoming: [{ slug: 'worlds-2026' }],
    pagination: { page: 2 },
  });
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledWith({
    page: 2,
    page_size: 50,
    movement: 'ris',
    federation: 'FinalRep',
  });
});

it('answers an unknown federation with a 404', async () => {
  await expect(load(request('nobody'))).rejects.toMatchObject({ status: 404 });
  expect(rankingsService.getGlobalRankings).not.toHaveBeenCalled();
});

it('treats a backend failure as temporary unavailability', async () => {
  vi.spyOn(console, 'error').mockImplementation(() => {});
  vi.mocked(competitionsService.getAll).mockRejectedValue(new TypeError('fetch failed'));

  await expect(load(request('finalrep'))).rejects.toMatchObject({ status: 503 });
});
