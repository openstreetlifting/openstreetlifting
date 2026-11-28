import { expect, it, vi } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import AthleteProgress from './athlete-progress.svelte';
import type { AthleteCompetitionSummary } from '#lib/types/athlete.js';
import { liftProgress } from '#lib/utils/athlete-progress.js';
import { WEIGHT_UNIT_CONTEXT, WeightPreference } from '#lib/state/weight-unit.svelte.js';

vi.mock('$app/paths', () => ({
  resolve: (path: string, params?: Record<string, string>) =>
    path.startsWith('/')
      ? path.replace(/\[([^\]]+)\]/g, (_, key) => params?.[key] ?? '')
      : `/${path}`,
}));

const meet = (
  id: string,
  date: string,
  lifts: Record<string, string | null>,
  event = 'MPDS'
): AthleteCompetitionSummary => ({
  competition_id: id,
  competition_name: id,
  competition_slug: id,
  competition_date: date,
  category_name: 'Men -80 kg',
  division: null,
  rank: 1,
  total: null,
  total_record: null,
  ris_score: null,
  ris_source: null,
  status: 'competed',
  event,
  lifts: Object.entries(lifts).map(([movement_name, best_weight]) => ({
    movement_name: movement_name as AthleteCompetitionSummary['lifts'][number]['movement_name'],
    best_weight,
    record: null,
    attempts: [],
  })),
});

const progress = liftProgress([
  meet('First meet', '2024-01-01', { 'Muscle-up': '0', 'Pull-up': '40' }),
  meet('Second meet', '2025-01-01', { 'Muscle-up': '10', 'Pull-up': '45', Dips: '90' }, 'PD'),
]);

it('names every plotted lift in the legend, with no metric selector', async () => {
  render(AthleteProgress, { progress });
  const legend = page.getByRole('list');
  await expect.element(legend).toBeVisible();
  expect(
    legend
      .getByRole('listitem')
      .elements()
      .map((item) => item.textContent?.trim())
  ).toEqual(['Muscle Up', 'Pull Up', 'Dips']);
  expect(page.getByRole('combobox').elements()).toHaveLength(0);
});

it('shows the meet name and the hovered lift value, linked to the meet sorted by that lift', async () => {
  render(AthleteProgress, { progress, athleteSlug: 'alex-martin' });
  await expect.element(page.getByRole('tooltip')).not.toBeInTheDocument();
  await page.getByRole('button', { name: /Second meet.*Pull Up 45 kg/ }).hover();
  const tooltip = page.getByRole('tooltip');
  await expect.element(tooltip).toHaveTextContent('Second meet Pull Up 45 kg');
  await expect
    .element(tooltip.getByRole('link', { name: 'Second meet' }))
    .toHaveAttribute('href', '/en/competitions/Second meet?movement=pullup&athlete=alex-martin');
  await page
    .getByRole('button', { name: /Second meet.*Pull Up/ })
    .element()
    .dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }));
  await expect.element(tooltip).not.toBeInTheDocument();
});

it('converts plotted weights to pounds and keeps a real zero', async () => {
  render(AthleteProgress, {
    props: { progress },
    context: new Map([[WEIGHT_UNIT_CONTEXT, new WeightPreference(() => 'lb')]]),
  });
  await expect
    .element(page.getByRole('button', { name: /Second meet.*Dips 198.42 lb/ }))
    .toBeInTheDocument();
  await expect
    .element(page.getByRole('button', { name: /First meet.*Muscle Up 0 lb/ }))
    .toBeInTheDocument();
});
