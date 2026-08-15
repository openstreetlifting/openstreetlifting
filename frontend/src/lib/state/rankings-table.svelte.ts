import { resolve } from '$app/paths';
import { goto } from '$app/navigation';
import { SvelteURLSearchParams } from 'svelte/reactivity';
import type { RankingEntry, PaginationMeta } from '$lib/types/ranking';

interface RankingsTableConfig {
  /** Route to push filter changes onto, e.g. `/` or `/competitions/some-slug`. */
  basePath: string;
  /** Sent on every fetch but never shown as a filter, e.g. a fixed competition_id. */
  fixedParams?: Record<string, string>;
  /** Whether this table supports filtering by competition year. */
  includeYear?: boolean;
  initialUrl: URL;
}

/**
 * Shared filter, sort and pagination state for a rankings table. Both the
 * global rankings page and a per-meet leaderboard read from the same
 * `/api/v1/rankings` shape, so they share this instead of each carrying its
 * own near-identical copy.
 */
export class RankingsTable {
  rankings = $state<RankingEntry[]>([]);
  currentPage = $state(1);
  totalPages = $state(1);
  totalItems = $state(0);
  isLoading = $state(false);

  genderFilter = $state<string | null>(null);
  categoryFilter = $state<string | null>(null);
  countryFilter = $state<string | null>(null);
  searchFilter = $state('');
  yearFilter = $state<number | null>(null);
  movementFilter = $state('ris');
  sortDirection = $state<'asc' | 'desc'>('desc');

  readonly includeYear: boolean;
  private basePath: string;
  private fixedParams: Record<string, string>;

  constructor(config: RankingsTableConfig) {
    this.basePath = config.basePath;
    this.fixedParams = config.fixedParams ?? {};
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

  /** Call from a `$effect` whenever the page's `data` prop changes. */
  syncFromData(data: { initialRankings: RankingEntry[]; pagination: PaginationMeta }) {
    this.rankings = data.initialRankings;
    this.currentPage = data.pagination.page;
    this.totalPages = data.pagination.total_pages;
    this.totalItems = data.pagination.total_items;
  }

  private get searchQuery(): string {
    return this.searchFilter.trim();
  }

  private buildFilterParams(targetPage: number): SvelteURLSearchParams {
    const params = new SvelteURLSearchParams();
    params.set('page', String(targetPage));
    params.set('movement', this.movementFilter);
    params.set('direction', this.sortDirection);
    if (this.genderFilter) params.set('gender', this.genderFilter);
    if (this.categoryFilter) params.set('category', this.categoryFilter);
    if (this.includeYear && this.yearFilter) params.set('year', String(this.yearFilter));
    if (this.countryFilter) params.set('country', this.countryFilter);
    if (this.searchQuery) params.set('q', this.searchQuery);
    for (const [key, value] of Object.entries(this.fixedParams)) params.set(key, value);
    return params;
  }

  private updateURL(targetPage: number) {
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
    goto(resolve(path as any), { replaceState: true, keepFocus: true, noScroll: true });
  }

  private async fetchRankings(targetPage: number) {
    if (this.isLoading) return;
    this.isLoading = true;
    try {
      const params = this.buildFilterParams(targetPage);
      const response = await fetch(`/api/rankings?${params}`);
      const result = await response.json();

      this.rankings = result.data;
      this.currentPage = result.pagination.page;
      this.totalPages = result.pagination.total_pages;
      this.totalItems = result.pagination.total_items;
    } catch (error) {
      console.error('Error loading rankings:', error);
    } finally {
      this.isLoading = false;
    }
  }

  async handleFilterChange() {
    this.updateURL(1);
    await this.fetchRankings(1);
  }

  async sortBy(value: string) {
    if (this.movementFilter === value) {
      this.sortDirection = this.sortDirection === 'desc' ? 'asc' : 'desc';
    } else {
      this.movementFilter = value;
      this.sortDirection = 'desc';
    }
    await this.handleFilterChange();
  }

  async goToPage(targetPage: number) {
    if (targetPage < 1 || targetPage > this.totalPages || targetPage === this.currentPage) return;
    this.updateURL(targetPage);
    await this.fetchRankings(targetPage);
  }

  async clearFilters() {
    this.genderFilter = null;
    this.categoryFilter = null;
    this.countryFilter = null;
    this.yearFilter = null;
    this.searchFilter = '';
    this.movementFilter = 'ris';
    this.sortDirection = 'desc';
    await this.handleFilterChange();
  }
}
