import { goto } from '$app/navigation';
import type { AfterNavigate } from '@sveltejs/kit';

export class ListingSearch {
  value = $state('');
  applied = $state('');
  private pending: { href: string } | undefined;

  constructor(url: URL) {
    this.value = this.applied = url.searchParams.get('q') ?? '';
  }

  sync(url: URL, type?: AfterNavigate['type']) {
    this.applied = url.searchParams.get('q') ?? '';
    const ownNavigation = type === 'goto' && this.pending?.href === url.pathname + url.search;
    // A completed search must not overwrite characters typed while it was loading.
    if (!ownNavigation) this.value = this.applied;
  }

  async navigate(href: string, replaceState = true) {
    const request = { href };
    this.pending = request;
    try {
      // eslint-disable-next-line svelte/no-navigation-without-resolve -- Callers resolve listing URLs.
      await goto(href, { replaceState, keepFocus: true, noScroll: true });
    } finally {
      if (this.pending === request) this.pending = undefined;
    }
  }
}
