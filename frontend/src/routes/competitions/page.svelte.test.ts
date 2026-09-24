import { beforeEach, expect, it, vi } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import { page as appPage } from '$app/state';
import { goto } from '$app/navigation';
import CompetitionsPage from './+page.svelte';
import type { PageData } from './$types';
import { FORMAT_MOVEMENTS } from '$lib/utils/competition-format';

const navigation = vi.hoisted(() => ({ after: (() => {}) as (event: { type: string }) => void }));
vi.mock('$env/dynamic/public', () => ({ env: {} }));
vi.mock('$app/paths', () => ({ resolve: (path: string) => path }));
vi.mock('$app/state', async () => {
  const { SvelteURL } = await import('svelte/reactivity');
  return {
    page: { url: new SvelteURL('http://localhost/competitions'), state: {} },
    navigating: {},
  };
});
vi.mock('$app/navigation', async () => {
  const { page } = await import('$app/state');
  return {
    beforeNavigate: vi.fn(),
    afterNavigate: (callback: typeof navigation.after) => {
      navigation.after = callback;
    },
    goto: vi.fn(async (url: string) => {
      page.url.href = new URL(url, page.url).href;
    }),
  };
});

function data(event = ''): PageData {
  return {
    competitions: ['PD', ''].map((format, i) => ({
      competition_id: String(i),
      slug: `meet-${i}`,
      name: `Meet ${i}`,
      created_at: '',
      status: 'completed',
      country: 'FR',
      city: null,
      region: null,
      start_date: '2026-01-01',
      end_date: '2026-01-01',
      federation: { federation_id: 'test', name: 'Test', abbreviation: null, country: 'FR' },
      movements: FORMAT_MOVEMENTS.filter(({ code }) => format.includes(code)).map(
        ({ code, label }) => ({
          code,
          movement_name: label,
          display_order: null,
        })
      ),
    })),
    status: 'completed',
    country: 'FR',
    year: 2026,
    federation: 'Test',
    q: 'meet',
    event,
    pagination: { page: 2, page_size: 50, total_items: 101, total_pages: 3 },
    facets: { formats: ['PD', 'MPDS'], countries: ['FR'], years: [2026], federations: ['Test'] },
    counts: { completed: 101, upcoming: 2 },
  } as PageData;
}

beforeEach(() => {
  vi.clearAllMocks();
  appPage.url.href =
    'http://localhost/competitions?country=FR&year=2026&federation=Test&q=meet&page=2';
});

it('selects an exact format in either click order, preserves filters, and resets pagination', async () => {
  render(CompetitionsPage, { data: data() });
  await expect.element(page.getByText('Format', { exact: true })).toBeVisible();
  await expect.element(page.getByRole('cell', { name: 'PD', exact: true })).toBeVisible();
  await expect.element(page.getByRole('cell', { name: '—', exact: true })).toBeVisible();
  await page.getByText('All formats', { exact: true }).click();
  await page.getByRole('checkbox', { name: 'Dips', exact: true }).click();
  await page.getByRole('checkbox', { name: 'Pull-up', exact: true }).click();
  expect(Object.fromEntries(appPage.url.searchParams)).toEqual({
    q: 'meet',
    federation: 'Test',
    event: 'PD',
    country: 'FR',
    year: '2026',
  });
  await page.getByRole('button', { name: 'Clear movements' }).click();
  expect(appPage.url.searchParams.has('event')).toBe(false);
  expect(appPage.url.searchParams.get('country')).toBe('FR');
  expect(goto).toHaveBeenCalled();
});

it('restores selection on history navigation and retains format in paging and tab links', async () => {
  appPage.url.searchParams.set('event', 'PD');
  const screen = await render(CompetitionsPage, { data: data('PD') });
  const links = page.getByRole('link', { name: 'Next page' }).elements();
  expect(links.length).toBeGreaterThan(0);
  expect(
    links.every(
      (link) =>
        new URL(link.getAttribute('href')!, location.origin).searchParams.get('event') === 'PD'
    )
  ).toBe(true);
  await page.getByRole('button', { name: /Upcoming/ }).click();
  expect(appPage.url.searchParams.get('event')).toBe('PD');
  expect(appPage.url.searchParams.get('status')).toBe('upcoming');
  expect(appPage.url.searchParams.has('page')).toBe(false);
  appPage.url.href = 'http://localhost/competitions?event=MS';
  await screen.rerender({ data: data('MS') });
  navigation.after({ type: 'popstate' });
  await page.getByText('Muscle-up, Squat', { exact: true }).click();
  await expect
    .element(page.getByRole('checkbox', { name: 'Muscle-up', exact: true }))
    .toBeChecked();
  await expect.element(page.getByRole('checkbox', { name: 'Squat', exact: true })).toBeChecked();
  await expect
    .element(page.getByRole('checkbox', { name: 'Pull-up', exact: true }))
    .not.toBeChecked();
});

it('keeps every movement available with no matches and uses the shared reset state', async () => {
  const empty = data('MS');
  empty.competitions = [];
  empty.status = 'upcoming';
  empty.pagination = { page: 1, page_size: 50, total_items: 0, total_pages: 0 };
  render(CompetitionsPage, { data: empty });
  await expect.element(page.getByRole('status')).toMatchTextContent('Oops, no competitions found.');
  await expect
    .element(page.getByRole('link', { name: 'Clear search & filters' }))
    .toHaveAttribute('href', '/competitions?status=upcoming');
  await page.getByText('Muscle-up, Squat', { exact: true }).click();
  expect(page.getByRole('checkbox').elements()).toHaveLength(4);
  await expect.element(page.getByRole('checkbox', { name: 'Pull-up', exact: true })).toBeEnabled();
});
