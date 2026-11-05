import { expect, it } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import CompetitionBodyweight, { type BodyweightPoint } from './competition-bodyweight.svelte';
import type { RisFormula } from '#lib/types/ris.js';

const formula: RisFormula = {
  year: 2026,
  is_current: true,
  constants: {
    men: { a: 338, b: 549, k: 0.11354, q: 0.53096, v: 74.777 },
    women: { a: 164, b: 270, k: 0.13776, q: 0.37089, v: 57.855 },
  },
};

const points: BodyweightPoint[] = [
  { id: 'a', name: 'Alex Martin', sex: 'men', bodyweight: 80, total: 600 },
  { id: 'b', name: 'Lea Roux', sex: 'women', bodyweight: 60, total: 100 },
];

it('summarises the field and draws one point per athlete', async () => {
  render(CompetitionBodyweight, { points, formula });
  const chart = page.getByRole('img', { name: /Total by bodyweight for 2 athletes/ });
  await expect.element(chart).toBeVisible();
  expect(chart.element().querySelectorAll('circle[r="3"]')).toHaveLength(2);
  await expect.element(page.getByText('RIS 100, women')).toBeVisible();
});

it('omits the legend for a sex with no athletes', async () => {
  render(CompetitionBodyweight, { points: points.slice(0, 1), formula });
  await expect.element(page.getByText('RIS 100, men')).toBeVisible();
  expect(page.getByText('RIS 100, women').elements()).toHaveLength(0);
});
