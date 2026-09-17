import { expect, it, vi } from 'vitest';
import type { Competition } from '$lib/types/competition';
import { summarizeFederations } from './federations';

vi.mock('$lib/server/api', () => ({ competitionsService: { getAll: vi.fn() } }));

const competition = (overrides: Partial<Competition>): Competition =>
  ({
    status: 'completed',
    country: 'FR',
    start_date: '2025-01-01',
    lifter_count: 10,
    federation: { federation_id: 'f', name: 'FinalRep', abbreviation: null, country: null },
    ...overrides,
  }) as Competition;

it('groups competitions by federation and counts only held meets', () => {
  const [summary] = summarizeFederations([
    competition({ start_date: '2023-05-01', country: 'DE' }),
    competition({ start_date: '2025-10-04', lifter_count: 82 }),
    competition({ status: 'upcoming', start_date: '2027-01-01', lifter_count: 0 }),
    competition({ status: 'cancelled', start_date: '2020-01-01' }),
  ]);

  expect(summary).toEqual({
    slug: 'finalrep',
    name: 'FinalRep',
    abbreviation: null,
    country: null,
    competitions: 2,
    upcoming: 1,
    entries: 92,
    countries: 2,
    firstYear: 2023,
    lastYear: 2025,
  });
});

it('orders federations by number of competitions, then by name', () => {
  const federation = (name: string) => ({
    federation: { federation_id: name, name, abbreviation: null, country: null },
  });
  const summaries = summarizeFederations([
    competition(federation('SLI')),
    competition(federation('FNSL')),
    competition(federation('DCSV')),
    competition(federation('DCSV')),
  ]);

  expect(summaries.map((summary) => summary.slug)).toEqual(['dcsv', 'fnsl', 'sli']);
});
