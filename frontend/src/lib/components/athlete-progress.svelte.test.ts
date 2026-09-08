import { expect, it } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import AthleteProgress from './athlete-progress.svelte';
import type { AthleteCompetitionSummary } from '$lib/types/athlete';

const competitions: AthleteCompetitionSummary[] = [
  {
    competition_id: 'first',
    competition_name: 'First meet',
    competition_slug: 'first',
    competition_date: '2024-01-01',
    category_name: 'Men -80 kg',
    rank: 2,
    total: '400',
    ris_score: '95',
    ris_source: 'computed',
    status: 'competed',
    event: 'MPDS',
    lifts: [{ movement_name: 'Muscle-up', best_weight: '0', attempts: [] }],
  },
  {
    competition_id: 'second',
    competition_name: 'Second meet',
    competition_slug: 'second',
    competition_date: '2025-01-01',
    category_name: 'Men -87 kg',
    rank: 1,
    total: '450',
    ris_score: '98',
    ris_source: 'reported',
    status: 'competed',
    event: 'MPDS',
    lifts: [{ movement_name: 'Muscle-up', best_weight: '10', attempts: [] }],
  },
];

it('selects competition points and switches metrics without losing a real zero result', async () => {
  render(AthleteProgress, { competitions, formats: ['MPDS'], athleteSlug: 'alex-martin' });
  await expect
    .element(page.getByRole('link', { name: 'Second meet' }))
    .toHaveAttribute('href', '/competitions/second?movement=total&athlete=alex-martin');
  await expect.element(page.getByRole('tooltip')).not.toBeInTheDocument();
  await page.getByRole('button', { name: /First meet/ }).hover();
  await expect
    .element(page.getByRole('link', { name: 'First meet' }))
    .toHaveAttribute('href', '/competitions/first?movement=total&athlete=alex-martin');
  await page.getByRole('combobox', { name: 'Performance', exact: true }).selectOptions('muscleup');
  await page.getByRole('button', { name: /First meet.*0 kg/ }).click();
  await expect
    .element(page.getByRole('button', { name: /First meet.*0 kg/ }))
    .toHaveAttribute('aria-pressed', 'true');
  await expect
    .element(page.getByRole('tooltip').getByRole('link', { name: 'First meet' }))
    .toHaveAttribute('href', '/competitions/first?movement=muscleup&athlete=alex-martin');
  await page.getByRole('combobox', { name: 'Performance', exact: true }).selectOptions('ris');
  await page.getByRole('button', { name: /Second meet/ }).click();
  await expect.element(page.getByText(/Reported RIS/)).toBeVisible();
  await page
    .getByRole('button', { name: /Second meet/ })
    .element()
    .dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }));
  await expect.element(page.getByRole('tooltip')).not.toBeInTheDocument();
});

it('shows a single result as a milestone and handles a missing metric', async () => {
  render(AthleteProgress, { competitions: competitions.slice(0, 1), formats: ['MPDS'] });
  await expect.element(page.getByText(/One result recorded/)).toBeVisible();
  await page.getByRole('combobox', { name: 'Performance', exact: true }).selectOptions('dips');
  await expect.element(page.getByText('No dated Dips results available.')).toBeVisible();
});
