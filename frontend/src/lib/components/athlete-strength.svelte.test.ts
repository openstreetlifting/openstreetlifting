import { expect, it } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import AthleteStrength from './athlete-strength.svelte';
import type { StrengthProfile } from '$lib/types/athlete';

const profile: StrengthProfile = {
  category: 'Men -80kg',
  lifts: [
    { movement_name: 'Muscle-up', value: '0', percentile: 0, field: 10 },
    { movement_name: 'Pull-up', value: '80', percentile: 50, field: 20 },
    { movement_name: 'Dips', value: '100', percentile: 75, field: 15 },
    { movement_name: 'Squat', value: '180', percentile: 100, field: 12 },
  ],
};

it('plots all four comparisons, including a real zero percentile, and shows peer details on hover', async () => {
  render(AthleteStrength, { profile });
  await page.getByRole('button', { name: 'Muscle-up: percentile 0' }).hover();
  await expect.element(page.getByRole('tooltip')).toHaveTextContent('Muscle-up · 0 kg');
  await expect
    .element(page.getByRole('tooltip'))
    .toHaveTextContent('Compared with 10 other athletes');
});

it('does not turn an unscored lift into a zero point', async () => {
  render(AthleteStrength, {
    profile: {
      ...profile,
      lifts: profile.lifts.map((lift) =>
        lift.movement_name === 'Dips' ? { ...lift, value: null, percentile: null } : lift
      ),
    },
  });
  await expect.element(page.getByText('Not enough data to score all four lifts.')).toBeVisible();
  expect(page.getByRole('button', { name: /: percentile/ }).elements()).toHaveLength(3);
});

it('explains why there is no category comparison before a completed competition', async () => {
  render(AthleteStrength, { profile: null });
  await expect
    .element(
      page.getByText('A completed competition is needed to establish a comparison category.')
    )
    .toBeVisible();
});
