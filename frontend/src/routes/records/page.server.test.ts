import { beforeEach, expect, it, vi } from 'vitest';
import { recordsService } from '#lib/server/api/index.js';
import { load } from './+page.server';

vi.mock('#lib/server/logger.js', () => ({ logger: { error: vi.fn() } }));
vi.mock('#lib/server/api/index.js', () => ({
  recordsService: { getRecords: vi.fn(), getFacets: vi.fn() },
}));

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(recordsService.getRecords).mockResolvedValue([]);
  vi.mocked(recordsService.getFacets).mockResolvedValue({
    countries: ['FR', 'IT'],
    federations: ['FNSL'],
    years: [2026, 2025],
  });
});

function loadAt(search: string) {
  return load({ url: new URL(`http://localhost/en/records${search}`) } as Parameters<
    typeof load
  >[0]);
}

it('shows men by default across every country', async () => {
  const result = await loadAt('');
  expect(recordsService.getRecords).toHaveBeenCalledWith({
    gender: 'M',
    country: null,
    federation: null,
    year: null,
  });
  expect(result).toMatchObject({
    gender: 'M',
    filters: { country: null },
    facets: { countries: ['FR', 'IT'] },
  });
});

it('reads the sex and country from the address', async () => {
  const result = await loadAt('?gender=F&country=fr');
  expect(recordsService.getRecords).toHaveBeenCalledWith({
    gender: 'F',
    country: 'FR',
    federation: null,
    year: null,
  });
  expect(result).toMatchObject({
    gender: 'F',
    filters: { country: 'FR', federation: null, year: null },
  });
});

it('passes the federation and year', async () => {
  const result = await loadAt('?federation=FNSL&year=2025');
  expect(recordsService.getRecords).toHaveBeenCalledWith({
    gender: 'M',
    country: null,
    federation: 'FNSL',
    year: 2025,
  });
  expect(recordsService.getFacets).toHaveBeenCalledWith({
    gender: 'M',
    country: null,
    federation: 'FNSL',
    year: 2025,
  });
  expect(result).toMatchObject({
    facets: { federations: ['FNSL'], years: [2026, 2025] },
    filters: { year: 2025 },
  });
});

it('keeps a selected value that its options no longer offer', async () => {
  const result = await loadAt('?country=DE&year=2019');
  expect(result).toMatchObject({
    facets: { countries: ['FR', 'IT', 'DE'], years: [2026, 2025, 2019] },
  });
});

it('falls back to men for an unknown sex', async () => {
  await loadAt('?gender=MX');
  expect(recordsService.getRecords).toHaveBeenCalledWith({
    gender: 'M',
    country: null,
    federation: null,
    year: null,
  });
});

it('answers 503 when the API fails', async () => {
  vi.mocked(recordsService.getRecords).mockRejectedValue(new Error('down'));
  await expect(loadAt('')).rejects.toMatchObject({ status: 503 });
});
