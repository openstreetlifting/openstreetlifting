import { expect, it } from 'vitest';
import { render } from 'svelte/server';
import AthleteProgress from './athlete-progress.svelte';
import AthleteStrength from './athlete-strength.svelte';
import { MOVEMENTS } from '#lib/types/enums.js';
import { WEIGHT_UNIT_CONTEXT, WeightPreference } from '#lib/state/weight-unit.svelte.js';

it.each(['kg', 'lb'] as const)(
  'renders performance geometry and %s weights in the initial HTML without client effects',
  (unit) => {
    const { body } = render(AthleteProgress, {
      context: new Map([[WEIGHT_UNIT_CONTEXT, new WeightPreference(() => unit)]]),
      props: {
        formats: ['MPDS'],
        competitions: [
          {
            competition_id: 'meet',
            competition_name: 'Euros',
            competition_slug: 'euros',
            competition_date: '2024-11-02',
            category_name: 'Men -80kg',
            division: null,
            rank: 1,
            total: '400',
            ris_score: null,
            ris_source: null,
            status: 'competed',
            event: 'MPDS',
            lifts: [],
          },
        ],
      },
    });
    expect(body).toContain('aria-label="Total over time"');
    expect(body).toContain(unit === 'kg' ? '400 kg' : '881.85 lb');
    expect(body).toMatch(/<circle cx="[\d.-]+" cy="[\d.-]+" r="4"/);
    expect(body).not.toMatch(/(?:cx|cy|d)="[^"]*NaN/);
  }
);

it('renders the percentile polygon in the initial HTML with finite coordinates', () => {
  const { body } = render(AthleteStrength, {
    props: {
      profile: {
        category: 'Men -80kg',
        lifts: MOVEMENTS.map((movement_name) => ({
          movement_name,
          value: '50',
          percentile: 75,
          field: 10,
        })),
      },
    },
  });
  expect(body).toContain('Muscle-up: percentile 75');
  expect(body).toMatch(/<path d="M[\d.-]+,[\d.-]+/);
  expect(body).not.toMatch(/(?:cx|cy|d)="[^"]*NaN/);
});
