import type { page } from '$app/state';
import { goto, type AfterNavigate } from '$app/navigation';

export class ListingSearch {
  value = $state('');
  applied = $state('');
  private pending: { href: string } | undefined;

  constructor(url: typeof page.url) {
    this.value = this.applied = url.searchParams.get('q') ?? '';
  }

  sync(url: typeof page.url, type?: AfterNavigate['type']) {
    this.applied = url.searchParams.get('q') ?? '';
    const ownNavigation = type === 'goto' && this.pending?.href === url.pathname + url.search;
    // A completed search must not overwrite characters typed while it was loading.
    if (!ownNavigation) this.value = this.applied;
  }

  async navigate(href: string, replaceState = true) {
    const request = { href };
    this.pending = request;
    try {
      await goto(href, { replace: replaceState, reset: false });
    } finally {
      if (this.pending === request) this.pending = undefined;
    }
  }
}
