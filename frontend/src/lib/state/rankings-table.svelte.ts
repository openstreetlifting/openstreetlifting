import { resolve } from '$app/paths';
import { goto } from '$app/navigation';
import { SvelteURLSearchParams } from 'svelte/reactivity';

interface RankingsTableConfig {
  /** Route to push filter changes onto, e.g. `/` or `/competitions/some-slug`. */
  basePath: string;
  /** Whether this table supports filtering by competition year. */
  includeYear?: boolean;
  initialUrl: URL;
}

/**
 * Shared filter and sort state for a rankings table. Both the global rankings
 * page and a per-meet leaderboard read from the same `/api/v1/rankings` shape,
 * so they share this instead of each carrying its own near-identical copy.
 *
 * The URL is the only source of truth: every change navigates, and the page's
 * `load` refetches. Nothing here holds rows or pagination, so a server-rendered
 * page is already correct before hydration.
 */
export class RankingsTable {
  genderFilter = $state<string | null>(null);
  categoryFilter = $state<string | null>(null);
  countryFilter = $state<string | null>(null);
  searchFilter = $state('');
  yearFilter = $state<number | null>(null);
  movementFilter = $state('ris');
  sortDirection = $state<'asc' | 'desc'>('desc');

  readonly includeYear: boolean;
  private basePath: string;

  constructor(config: RankingsTableConfig) {
    this.basePath = config.basePath;
    this.includeYear = config.includeYear ?? false;

    const params = config.initialUrl.searchParams;
    this.genderFilter = params.get('gender') || null;
    this.categoryFilter = params.get('category') || null;
    this.countryFilter = params.get('country') || null;
    this.searchFilter = params.get('q') ?? '';
    this.yearFilter = this.includeYear ? Number(params.get('year')) || null : null;
    this.movementFilter = params.get('movement') || 'ris';
    this.sortDirection = params.get('direction') === 'asc' ? 'asc' : 'desc';
  }

  private get searchQuery(): string {
    return this.searchFilter.trim();
  }

  /** Defaults stay out of the query string so a shared link carries only what was chosen. */
  private navigate(targetPage: number) {
    const params = new SvelteURLSearchParams();
    if (this.movementFilter !== 'ris') params.set('movement', this.movementFilter);
    if (this.sortDirection !== 'desc') params.set('direction', this.sortDirection);
    if (this.genderFilter) params.set('gender', this.genderFilter);
    if (this.categoryFilter) params.set('category', this.categoryFilter);
    if (this.includeYear && this.yearFilter) params.set('year', String(this.yearFilter));
    if (this.countryFilter) params.set('country', this.countryFilter);
    if (this.searchQuery) params.set('q', this.searchQuery);
    if (targetPage > 1) params.set('page', String(targetPage));

    const queryString = params.toString();
    const path = queryString ? `${this.basePath}?${queryString}` : this.basePath;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any -- basePath is built at runtime, not a literal route
    return goto(resolve(path as any), { replaceState: true, keepFocus: true, noScroll: true });
  }

  async handleFilterChange() {
    await this.navigate(1);
  }

  async goToPage(targetPage: number) {
    await this.navigate(targetPage);
  }

  async clearFilters() {
    this.genderFilter = null;
    this.categoryFilter = null;
    this.countryFilter = null;
    this.yearFilter = null;
    this.searchFilter = '';
    this.movementFilter = 'ris';
    this.sortDirection = 'desc';
    await this.navigate(1);
  }
}
