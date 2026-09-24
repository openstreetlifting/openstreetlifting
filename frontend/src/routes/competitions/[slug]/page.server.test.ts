import { beforeEach, expect, it, vi } from 'vitest';
import { competitionsService, rankingsService } from '$lib/server/api';
import { load } from './+page.server';

vi.mock('$lib/server/api', () => ({
  competitionsService: { getById: vi.fn() },
  rankingsService: {
    getGlobalRankings: vi.fn(),
    getRankingClasses: vi.fn(),
    getRankingCountries: vi.fn(),
  },
}));

const athlete = { slug: 'alex-martin', first_name: 'Alex', last_name: 'Martin' };
const request = (query: string) =>
  ({
    params: { slug: 'euros' },
    url: new URL(`http://localhost/competitions/euros?${query}`),
  }) as Parameters<typeof load>[0];

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(competitionsService.getById).mockResolvedValue({
    competition_id: 'meet-id',
    movements: ['Muscle-up', 'Pull-up', 'Dips', 'Squat'],
    categories: [{ participants: [{ athlete }] }],
  } as unknown as Awaited<ReturnType<typeof competitionsService.getById>>);
  vi.mocked(rankingsService.getRankingClasses).mockResolvedValue([]);
  vi.mocked(rankingsService.getRankingCountries).mockResolvedValue([]);
});

it('opens the page containing the requested athlete while retaining the chosen metric', async () => {
  vi.mocked(rankingsService.getGlobalRankings).mockResolvedValue({
    data: [{ athlete, rank: 76 }],
    pagination: { page: 1, page_size: 50, total_items: 1, total_pages: 1 },
  } as Awaited<ReturnType<typeof rankingsService.getGlobalRankings>>);
  await expect(load(request('athlete=alex-martin&movement=squat'))).rejects.toMatchObject({
    status: 307,
    location: '/competitions/euros?athlete=alex-martin&movement=squat&page=2',
  });
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledWith(
    expect.objectContaining({ q: 'Alex Martin', movement: 'squat', competition_id: 'meet-id' })
  );
});

it('respects an explicit page without repeating the athlete lookup', async () => {
  vi.mocked(rankingsService.getGlobalRankings).mockResolvedValue({
    data: [],
    pagination: { page: 2, page_size: 50, total_items: 100, total_pages: 2 },
  });
  const result = await load(request('athlete=alex-martin&movement=squat&page=2'));
  expect(result).toMatchObject({ pagination: { page: 2 } });
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledTimes(1);
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledWith(
    expect.objectContaining({ page: 2, q: null })
  );
});

it('ranks an All4 meet on total when its source published no bodyweight', async () => {
  vi.mocked(rankingsService.getGlobalRankings).mockResolvedValue({
    data: [],
    pagination: { page: 1, page_size: 50, total_items: 0, total_pages: 0 },
  });
  const result = await load(request(''));
  expect(result).toMatchObject({ ris: 'not-published' });
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledWith(
    expect.objectContaining({ movement: 'total' })
  );
});

it('keeps RIS as the default once an athlete carries a recomputed score', async () => {
  vi.mocked(competitionsService.getById).mockResolvedValue({
    competition_id: 'meet-id',
    movements: ['Muscle-up', 'Pull-up', 'Dips', 'Squat'],
    categories: [{ participants: [{ athlete, ris_score: '312.45', ris_source: 'computed' }] }],
  } as unknown as Awaited<ReturnType<typeof competitionsService.getById>>);
  vi.mocked(rankingsService.getGlobalRankings).mockResolvedValue({
    data: [],
    pagination: { page: 1, page_size: 50, total_items: 0, total_pages: 0 },
  });
  const result = await load(request(''));
  expect(result).toMatchObject({ ris: 'recomputed' });
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledWith(
    expect.objectContaining({ movement: 'ris' })
  );
});

it('reports a failed ranking request instead of showing only unranked participants', async () => {
  vi.mocked(rankingsService.getGlobalRankings).mockRejectedValue(new Error('API unavailable'));
  const log = vi.spyOn(console, 'error').mockImplementation(() => {});
  try {
    await expect(load(request(''))).rejects.toMatchObject({
      status: 502,
      body: { message: 'Competition results are temporarily unavailable' },
    });
  } finally {
    log.mockRestore();
  }
});

it('uses the declared scoring rule for mixed contests and keeps the mixed filter', async () => {
  vi.mocked(competitionsService.getById).mockResolvedValue({
    competition_id: 'meet-id',
    scoring: 'total',
    movements: ['Muscle-up', 'Pull-up', 'Dips', 'Squat'],
    categories: [
      {
        category: { gender: 'MX' },
        participants: [{ athlete, ris_score: '80', ris_source: 'computed' }],
      },
    ],
  } as unknown as Awaited<ReturnType<typeof competitionsService.getById>>);
  vi.mocked(rankingsService.getGlobalRankings).mockResolvedValue({
    data: [],
    pagination: { page: 1, page_size: 50, total_items: 0, total_pages: 0 },
  });
  await load(request('gender=MX'));
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledWith(
    expect.objectContaining({ movement: 'total', gender: 'MX' })
  );
});
