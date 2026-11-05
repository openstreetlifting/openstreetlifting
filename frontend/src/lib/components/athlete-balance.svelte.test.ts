import { expect, it } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import AthleteBalance from './athlete-balance.svelte';
import type { LiftBalance } from '#lib/types/athlete.js';

const balance: LiftBalance = {
  category: 'Men -80kg',
  total: '400',
  competition_name: 'Euros',
  competition_slug: 'euros',
  competition_date: '2024-11-02',
  field: 12,
  lifts: [
    { movement_name: 'Muscle-up', value: '20', share: 5, class_share: 6, class_spread: 1 },
    { movement_name: 'Pull-up', value: '70', share: 17.5, class_share: 17.5, class_spread: 2 },
    { movement_name: 'Dips', value: '110', share: 27.5, class_share: 28, class_spread: 2 },
    { movement_name: 'Squat', value: '200', share: 50, class_share: 48.5, class_spread: 4 },
  ],
};

it('labels each axis with its difference from the class average', async () => {
  render(AthleteBalance, { balance });
  const chart = page.getByRole('group', { name: /Lift balance compared/ });
  await expect.element(chart).toBeVisible();
  expect(chart.element().textContent?.replace(/\s+/g, ' ').trim()).toBe(
    'Muscle-up -1.0% Pull-up 0.0% Squat +1.5% Dips -0.5%'
  );
  await expect.element(page.getByText('Reference (-80kg)')).toBeVisible();
  const squat = page.getByRole('definition').nth(2);
  await expect
    .element(squat)
    .toHaveTextContent('+1.5% Athlete: 50.0% of total Reference: 48.5% of total');
  await expect.element(page.getByText(/Based on/)).not.toBeVisible();
});

it('shows the share and class average on hover', async () => {
  render(AthleteBalance, { balance });
  await page.getByRole('button', { name: 'Squat: 50.0% of total, reference 48.5%' }).hover();
  await expect.element(page.getByRole('tooltip')).toMatchTextContent(/Squat · 200 kg/);
  await expect.element(page.getByRole('tooltip')).toMatchTextContent(/48\.5%/);
});

it('lists shares without a comparison when the class is too small', async () => {
  render(AthleteBalance, { balance: { ...balance, field: 9 } });
  await expect
    .element(page.getByText('Too few athletes in this weight class to compare.'))
    .toBeVisible();
  expect(page.getByRole('group', { name: /Lift balance compared/ }).elements()).toHaveLength(0);
  await expect.element(page.getByRole('definition').first()).toHaveTextContent('5.0% of total');
});

it('explains when no four-lift competition exists', async () => {
  render(AthleteBalance, { balance: null });
  await expect
    .element(
      page.getByText('A completed competition with all four lifts is needed to show lift balance.')
    )
    .toBeVisible();
});
