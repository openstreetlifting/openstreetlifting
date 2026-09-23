import { beforeEach, expect, it, vi } from 'vitest';
import { rankingsService } from '$lib/server/api';
import { load } from './+page.server';

vi.mock('$lib/server/api', () => ({
  rankingsService: {
    getGlobalRankings: vi.fn(),
    getRankingClasses: vi.fn(),
    getRankingYears: vi.fn(),
    getRankingCountries: vi.fn(),
    getRankingFederations: vi.fn(),
  },
}));

beforeEach(() => {
  vi.resetAllMocks();
  for (const method of [
    'getRankingClasses',
    'getRankingYears',
    'getRankingCountries',
    'getRankingFederations',
  ] as const) {
    vi.mocked(rankingsService[method]).mockResolvedValue([]);
  }
  vi.mocked(rankingsService.getGlobalRankings).mockResolvedValue({
    data: [],
    pagination: { page: 2, page_size: 100, total_items: 201, total_pages: 3 },
  });
});

it('requests 100 athletes on the main ranking table while retaining filters', async () => {
  const result = await load({
    url: new URL('http://localhost/?page=2&gender=F&country=FR'),
  } as Parameters<typeof load>[0]);
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledWith(
    expect.objectContaining({
      page: 2,
      page_size: 100,
      gender: 'F',
      country: 'FR',
    })
  );
  expect(result).toMatchObject({ pagination: { page_size: 100, total_pages: 3 } });
  expect(rankingsService.getRankingClasses).not.toHaveBeenCalled();
  expect(rankingsService.getRankingYears).not.toHaveBeenCalled();
  expect(rankingsService.getRankingCountries).not.toHaveBeenCalled();
  expect(rankingsService.getRankingFederations).not.toHaveBeenCalled();
});
