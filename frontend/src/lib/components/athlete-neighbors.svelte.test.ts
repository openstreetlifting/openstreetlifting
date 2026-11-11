import { expect, it, vi } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import AthleteNeighbors from './athlete-neighbors.svelte';
import type { AthleteNeighborhood } from '#lib/types/athlete.js';
import { WEIGHT_UNIT_CONTEXT, WeightPreference } from '#lib/state/weight-unit.svelte.js';

vi.mock('$app/paths', () => ({
  resolve: (path: string, params: Record<string, string>) =>
    path.replace(/\[([^\]]+)\]/g, (_, key) => params[key]),
}));

const row = (slug: string, place: number, value: string, gap: string) => ({
  athlete_id: slug,
  first_name: slug === 'profile' ? 'Alex' : 'Robin',
  last_name: slug,
  slug,
  country: 'FR',
  value,
  gap,
  place,
  field: 20,
  competition_name: 'Test meet',
  competition_slug: 'test-meet',
  competition_date: '2026-01-01',
});
const neighbors: AthleteNeighborhood = {
  category: 'Men -80kg',
  global: [
    row('above', 9, '405', '5'),
    row('profile', 10, '400', '0'),
    row('below', 11, '397.5', '-2.5'),
  ],
  national: [
    row('tied', 1, '400', '0'),
    row('profile', 2, '400', '0'),
    row('below', 3, '397.5', '-2.5'),
  ],
};

it('shows totals without gaps and highlights the profile athlete', async () => {
  render(AthleteNeighbors, { neighbors, athleteSlug: 'profile', country: 'FR' });
  await expect.element(page.getByText('Profile athlete')).toBeVisible();
  await expect
    .element(page.getByRole('link', { name: /Robin above/ }))
    .toHaveAttribute('href', '/en/athletes/above');
  await expect.element(page.getByText(/to match|behind/)).not.toBeInTheDocument();
  const highlighted = page.getByRole('listitem').filter({ hasText: 'Alex profile' });
  await expect.element(highlighted).toHaveAttribute('aria-current', 'true');
  await expect.element(highlighted.getByRole('link', { name: '400 kg' })).toBeVisible();
});

it('puts the category in the heading without an info tooltip', async () => {
  render(AthleteNeighbors, { neighbors, athleteSlug: 'profile', country: 'FR' });
  await expect.element(page.getByRole('heading', { name: 'Current rivals (-80kg)' })).toBeVisible();
  expect(page.getByRole('button', { name: 'Current rivals', exact: true }).elements()).toHaveLength(
    0
  );
});

it('switches to the national neighborhood without leaving the profile', async () => {
  render(AthleteNeighbors, { neighbors, athleteSlug: 'profile', country: 'FR' });
  await page.getByRole('button', { name: 'France', exact: true }).click();
  await expect
    .element(page.getByRole('button', { name: 'France', exact: true }))
    .toHaveAttribute('aria-pressed', 'true');
  await expect.element(page.getByRole('link', { name: /Robin above/ })).not.toBeInTheDocument();
  await expect.element(page.getByRole('link', { name: /Robin tied/ })).toBeVisible();
});

it('explains unavailable national rankings while preserving global context', async () => {
  render(AthleteNeighbors, {
    neighbors: { ...neighbors, national: null },
    athleteSlug: 'profile',
    country: null,
  });
  await page.getByRole('button', { name: 'National', exact: true }).click();
  await expect.element(page.getByText('Country needed for national ranking')).toBeVisible();
  await page.getByRole('button', { name: 'Global', exact: true }).click();
  await expect.element(page.getByRole('link', { name: /Robin above/ })).toBeVisible();
});

it('explains a missing qualifying total', async () => {
  render(AthleteNeighbors, { neighbors: null, athleteSlug: 'profile', country: 'FR' });
  await expect
    .element(
      page.getByText('An All4 total in the latest All4 category is needed to show nearby athletes.')
    )
    .toBeVisible();
});

it('converts totals and the category with the weight preference', async () => {
  const preference = new WeightPreference(() => 'lb');
  render(AthleteNeighbors, {
    props: { neighbors, athleteSlug: 'profile', country: 'FR' },
    context: new Map([[WEIGHT_UNIT_CONTEXT, preference]]),
  });
  await expect.element(page.getByRole('link', { name: /Robin above/ })).toBeVisible();
  expect(linkText(/Robin above/)).toContain('(892.87 lb)');
  await expect
    .element(page.getByRole('heading', { name: 'Current rivals (-176.37 lb)' }))
    .toBeVisible();
});

it('reads as one inline lineup from weaker to stronger with flags and comparison signs', async () => {
  render(AthleteNeighbors, { neighbors, athleteSlug: 'profile', country: 'FR' });
  expect(
    page
      .getByRole('list', { name: 'Athletes' })
      .getByRole('listitem')
      .elements()
      .map((element) => element.querySelector('a')?.getAttribute('href'))
  ).toEqual(['/en/athletes/below', '/en/competitions/test-meet', '/en/athletes/above']);
  expect(page.getByRole('img', { name: 'France' }).elements()).toHaveLength(3);
  const signs = page
    .getByRole('list', { name: 'Athletes' })
    .element()
    .querySelectorAll('li > [aria-hidden="true"]');
  expect([...signs].map((sign) => sign.textContent?.trim())).toEqual(['<', '<']);
});

it('marks equal totals in the national lineup', async () => {
  render(AthleteNeighbors, { neighbors, athleteSlug: 'profile', country: 'FR' });
  await page.getByRole('button', { name: 'France', exact: true }).click();
  const signs = page
    .getByRole('list', { name: 'Athletes' })
    .element()
    .querySelectorAll('li > [aria-hidden="true"]');
  expect([...signs].map((sign) => sign.textContent?.trim())).toEqual(['<', '=']);
});

const linkText = (name: RegExp) =>
  page.getByRole('link', { name }).element().textContent?.replace(/\s+/g, ' ').trim();

it('leads each rival with the signed difference and keeps their total in parentheses', async () => {
  render(AthleteNeighbors, { neighbors, athleteSlug: 'profile', country: 'FR' });
  await expect.element(page.getByRole('link', { name: /Robin above/ })).toBeVisible();
  expect(linkText(/Robin above/)).toBe('Robin above +5 kg (405 kg)');
  expect(linkText(/Robin below/)).toBe('Robin below −2.5 kg (397.5 kg)');
});

it('shows only the total for a rival with the same performance', async () => {
  render(AthleteNeighbors, { neighbors, athleteSlug: 'profile', country: 'FR' });
  await page.getByRole('button', { name: 'France', exact: true }).click();
  await expect.element(page.getByRole('link', { name: /Robin tied/ })).toBeVisible();
  expect(linkText(/Robin tied/)).toBe('Robin tied 400 kg');
});
