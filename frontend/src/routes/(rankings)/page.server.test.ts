import { beforeEach, expect, it, vi } from 'vitest';
import { rankingsService } from '#lib/server/api/index.js';
import { load } from './+page.server';

vi.mock('#lib/server/api/index.js', () => ({
  rankingsService: { getGlobalRankings: vi.fn(), getFacets: vi.fn() },
}));

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(rankingsService.getGlobalRankings).mockResolvedValue({
    data: [],
    pagination: { page: 2, page_size: 100, total_items: 201, total_pages: 3 },
  });
});

function loadAt(search: string) {
  return load({ url: new URL(`http://localhost/${search}`) } as Parameters<typeof load>[0]);
}

it('requests 100 athletes on the main ranking table while retaining filters', async () => {
  const result = await loadAt('?page=2&gender=F&country=FR&series=dcsv-sueddeutsche-meisterschaft');
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledWith(
    expect.objectContaining({
      page: 2,
      page_size: 100,
      gender: 'F',
      country: 'FR',
      series: 'dcsv-sueddeutsche-meisterschaft',
    })
  );
  expect(result).toMatchObject({ pagination: { page_size: 100, total_pages: 3 } });
  expect(rankingsService.getFacets).not.toHaveBeenCalled();
});

function boardTotals(totals: Record<string, number>) {
  vi.mocked(rankingsService.getGlobalRankings).mockImplementation(async (params) => ({
    data: [],
    pagination: {
      page: 1,
      page_size: params.page_size ?? 100,
      total_items: totals[params.movement ?? 'ris'] ?? 0,
      total_pages: 1,
    },
  }));
}

it('counts the athletes a one-event board hides from a name search', async () => {
  boardTotals({ ris: 2, name: 5 });

  const result = await loadAt('?q=michal');

  expect(result).toMatchObject({ otherEventAthletes: 3 });
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledWith(
    expect.objectContaining({ movement: 'name', q: 'michal', page_size: 1 })
  );
});

it('does not look for hidden athletes without a search or on a board that spans events', async () => {
  boardTotals({ ris: 2, name: 5 });

  const unfiltered = await loadAt('');
  const directory = await loadAt('?q=michal&movement=name');

  expect(unfiltered).toMatchObject({ otherEventAthletes: 0 });
  expect(directory).toMatchObject({ otherEventAthletes: 0 });
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledTimes(2);
});
