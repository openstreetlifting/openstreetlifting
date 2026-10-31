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
                category: { name: 'Men', gender: 'M', weight_class: '', division: null },
                participants: [],
              },
            ],
      previous_editions: [
        { slug: 'previous-2025', name: 'Previous edition', start_date: '2025-01-01' },
        { slug: 'original-2023', name: 'Original name', start_date: '2023-06-15' },
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
  'links previous editions on a %s page with localized dates and clean URLs',
  async (status) => {
    render(CompetitionPage, {
      props: { data: data(status) },
      context: new Map([[I18N_CONTEXT, new I18n(() => 'fr')]]),
    });
    const nav = page.getByRole('navigation', { name: 'Éditions précédentes' });
    await expect.element(nav).toBeVisible();
    const links = nav.getByRole('link').elements();
    expect(links.map((link) => link.getAttribute('href'))).toEqual([
      '/fr/competitions/previous-2025',
      '/fr/competitions/original-2023',
    ]);
    await expect
      .element(nav.getByRole('link').first())
      .toHaveTextContent('Previous edition · 1 janvier 2025');
  }
);

it('omits the section for a competition with no previous editions', async () => {
  const first = data();
  first.competition.previous_editions = [];
  render(CompetitionPage, { data: first });
  expect(page.getByRole('navigation', { name: 'Previous editions' }).elements()).toHaveLength(0);
  await expect
    .element(page.getByRole('heading', { name: /Current edition/, level: 1 }))
    .toBeVisible();
});
