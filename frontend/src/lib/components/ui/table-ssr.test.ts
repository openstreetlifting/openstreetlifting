import { beforeEach, expect, it, vi } from 'vitest';
import { createRawSnippet } from 'svelte';
import { render } from 'svelte/server';
import { page } from '$app/state';
import Table from './table.svelte';

vi.mock('$app/state', () => ({
  page: { url: new URL('https://openstreetlifting.org/countries/fr') },
}));

const rows = Array.from({ length: 101 }, (_, i) => i + 1);
const head = createRawSnippet(() => ({ render: () => '<th>Competition</th>' }));
const body = createRawSnippet<[number[]]>((getRows) => ({
  render: () =>
    getRows()
      .map((row) => `<tr><td>Result ${row}</td></tr>`)
      .join(''),
}));

beforeEach(() => {
  page.url.href = 'https://openstreetlifting.org/countries/fr';
});

it('renders 50 rows and identical top and bottom navigation before hydration', () => {
  const { body: html } = render(Table<number>, {
    props: { rows, head, body, pageParam: 'competitions_page' },
  });
  expect(html.match(/<td>/g)).toHaveLength(50);
  expect(html).toContain('Result 50</td>');
  expect(html).not.toContain('Result 51</td>');
  expect(html.match(/aria-label="Pagination"/g)).toHaveLength(2);
  expect(html.match(/Page 1 of 3/g)).toHaveLength(2);
  expect(html.match(/href="\/countries\/fr\?competitions_page=2"/g)).toHaveLength(4);
});

it('pages a complete list independently and preserves the other tables and filters', () => {
  page.url.search = '?page=2&competitions_page=2&upcoming_page=3&q=Open';
  const { body: html } = render(Table<number>, {
    props: { rows, head, body, pageParam: 'competitions_page' },
  });
  expect(html.match(/<td>/g)).toHaveLength(50);
  expect(html).toContain('Result 51</td>');
  expect(html).toContain('Result 100</td>');
  expect(html).not.toContain('Result 101</td>');
  expect(html).toContain('href="/countries/fr?page=2&amp;upcoming_page=3&amp;q=Open"');
});

it.each(['-2', 'bad', '1.5', '999'])(
  'clamps invalid collection page %s to a populated page',
  (value) => {
    page.url.search = `?table_page=${value}`;
    const { body: html } = render(Table<number>, { props: { rows, head, body } });
    expect(html).toContain(value === '999' ? 'Result 101</td>' : 'Result 1</td>');
  }
);

it('keeps all 100 server-provided ranking rows and uses server pagination', () => {
  const { body: html } = render(Table<number>, {
    props: {
      rows: rows.slice(0, 100),
      head,
      body,
      itemName: 'athlete',
      pagination: {
        page: 1,
        total_pages: 4,
        total_items: 350,
        pageHref: (target) => `/?page=${target}`,
      },
    },
  });
  expect(html.match(/<td>/g)).toHaveLength(100);
  expect(html.replace(/<!--.*?-->/gs, '').match(/350\s+athletes/g)).toHaveLength(2);
  expect(html).toContain('href="/?page=2"');
});

it('shows one page and a singular label for a one-row table', () => {
  const { body: html } = render(Table<number>, {
    props: { rows: [1], head, body, itemName: 'competition' },
  });
  expect(html.match(/Page 1 of 1/g)).toHaveLength(2);
  expect(html.replace(/<!--.*?-->/gs, '').match(/1\s+competition</g)).toHaveLength(2);
  expect(html).not.toContain('href=');
});

it('uses the shared navigation for server pages and preserves the base path, filters and hash', () => {
  page.url.href =
    'https://openstreetlifting.org/base/countries/fr?page=2&competitions_page=3#athletes';
  const { body: html } = render(Table<number>, {
    props: {
      rows: rows.slice(50, 100),
      head,
      body,
      pageParam: 'page',
      pagination: { page: 2, total_pages: 3, total_items: 101 },
    },
  });
  expect(html.match(/<td>/g)).toHaveLength(50);
  expect(html).toContain('Result 51</td>');
  expect(html).toContain('href="/base/countries/fr?competitions_page=3#athletes"');
  expect(html).toContain('href="/base/countries/fr?page=3&amp;competitions_page=3#athletes"');
});
