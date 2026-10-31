import { expect, it, vi } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import CompetitionPage from './+page.svelte';
import type { PageData } from './$types';
import { I18N_CONTEXT, I18n } from '#lib/i18n/index.js';

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
vi.mock('$app/state', () => ({
  page: { url: new URL('http://localhost/fr/competitions/current?page=2&gender=M'), state: {} },
  navigating: {},
}));
vi.mock('$app/navigation', () => ({
  afterNavigate: vi.fn(),
  beforeNavigate: vi.fn(),
  goto: vi.fn(),
}));

function data(status: 'upcoming' | 'completed' = 'upcoming'): PageData {
  return {
    locale: 'fr',
    competition: {
      competition_id: 'meet',
      created_at: '2026-01-01T00:00:00',
      federation_id: 'organizer',
      event_code: null,
      lifter_count: null,
      name: 'Current edition',
      slug: 'current',
      status,
      scoring: null,
      start_date: '2026-11-01',
      end_date: '2026-11-01',
      city: null,
      venue: null,
      region: null,
      country: 'FR',
      federation: {
        federation_id: 'organizer',
        name: 'Organizer',
        abbreviation: null,
        country: 'FR',
      },
      movements: [],
      categories:
        status === 'upcoming'
          ? []
          : [
              {
                category: { name: 'Men -80kg', gender: 'M', weight_class: '-80kg', division: null },
                participants: [],
              },
            ],
      editions: [
        { slug: 'original-2023', name: 'Original name', start_date: '2023-06-15' },
        { slug: 'previous-2025', name: 'Previous edition', start_date: '2025-01-01' },
        { slug: 'current', name: 'Current edition', start_date: '2026-11-01' },
      ],
    },
    ris: 'not-contested',
    classes: [],
    countries: [],
    initialRankings: [],
    pagination: { page: 1, page_size: 50, total_items: 0, total_pages: 0 },
  };
}

it.each(['upcoming', 'completed'] as const)(
  'links previous editions on a %s page with compact years and clean URLs',
  async (status) => {
    render(CompetitionPage, {
      props: { data: data(status) },
      context: new Map([[I18N_CONTEXT, new I18n(() => 'fr')]]),
    });
    const nav = page.getByRole('navigation', { name: 'Éditions' });
    await expect.element(nav).toBeVisible();
    const links = nav.getByRole('link').elements();
    expect(links.map((link) => link.getAttribute('href'))).toEqual([
      '/fr/competitions/original-2023',
      '/fr/competitions/previous-2025',
    ]);
    await expect.element(nav.getByRole('link').first()).toHaveTextContent('2023');
    await expect
      .element(nav.getByRole('link').first())
      .toHaveAccessibleName('2023 · Original name');
    await expect
      .element(nav.getByRole('link').first())
      .toHaveAttribute('title', 'Original name · 15 juin 2023');
  }
);

it('adds localized short dates when previous editions share a year', async () => {
  const repeated = data();
  repeated.competition.editions = [
    { slug: 'original', name: 'Original edition', start_date: '2023-06-15' },
    { slug: 'spring', name: 'Revolutions', start_date: '2025-03-29' },
    { slug: 'autumn', name: 'Resurrections', start_date: '2025-10-18' },
    { slug: 'current', name: 'Current edition', start_date: '2026-11-01' },
  ];
  render(CompetitionPage, {
    props: { data: repeated },
    context: new Map([[I18N_CONTEXT, new I18n(() => 'fr')]]),
  });
  const links = page.getByRole('navigation', { name: 'Éditions' }).getByRole('link');
  expect(links.elements().map((link) => link.textContent?.trim())).toEqual([
    '2023',
    '2025 · 29 mars',
    '2025 · 18 oct.',
  ]);
});

it('distinguishes a previous edition in the current year and updates after navigation', async () => {
  const current = data();
  current.competition.start_date = '2025-10-18';
  current.competition.end_date = '2025-10-18';
  current.competition.editions[2].start_date = '2025-10-18';
  const screen = await render(CompetitionPage, { data: current });
  const links = page.getByRole('navigation', { name: 'Editions' }).getByRole('link');
  await expect.element(links.last()).toHaveTextContent('2025 · 1 Jan');
  const earlier = data();
  earlier.competition.start_date = '2025-01-01';
  earlier.competition.end_date = '2025-01-01';
  earlier.competition.editions = [
    { slug: 'original-2023', name: 'Original name', start_date: '2023-06-15' },
    { slug: 'current', name: 'Current edition', start_date: '2025-01-01' },
  ];
  await screen.rerender({ data: earlier });
  expect(links.elements()).toHaveLength(1);
  await expect.element(links.first()).toHaveTextContent('2023');
  await expect.element(links.first()).toHaveAttribute('href', '/en/competitions/original-2023');
});

it('links later editions and marks the current edition without a link', async () => {
  const earlier = data('completed');
  earlier.competition.slug = 'previous-2025';
  earlier.competition.name = 'Previous edition';
  earlier.competition.start_date = '2025-01-01';
  earlier.competition.end_date = '2025-01-01';
  render(CompetitionPage, { data: earlier });
  const nav = page.getByRole('navigation', { name: 'Editions' });
  const next = nav.getByRole('link', { name: '2026 · Current edition' });
  await expect.element(next).toHaveAttribute('href', '/en/competitions/current');
  expect(nav.element().querySelector('[aria-current="page"]')?.textContent?.trim()).toBe('2025');
  expect(nav.getByRole('link').elements()).toHaveLength(2);
});

it.each([false, true])(
  'omits the section with no other editions (assigned: %s)',
  async (assigned) => {
    const first = data();
    first.competition.editions = assigned
      ? [{ slug: 'current', name: 'Current edition', start_date: '2026-11-01' }]
      : [];
    render(CompetitionPage, { data: first });
    expect(page.getByRole('navigation', { name: 'Editions' }).elements()).toHaveLength(0);
    await expect
      .element(page.getByRole('heading', { name: /Current edition/, level: 1 }))
      .toBeVisible();
  }
);
