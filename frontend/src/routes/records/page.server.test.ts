import { beforeEach, expect, it, vi } from 'vitest';
import { rankingsService, recordsService } from '#lib/server/api/index.js';
import { load } from './+page.server';

vi.mock('#lib/server/logger.js', () => ({ logger: { error: vi.fn() } }));
vi.mock('#lib/server/api/index.js', () => ({
  rankingsService: {
    getRankingCountries: vi.fn(),
    getRankingFederations: vi.fn(),
  },
  recordsService: { getRecords: vi.fn(), getYears: vi.fn() },
}));

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(rankingsService.getRankingCountries).mockResolvedValue(['FR', 'IT']);
  vi.mocked(recordsService.getRecords).mockResolvedValue([]);
  vi.mocked(rankingsService.getRankingFederations).mockResolvedValue(['FNSL']);
  vi.mocked(recordsService.getYears).mockResolvedValue([2026, 2025]);
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
  expect(result).toMatchObject({ gender: 'M', country: null, countries: ['FR', 'IT'] });
});

it('reads the sex and country from the address', async () => {
  const result = await loadAt('?gender=F&country=fr');
  expect(recordsService.getRecords).toHaveBeenCalledWith({
    gender: 'F',
    country: 'FR',
    federation: null,
    year: null,
  });
  expect(result).toMatchObject({ gender: 'F', country: 'FR', federation: null, year: null });
});

it('passes the federation and year', async () => {
  const result = await loadAt('?federation=FNSL&year=2025');
  expect(recordsService.getRecords).toHaveBeenCalledWith({
    gender: 'M',
    country: null,
    federation: 'FNSL',
    year: 2025,
  });
  expect(result).toMatchObject({ federations: ['FNSL'], years: [2026, 2025], year: 2025 });
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
