import { beforeEach, expect, it, vi } from 'vitest';
import { page as appPage } from '$app/state';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import AthletePage from './+page.svelte';
import type { AthleteCompetitionSummary, AthleteDetail, MetricStanding } from '$lib/types/athlete';

vi.mock('$env/dynamic/public', () => ({ env: {} }));
vi.mock('$app/state', async () => {
  const { SvelteURL } = await import('svelte/reactivity');
  return { page: { url: new SvelteURL('http://localhost/athletes/alex-martin'), state: {} } };
});
vi.mock('$app/navigation', async () => {
  const { page } = await import('$app/state');
  return {
    goto: vi.fn(async (url: string) => {
      page.url.href = new URL(url, page.url).href;
    }),
  };
});

beforeEach(() => {
  appPage.url.search = '';
});

function standing(value: string, weightClass?: string): MetricStanding {
  return {
    value,
    class: weightClass,
    global: { place: 51, field: 120 },
    country: { code: 'FR', place: 3, field: 12 },
  };
}

function athlete(): AthleteDetail {
  return {
    athlete_id: 'test-athlete',
    first_name: 'Alex',
    last_name: 'Martin',
    slug: 'alex-martin',
    gender: 'M',
    country: 'FR',
    profile_picture_url: null,
    instagram_handle: null,
    created_at: '2026-01-01T00:00:00',
    competitions: [],
    personal_records: [],
    total_competitions: 0,
    standing: {
      ris: standing('105.5'),
      total: standing('450', '-80'),
      muscleup: standing('0', '-80'),
      pullup: standing('90', '-80'),
      dips: standing('110', '-80'),
      squat: standing('180', '-80'),
    },
  };
}

function query(link: Element): URLSearchParams {
  return new URL(link.getAttribute('href')!, location.origin).searchParams;
}

it('defaults both cards to RIS with separate global and country pages', async () => {
  render(AthletePage, { data: { athlete: athlete() } });
  await expect.element(page.getByRole('combobox', { name: 'Metric' })).toHaveValue('ris');
  const global = page.getByRole('link', { name: /Global.*#51/ });
  const country = page.getByRole('link', { name: /France.*#3/ });
  await expect.element(global).toBeVisible();
  await expect.element(country).toBeVisible();
  expect(Object.fromEntries(query(global.element()))).toEqual({
    page: '2',
    athlete: 'alex-martin',
  });
  expect(Object.fromEntries(query(country.element()))).toEqual({
    country: 'FR',
    athlete: 'alex-martin',
  });
});

it('switches both cards and their leaderboard filters for every kilogram metric', async () => {
  render(AthletePage, { data: { athlete: athlete() } });
  for (const [metric, label] of [
    ['total', 'Total'],
    ['muscleup', 'Muscle Up'],
    ['pullup', 'Pull Up'],
    ['dips', 'Dips'],
    ['squat', 'Squat'],
  ]) {
    await page.getByRole('combobox', { name: 'Metric' }).selectOptions(metric);
    const global = page.getByRole('link', { name: /Global.*#51/ });
    const country = page.getByRole('link', { name: /France.*#3/ });
    await expect.element(global).not.toHaveTextContent(label);
    await expect.element(global).toHaveTextContent('in category -80');
    await expect.element(country).toHaveTextContent('in category -80');
    expect(Object.fromEntries(query(global.element()))).toEqual({
      movement: metric,
      gender: 'M',
      category: '-80',
      page: '2',
      athlete: 'alex-martin',
    });
    expect(Object.fromEntries(query(country.element()))).toEqual({
      movement: metric,
      gender: 'M',
      category: '-80',
      country: 'FR',
      athlete: 'alex-martin',
    });
    if (metric === 'muscleup') await expect.element(global).not.toHaveTextContent('0 kg');
  }
  await page.getByRole('combobox', { name: 'Metric' }).selectOptions('ris');
  expect(query(page.getByRole('link', { name: /Global.*#51/ }).element()).has('category')).toBe(
    false
  );
});

it('shows unranked cards without links when the selected metric is missing', async () => {
  const partial = athlete();
  partial.standing = { pullup: standing('60', '-80') };
  render(AthletePage, { data: { athlete: partial } });
  await expect.element(page.getByRole('combobox', { name: 'Metric' })).toHaveValue('ris');
  expect(page.getByRole('link', { name: /Global/ }).elements()).toHaveLength(0);
  expect(page.getByRole('link', { name: /France/ }).elements()).toHaveLength(0);
  await expect.element(page.getByText('Not ranked', { exact: true }).first()).toBeVisible();
  await page.getByRole('combobox', { name: 'Metric' }).selectOptions('pullup');
  await expect.element(page.getByRole('link', { name: /Global.*#51/ })).toBeVisible();
});

it('omits the ranking controls when the athlete has no ranked performances', async () => {
  const unranked = athlete();
  unranked.standing = null;
  render(AthletePage, { data: { athlete: unranked } });
  expect(page.getByRole('combobox', { name: 'Metric' }).elements()).toHaveLength(0);
  await expect.element(page.getByRole('heading', { name: 'Competition history' })).toBeVisible();
});

it('renders multiple divisions from the same meet and distinguishes absent, bombed and zero lifts', async () => {
  const lifter = athlete();
  const result: AthleteCompetitionSummary = {
    competition_id: 'same-meet',
    competition_name: 'One Meet',
    competition_slug: 'one-meet',
    competition_date: '2026-01-01',
    category_name: '-80',
    rank: 1,
    total: '60',
    ris_score: null,
    ris_source: null,
    status: 'competed',
    event: 'MP',
    lifts: [
      {
        movement_name: 'Muscle-up',
        best_weight: '0',
        attempts: [{ attempt_number: 1, weight: '0', is_successful: true }],
      },
      { movement_name: 'Pull-up', best_weight: '60', attempts: [] },
    ],
  };
  lifter.competitions = [
    { ...result, division: 'Open' },
    {
      ...result,
      division: 'Junior',
      lifts: [
        {
          movement_name: 'Muscle-up',
          best_weight: null,
          attempts: [{ attempt_number: 1, weight: '20', is_successful: false }],
        },
        { movement_name: 'Pull-up', best_weight: '60', attempts: [] },
      ],
    },
  ];
  lifter.total_competitions = 1;
  render(AthletePage, { data: { athlete: lifter } });
  await expect.element(page.getByRole('cell', { name: 'Open', exact: true })).toBeVisible();
  await expect.element(page.getByRole('cell', { name: 'Junior', exact: true })).toBeVisible();
  expect(
    page.getByRole('table').getByRole('link', { name: 'One Meet', exact: true }).elements()
  ).toHaveLength(2);
  await expect.element(page.getByTitle('Attempt 1: 0 kg, good lift')).toHaveTextContent('0');
  await expect.element(page.getByTitle('No successful muscle up')).toBeVisible();
  expect(page.getByRole('columnheader', { name: 'Dips' }).elements()).toHaveLength(0);
  expect(page.getByRole('columnheader', { name: 'Squat' }).elements()).toHaveLength(0);
});
