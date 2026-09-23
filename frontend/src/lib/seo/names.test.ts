import { describe, expect, it } from 'vitest';
import { athleteTitle, competitionSeoName, competitionTitle, risStanding } from './names';

const meet = (name: string, federation: string, abbreviation: string | null = null) => ({
  name,
  start_date: '2025-10-04',
  federation: { name: federation, abbreviation },
});

describe('competitionSeoName', () => {
  it('adds the federation and the year a bare meet name lacks', () => {
    expect(competitionSeoName(meet('Worlds', 'FinalRep'))).toBe('FinalRep Worlds 2025');
  });

  it('prefers the abbreviation and skips what the name already says', () => {
    expect(
      competitionSeoName(meet('Serbian Open', 'Serbian Streetlifting Federation', 'SSF'))
    ).toBe('SSF Serbian Open 2025');
    expect(competitionSeoName(meet('FNSL Elite 2025', 'FNSL'))).toBe('FNSL Elite 2025');
    expect(competitionSeoName(meet('Ace Squad Cup', 'Ace Squad'))).toBe('Ace Squad Cup 2025');
  });

  it('leaves the year out when the date is unknown', () => {
    expect(competitionSeoName({ ...meet('Worlds', 'FinalRep'), start_date: null })).toBe(
      'FinalRep Worlds'
    );
  });
});

describe('competitionTitle', () => {
  it('names the sport once, and results only once published', () => {
    expect(competitionTitle('FinalRep Worlds 2025', true)).toBe(
      'FinalRep Worlds 2025 Streetlifting results'
    );
    expect(competitionTitle('Streetlifting Belgium Open 2026', false)).toBe(
      'Streetlifting Belgium Open 2026 competition'
    );
  });
});

it('adds the country to an athlete title when known', () => {
  expect(athleteTitle('Aubin Chevillard', 'FR')).toBe(
    'Aubin Chevillard, France - Streetlifting results'
  );
  expect(athleteTitle('Alex Martin', null)).toBe('Alex Martin - Streetlifting results');
});

it('states the RIS standing in the country and worldwide', () => {
  const standing = {
    ris: {
      value: '114.46',
      global: { place: 4, field: 1859 },
      country: { code: 'FR', place: 1, field: 442 },
    },
  };
  expect(risStanding(standing)).toBe(
    'Ranked #1 of 442 in France and #4 of 1,859 worldwide by RIS.'
  );
  expect(risStanding({ pullup: standing.ris })).toBe('');
  expect(risStanding(null)).toBe('');
});

it('describes the global standing without inventing a country', () => {
  expect(
    risStanding({ ris: { value: '114.46', global: { place: 4, field: 1859 }, country: null } })
  ).toBe('Ranked #4 of 1,859 worldwide by RIS.');
});
