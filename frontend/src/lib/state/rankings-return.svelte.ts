/**
 * The query string of the rankings board a visitor last looked at, so a
 * breadcrumb back to it returns them to their filters rather than to the whole
 * board. The board itself stays URL-driven: this only decides where a link
 * points, never what the page renders.
 *
 * Only the board writes here, and only after a client-side navigation, so a
 * server-rendered page can never inherit another visitor's filters.
 */
let lastQuery = $state('');

export function rememberRankings(search: string) {
  lastQuery = search;
}

/** Where a "back to the rankings" link should point right now. */
export function rankingsHref(): string {
  return lastQuery ? `/${lastQuery}` : '/';
}
