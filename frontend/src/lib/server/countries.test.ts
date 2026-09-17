import { expect, it, vi } from 'vitest';
import type { AthleteSummary } from '$lib/types/athlete';
import type { Competition } from '$lib/types/competition';
import { summarizeCountries } from './countries';

vi.mock('$lib/server/api', () => ({
  athletesService: { getAll: vi.fn() },
  competitionsService: { getAll: vi.fn() },
}));

const competition = (overrides: Partial<Competition>): Competition =>
  ({
    status: 'completed',
    country: 'FR',
    federation: { federation_id: 'f', name: 'FNSL', abbreviation: null, country: 'FR' },
    ...overrides,
  }) as Competition;

const athlete = (country: string) => ({ country }) as AthleteSummary;

it('counts athletes, meets held and federations based in each country', () => {
  const finalrep = { federation_id: 'r', name: 'FinalRep', abbreviation: null, country: null };
  const summaries = summarizeCountries(
    [
      competition({}),
      competition({ status: 'upcoming' }),
      competition({ country: 'DE', federation: finalrep }),
    ],
    [athlete('FR'), athlete('FR'), athlete('DE'), athlete('IS')],
    ['IS']
  );

  expect(summaries).toEqual([
    { code: 'FR', athletes: 2, competitions: 1, upcoming: 1, federations: ['FNSL'] },
    { code: 'DE', athletes: 1, competitions: 1, upcoming: 0, federations: [] },
    { code: 'IS', athletes: 1, competitions: 0, upcoming: 0, federations: [] },
  ]);
});
