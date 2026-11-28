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
    pagination: { page: 2, page_size: 50, total_items: 101, total_pages: 3 },
  });
});

it('requests 50 athletes on the main ranking table while retaining filters', async () => {
  const result = await load({
    url: new URL(
      'http://localhost/?page=2&gender=F&country=FR&series=dcsv-sueddeutsche-meisterschaft'
    ),
  } as Parameters<typeof load>[0]);
  expect(rankingsService.getGlobalRankings).toHaveBeenCalledWith(
    expect.objectContaining({
      page: 2,
      page_size: 50,
      gender: 'F',
      country: 'FR',
      series: 'dcsv-sueddeutsche-meisterschaft',
    })
  );
  expect(result).toMatchObject({ pagination: { page_size: 50, total_pages: 3 } });
  expect(rankingsService.getFacets).not.toHaveBeenCalled();
});
