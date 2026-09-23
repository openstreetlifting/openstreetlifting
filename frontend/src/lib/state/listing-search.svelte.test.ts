import { beforeEach, expect, it, vi } from 'vitest';
import { goto } from '$app/navigation';
import { ListingSearch } from './listing-search.svelte';
import { RankingsTable } from './rankings-table.svelte';

vi.mock('$app/navigation', () => ({ goto: vi.fn() }));
vi.mock('$app/paths', () => ({ resolve: (path: string) => path }));

const url = (path: string) => new URL(path, 'http://localhost');

beforeEach(() => vi.resetAllMocks());

it('keeps newer typing when a slow search completes, then applies the newer query', async () => {
  const search = new ListingSearch(url('/'));
  let finish!: () => void;
  vi.mocked(goto).mockImplementation(() => new Promise<void>((resolve) => (finish = resolve)));

  search.value = 'mar';
  const first = search.navigate('/?q=mar');
  search.value = 'martin';
  search.sync(url('/?q=mar'), 'goto');
  finish();
  await first;
  expect(search.value).toBe('martin');
  expect(search.applied).toBe('mar');

  const second = search.navigate('/?q=martin');
  search.sync(url('/?q=martin'), 'goto');
  finish();
  await second;
  expect(search.applied).toBe('martin');
  expect(search.value).toBe('martin');
  expect(goto).toHaveBeenLastCalledWith('/?q=martin', {
    replaceState: true,
    keepFocus: true,
    noScroll: true,
  });
});

it.each(['popstate', 'link', 'goto'] as const)(
  'restores the input for an external %s navigation, including clearing it',
  (type) => {
    const search = new ListingSearch(url('/?q=martin'));
    search.value = 'unfinished';
    search.sync(url('/?q=alex'), type);
    expect(search.value).toBe('alex');
    search.sync(url('/'), type);
    expect(search.value).toBe('');
    expect(search.applied).toBe('');
  }
);

it('does not let a cancelled search clear the identity of its replacement', async () => {
  const search = new ListingSearch(url('/'));
  const finishes: (() => void)[] = [];
  vi.mocked(goto).mockImplementation(() => new Promise<void>((resolve) => finishes.push(resolve)));
  const first = search.navigate('/?q=mar');
  const second = search.navigate('/?q=martin');
  finishes[0]();
  await first;
  search.value = 'martinez';
  search.sync(url('/?q=martin'), 'goto');
  finishes[1]();
  await second;
  expect(search.value).toBe('martinez');
  expect(search.applied).toBe('martin');
});

it('restores Back navigation even when it matches an outstanding search', async () => {
  const search = new ListingSearch(url('/'));
  let finish!: () => void;
  vi.mocked(goto).mockImplementation(() => new Promise<void>((resolve) => (finish = resolve)));
  const pending = search.navigate('/?q=mar');
  search.value = 'martin';
  search.sync(url('/?q=mar'), 'popstate');
  finish();
  await pending;
  expect(search.value).toBe('mar');
});

it('pages through the applied results while a new search is still a draft', async () => {
  const table = new RankingsTable({ basePath: '/', initialUrl: url('/?q=alex&country=FR') });
  table.searchFilter = 'martin';
  expect(table.pageHref(2)).toBe('/?country=FR&q=alex&page=2');
  await table.handleFilterChange();
  expect(goto).toHaveBeenCalledWith('/?country=FR&q=martin', expect.anything());
});

it('clears the query and filters together', async () => {
  const table = new RankingsTable({ basePath: '/', initialUrl: url('/?q=alex&country=FR') });
  await table.clearFilters();
  expect(table.searchFilter).toBe('');
  expect(table.countryFilter).toBeNull();
  expect(goto).toHaveBeenCalledWith('/', expect.anything());
});
