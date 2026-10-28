import { expect, it } from 'vitest';
import { render } from 'vitest-browser-svelte';
import { WEIGHT_UNIT_CONTEXT, WeightPreference } from '#lib/state/weight-unit.svelte.js';
import RisCurve from './ris-curve.svelte';
import type { RisPerformance } from '#lib/types/ris.js';

it('keeps RIS in kilograms while converting axes, keyboard announcements and selected weights', async () => {
  const point: RisPerformance = {
    athlete_name: 'Alex Martin',
    athlete_slug: 'alex-martin',
    bodyweight: 80,
    competition_date: '2026-01-01',
    competition_name: 'Euros',
    competition_slug: 'euros',
    participant_id: 'alex',
    total: 500,
  };
  const screen = await render(RisCurve, {
    props: {
      constants: {
        a: 335.5625,
        k: 556.1103380806655,
        b: 0.10289374204365953,
        v: 76.74125992622565,
        q: 0.4973075488457353,
      },
      points: [point],
      label: 'RIS distribution',
    },
    context: new Map([[WEIGHT_UNIT_CONTEXT, new WeightPreference(() => 'lb')]]),
  });
  await expect.element(screen.getByText('Total, lb', { exact: true })).toBeVisible();
  await expect.element(screen.getByText('Bodyweight, lb', { exact: true })).toBeVisible();
  const chart = screen.getByRole('group', { name: 'RIS distribution' });
  chart.element().dispatchEvent(new KeyboardEvent('keydown', { key: 'Home', bubbles: true }));
  await expect
    .element(screen.getByText(/Alex Martin, Euros.*Bodyweight 176.37 lb/))
    .toBeInTheDocument();
  chart.element().dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));
  const selection = screen.getByRole('region', { name: 'Selected performances' });
  await expect.element(selection).toMatchTextContent('176.37 lb');
  await expect.element(selection).toMatchTextContent('1102.31 lb');
  await expect.element(selection).toMatchTextContent('100.35');
  expect(point.bodyweight).toBe(80);
  expect(point.total).toBe(500);
});
