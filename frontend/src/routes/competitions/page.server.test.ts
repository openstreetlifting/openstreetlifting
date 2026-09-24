import { beforeEach, expect, it, vi } from 'vitest';
import { competitionsService } from '$lib/server/api';
import { load } from './+page.server';

vi.mock('$lib/server/api', () => ({
  competitionsService: { getAll: vi.fn(), getFacets: vi.fn() },
}));

const request = (query: string) =>
  ({
    url: new URL(`http://localhost/competitions?${query}`),
  }) as Parameters<typeof load>[0];

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(competitionsService.getAll).mockResolvedValue({
    data: [],
    pagination: { page: 1, page_size: 50, total_items: 0, total_pages: 0 },
  });
  vi.mocked(competitionsService.getFacets).mockResolvedValue({
    federations: ['Test'],
    countries: ['FR'],
    years: [2026],
    formats: ['PD', 'MPDS'],
  });
});

it('normalizes selection order and applies the format to both tab counts', async () => {
  const result = await load(request('event=DP&country=FR&year=2026&federation=Test&page=2'));
  expect(competitionsService.getAll).toHaveBeenCalledWith(
    expect.objectContaining({
      event: 'PD',
      country: 'FR',
      year: 2026,
      federation: 'Test',
      status: 'completed',
      page: 2,
    })
  );
  expect(competitionsService.getAll).toHaveBeenCalledWith(
    expect.objectContaining({
      event: 'PD',
      country: 'FR',
      year: 2026,
      federation: 'Test',
      status: 'upcoming',
      page_size: 1,
    })
  );
  expect(result).toMatchObject({ event: 'PD', facets: { formats: ['PD', 'MPDS'] } });
  expect(competitionsService.getFacets).toHaveBeenCalledWith();
});

it('keeps a valid but nonexistent combination so the page can show no matches', async () => {
  const result = await load(request('event=MS&status=upcoming'));
  expect(result).toMatchObject({
    event: 'MS',
    competitions: [],
    facets: { formats: ['PD', 'MPDS'] },
  });
  expect(competitionsService.getAll).toHaveBeenCalledWith(
    expect.objectContaining({ event: 'MS', status: 'upcoming' })
  );
});

it.each(['', 'event=', 'event=invalid'])('leaves format unrestricted for %s', async (query) => {
  await load(request(query));
  expect(competitionsService.getAll).toHaveBeenCalledWith(
    expect.objectContaining({ event: undefined })
  );
});
