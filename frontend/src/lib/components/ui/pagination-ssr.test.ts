import { expect, it, vi } from 'vitest';
import { render } from 'svelte/server';
import Pagination from './pagination.svelte';
import { RankingsTable } from '$lib/state/rankings-table.svelte';

vi.mock('$app/paths', () => ({ resolve: (path: string) => path }));

function links(body: string): URL[] {
  return [...body.matchAll(/<a\s[^>]*href="([^"]+)"/g)].map(
    ([, href]) => new URL(href.replaceAll('&amp;', '&'), 'https://openstreetlifting.org')
  );
}

it('exposes the next rankings page in the initial HTML without JavaScript', () => {
  const table = new RankingsTable({
    basePath: '/',
    initialUrl: new URL('https://openstreetlifting.org/'),
  });
  const { body } = render(Pagination, {
    props: { page: 1, totalPages: 38, pageHref: (target) => table.pageHref(target) },
  });

  expect(links(body).map((url) => url.pathname + url.search)).toEqual([
    '/?page=2',
    '/?page=38',
    '/?page=2',
  ]);
  expect(body).not.toContain('<button');
});

it('preserves ranking filters in crawlable links and drops the focused athlete', () => {
  const table = new RankingsTable({
    basePath: '/',
    includeYear: true,
    initialUrl: new URL(
      'https://openstreetlifting.org/?page=2&movement=pullup&direction=asc&gender=F&category=-63&year=2025&country=FR&federation=FNSL&q=Alex%20%26%20Sam&athlete=alex'
    ),
  });
  const { body } = render(Pagination, {
    props: { page: 2, totalPages: 3, pageHref: (target) => table.pageHref(target) },
  });
  const targets = links(body);
  expect(targets.map((url) => url.searchParams.get('page'))).toEqual([null, null, '3', '3']);
  for (const url of targets) {
    url.searchParams.delete('page');
    expect(Object.fromEntries(url.searchParams)).toEqual({
      movement: 'pullup',
      direction: 'asc',
      gender: 'F',
      category: '-63',
      year: '2025',
      country: 'FR',
      federation: 'FNSL',
      q: 'Alex & Sam',
    });
  }
});

it('links within the competition and omits a next link on the last page', () => {
  const table = new RankingsTable({
    basePath: '/competitions/worlds',
    defaultSort: 'total',
    initialUrl: new URL('https://openstreetlifting.org/competitions/worlds?page=3&gender=M'),
  });
  const { body } = render(Pagination, {
    props: { page: 3, totalPages: 3, pageHref: (target) => table.pageHref(target) },
  });
  expect(links(body).map((url) => url.pathname + url.search)).toEqual([
    '/competitions/worlds?gender=M&page=2',
    '/competitions/worlds?gender=M',
    '/competitions/worlds?gender=M&page=2',
  ]);
});

it('removes navigation targets while pagination is disabled', () => {
  const { body } = render(Pagination, {
    props: { page: 2, totalPages: 3, disabled: true, pageHref: (target) => `/?page=${target}` },
  });
  expect(links(body)).toHaveLength(0);
  expect(body).toContain('aria-current="page"');
});
