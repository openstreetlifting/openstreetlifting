import { beforeEach, expect, it, vi } from 'vitest';
import { athletesService, rankingsService } from '#lib/server/api/index.js';
import { load } from './+page.server';

vi.mock('#lib/server/api/index.js', () => ({
  rankingsService: { getGlobalRankings: vi.fn(), getFacets: vi.fn() },
  athletesService: { search: vi.fn() },
}));

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(rankingsService.getGlobalRankings).mockResolvedValue({
    data: [],
    pagination: { page: 2, page_size: 100, total_items: 201, total_pages: 3 },
  });
});

it('requests 100 athletes on the main ranking table while retaining filters', async () => {
  const result = await load({
    url: new URL(
      'http://localhost/?page=2&gender=F&country=FR&series=dcsv-sueddeutsche-meisterschaft'
    ),
  } as Parameters<typeof load>[0]);
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

const hit = (slug: string, rankings: ('total' | 'pullup')[] = []) => ({
  slug,
  first_name: 'A',
  last_name: slug,
  gender: 'M' as const,
  country: 'IT',
  rankings,
});

it('lists matching athletes outside the ranking when searching', async () => {
  vi.mocked(rankingsService.getGlobalRankings).mockResolvedValue({
    data: [{ athlete: { slug: 'jose-rossi' } }],
    pagination: { page: 1, page_size: 100, total_items: 1, total_pages: 1 },
  } as never);
  vi.mocked(athletesService.search).mockResolvedValue({
    data: [hit('jose-rossi', ['total']), hit('marco-rossi', ['pullup']), hit('luca-rossi')],
    total_items: 3,
  });

  const result = (await load({
    url: new URL('http://localhost/?q=rossi'),
  } as Parameters<typeof load>[0])) as { otherAthletes: unknown };

  expect(athletesService.search).toHaveBeenCalledWith('rossi', 20);
  expect(result.otherAthletes).toEqual({
    hits: [hit('marco-rossi', ['pullup']), hit('luca-rossi')],
    total: 2,
  });
});

it('skips the athlete search without a query and survives its failure', async () => {
  await load({ url: new URL('http://localhost/') } as Parameters<typeof load>[0]);
  expect(athletesService.search).not.toHaveBeenCalled();

  vi.mocked(athletesService.search).mockRejectedValue(new Error('down'));
  const result = (await load({
    url: new URL('http://localhost/?q=rossi'),
  } as Parameters<typeof load>[0])) as { otherAthletes: unknown; error?: string };
  expect(result.otherAthletes).toBeNull();
  expect(result.error).toBeUndefined();
});
