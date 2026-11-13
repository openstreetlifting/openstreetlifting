import { beforeEach, expect, it, vi } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import { goto } from '$app/navigation';
import RecordsPage from './+page.svelte';
import { WEIGHT_UNIT_CONTEXT, WeightPreference } from '#lib/state/weight-unit.svelte.js';
import type { RecordEntry, RecordGroup } from '#lib/types/records.js';
import type { PageData } from './$types';

vi.mock('$app/env/public', () => ({
  PUBLIC_SITE_URL: 'https://openstreetlifting.org',
  PUBLIC_ENVIRONMENT: '',
}));
vi.mock('$app/paths', () => ({
  resolve: (path: string, params?: Record<string, string>) =>
    path.startsWith('/')
      ? path.replace(/\[([^\]]+)\]/g, (_, key) => params?.[key] ?? '')
      : `/${path}`,
}));
vi.mock('$app/state', async () => {
  const { SvelteURL } = await import('svelte/reactivity');
  return { page: { url: new SvelteURL('http://localhost/en/records'), state: {} } };
});
vi.mock('$app/navigation', () => ({ goto: vi.fn() }));

beforeEach(() => {
  vi.mocked(goto).mockReset();
});

function entry(rank: number, last_name: string, weight: string): RecordEntry {
  return {
    rank,
    athlete: {
      first_name: 'Ana',
      last_name,
      slug: `ana-${last_name.toLowerCase()}`,
      country: 'FR',
    },
    weight,
    date: '2026-01-01',
    competition: { name: 'Euros', slug: 'euros-2026' },
  };
}

function group(movement: RecordGroup['movement'], weight_class: string | null): RecordGroup {
  return {
    movement,
    gender: 'M',
    weight_class,
    entries: [entry(1, 'First', '32'), entry(2, 'Second', '27.5')],
  };
}

function data(overrides: Partial<PageData> = {}): PageData {
  return {
    locale: 'en',
    gender: 'M',
    country: null,
    federation: null,
    year: null,
    countries: ['FR', 'IT'],
    federations: ['FinalRep', 'FNSL'],
    years: [2026, 2025],
    groups: [group('muscleup', null), group('muscleup', 'F-57'), group('squat', null)],
    ...overrides,
  } as PageData;
}

function classLabels(movement: string): string[] {
  const table = page.getByRole('table', { name: `${movement} records` }).element();
  return [...table.querySelectorAll('th[scope="rowgroup"]')].map(
    (cell) => cell.textContent?.replace(/\s+/g, ' ').trim() ?? ''
  );
}

it('lists each movement in order with the open group first', async () => {
  render(RecordsPage, { data: data() });
  await expect
    .element(page.getByRole('heading', { name: 'OpenStreetlifting records', level: 1 }))
    .toBeVisible();
  expect(
    page
      .getByRole('heading', { level: 2 })
      .elements()
      .map((heading) => heading.textContent?.trim())
  ).toEqual(['Muscle-up', 'Squat']);
  expect(classLabels('Muscle-up')).toEqual(['All Classes', '-57kg']);

  const muscleup = page.getByRole('table', { name: 'Muscle-up records' });
  await expect
    .element(muscleup.getByRole('link', { name: /Ana First/ }).first())
    .toHaveAttribute('href', '/en/athletes/ana-first');
  await expect
    .element(muscleup.getByRole('link', { name: 'Euros' }).first())
    .toHaveAttribute('href', '/en/competitions/euros-2026');
});

it('converts weights and classes to pounds', async () => {
  render(RecordsPage, {
    props: { data: data({ groups: [group('muscleup', 'F-57')] }) },
    context: new Map([[WEIGHT_UNIT_CONTEXT, new WeightPreference(() => 'lb')]]),
  });
  await expect.element(page.getByRole('columnheader', { name: 'lb' })).toBeVisible();
  expect(classLabels('Muscle-up')).toEqual(['-125.66 lb']);
});

it('names the country when one is selected', async () => {
  render(RecordsPage, { data: data({ country: 'FR' }) });
  await expect
    .element(page.getByRole('heading', { name: /France records/, level: 1 }))
    .toBeVisible();
});

it('switches to women while keeping every scope filter', async () => {
  render(RecordsPage, { data: data({ country: 'IT', year: 2025 }) });
  await expect
    .element(page.getByRole('button', { name: 'Men' }))
    .toHaveAttribute('aria-pressed', 'true');
  await page.getByRole('button', { name: 'Women' }).click();
  expect(goto).toHaveBeenCalledWith('/en/records?gender=F&country=IT&year=2025', {
    reset: false,
  });
});

it('narrows to a year', async () => {
  render(RecordsPage, { data: data() });
  await page.getByRole('combobox', { name: 'Year' }).selectOptions('2025');
  expect(goto).toHaveBeenCalledWith('/en/records?year=2025', { reset: false });
});

it('narrows to a federation', async () => {
  render(RecordsPage, { data: data() });
  await page.getByRole('combobox', { name: 'Federation' }).selectOptions('FNSL');
  expect(goto).toHaveBeenCalledWith('/en/records?federation=FNSL', { reset: false });
});

it('clears every scope filter and keeps the sex', async () => {
  render(RecordsPage, { data: data({ gender: 'F', country: 'FR', federation: 'FNSL' }) });
  await page.getByRole('button', { name: 'Clear' }).click();
  expect(goto).toHaveBeenCalledWith('/en/records?gender=F', { reset: false });
});

it('hides the clear button for worldwide records', async () => {
  render(RecordsPage, { data: data() });
  await expect.element(page.getByRole('combobox', { name: 'Year' })).toBeVisible();
  expect(page.getByRole('button', { name: 'Clear' }).elements()).toHaveLength(0);
});

it('explains when nothing matches', async () => {
  render(RecordsPage, { data: data({ groups: [] }) });
  await expect.element(page.getByText('No records for this selection yet.')).toBeVisible();
  expect(page.getByRole('table').elements()).toHaveLength(0);
});
