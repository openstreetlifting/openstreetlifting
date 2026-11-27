import type { page } from '$app/state';
import type { AfterNavigate } from '$app/navigation';
import { resolve } from '$app/paths';
import { SvelteURLSearchParams } from 'svelte/reactivity';
import type { Locale } from '#lib/i18n/locale.js';
import { localeHref, pathLocale } from '#lib/i18n/paths.js';
import { readFilters, type FilterSpec } from '#lib/utils/filters.js';
import { ListingSearch } from './listing-search.svelte';
import { UrlFilters } from './url-filters.svelte';

interface RankingsTableConfig<S extends FilterSpec> {
  /**
   * Route to push filter changes onto, e.g. `/` or `/competitions/some-slug`.
   * Pass a getter when it is read off page data: the class outlives a navigation
   * between two competitions, and a captured value would keep linking to the first.
   */
  basePath: string | (() => string);
  filters: S;
  initialUrl: typeof page.url;
  /** Column sorted before anyone picks one. Not every competition has a RIS to sort on. */
  defaultSort?: string | (() => string);
  /** Query parameters that select which board this is, kept through every filter change and clear. */
  scope?: () => Record<string, string | null>;
}

function reader<T>(value: T | (() => T)): () => T {
  return typeof value === 'function' ? (value as () => T) : () => value;
}

/**
 * Shared filter and sort state for a rankings table. Both the global rankings
 * page and a per-competition leaderboard read from the same `/api/v1/rankings` shape,
 * so they share this instead of each carrying its own near-identical copy.
 *
 * The URL describes the applied results; the search input keeps its own draft
 * while requests are in flight. Rows and pagination come from the page's load.
 */
export class RankingsTable<S extends FilterSpec> {
  readonly filters: UrlFilters<S>;
  readonly search: ListingSearch;
  private locale = $state<Locale>();

  get searchFilter() {
    return this.search.value;
  }

  set searchFilter(value: string) {
    this.search.value = value;
  }

  movementFilter = $state('ris');
  sortDirection = $state<'asc' | 'desc'>('desc');
  /**
   * The athlete a link asked the board to scroll to and highlight. Not a filter,
   * so it never narrows the rows and never survives a filter change, but it is
   * state a visitor arrived with and should be able to drop.
   */
  focusedAthlete = $state<string | null>(null);

  private readonly readBasePath: () => string;
  private readonly readDefaultSort: () => string;
  private readonly readScope: () => Record<string, string | null>;

  get defaultSort(): string {
    return this.readDefaultSort();
  }

  private get basePath(): string {
    return this.readBasePath();
  }

  constructor(config: RankingsTableConfig<S>) {
    this.readBasePath = reader(config.basePath);
    this.readDefaultSort = reader(config.defaultSort ?? 'ris');
    this.readScope = config.scope ?? (() => ({}));
    this.filters = new UrlFilters(
      config.filters,
      readFilters(config.initialUrl.searchParams, config.filters)
    );
    this.search = new ListingSearch(config.initialUrl);

    this.syncFromUrl(config.initialUrl);
  }

  /**
   * Read the controls back off the URL. The class outlives a navigation, so a
   * link that drops the query string (the header logo, back to the unfiltered
   * board) has to reach the selects as well as the rows.
   */
  syncFromUrl(url: typeof page.url, type?: AfterNavigate['type']) {
    this.locale = pathLocale(url.pathname);
    const params = url.searchParams;
    this.filters.sync(readFilters(params, this.filters.spec));
    this.search.sync(url, type);
    this.movementFilter = params.get('movement') || this.defaultSort;
    this.sortDirection = params.get('direction') === 'asc' ? 'asc' : 'desc';
    this.focusedAthlete = params.get('athlete') || null;
  }

  /** Whether anything but the defaults is in play, so clearing would do something. */
  get narrowed(): boolean {
    return Boolean(
      this.filters.activeCount ||
      this.searchQuery ||
      this.focusedAthlete ||
      this.movementFilter !== this.defaultSort ||
      this.sortDirection !== 'desc'
    );
  }

  private get searchQuery(): string {
    return this.searchFilter.trim();
  }

  /** Defaults stay out of the query string so a shared link carries only what was chosen. */
  pageHref(targetPage: number, query = this.search.applied) {
    const params = new SvelteURLSearchParams();
    for (const [name, value] of Object.entries(this.readScope())) {
      if (value) params.set(name, value);
    }
    if (this.movementFilter !== this.defaultSort) params.set('movement', this.movementFilter);
    if (this.sortDirection !== 'desc') params.set('direction', this.sortDirection);
    this.filters.writeTo(params);
    if (query.trim()) params.set('q', query.trim());
    if (targetPage > 1) params.set('page', String(targetPage));

    const queryString = params.toString();
    const path = queryString ? `${this.basePath}?${queryString}` : this.basePath;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any -- basePath is built at runtime, not a literal route
    const href = resolve(path.replace(/^\//, '') as any);
    return this.locale ? localeHref(href, this.locale) : href;
  }

  private navigate(targetPage: number) {
    return this.search.navigate(this.pageHref(targetPage, this.searchQuery));
  }

  async handleFilterChange() {
    await this.navigate(1);
  }

  async clearFilters() {
    this.filters.clear();
    this.searchFilter = '';
    this.focusedAthlete = null;
    this.movementFilter = this.defaultSort;
    this.sortDirection = 'desc';
    await this.navigate(1);
  }
}
