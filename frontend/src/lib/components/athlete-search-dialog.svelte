<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { afterNavigate, goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { SearchIcon, CloseIcon } from '#lib/components/icons/index.js';
  import SearchInput from '#lib/components/ui/search-input.svelte';
  import { OVERLAY_SURFACE, TEXT } from '#lib/constants/typography.js';
  import AthleteSearchList from './athlete-search-list.svelte';
  import type { AthleteSearchHit, AthleteSearchResponse } from '#lib/types/athlete.js';

  const i18n = useI18n();

  let { class: className = '' }: { class?: string } = $props();

  let dialog: HTMLDialogElement;
  let query = $state('');
  let searched = $state('');
  let hits = $state<AthleteSearchHit[]>([]);
  let total = $state(0);
  let failed = $state(false);
  let active = $state(0);
  let controller: AbortController | undefined;

  afterNavigate(() => dialog?.close());

  function open() {
    dialog.showModal();
  }

  async function search() {
    const q = query.trim();
    controller?.abort();
    failed = false;
    if (!q) {
      searched = '';
      hits = [];
      total = 0;
      return;
    }

    controller = new AbortController();
    try {
      const response = await fetch(`/api/athletes/search?q=${encodeURIComponent(q)}`, {
        signal: controller.signal,
      });
      if (!response.ok) throw new Error(`Athlete search failed: ${response.status}`);
      const result: AthleteSearchResponse = await response.json();
      hits = result.data;
      total = result.total_items;
      searched = q;
      active = 0;
    } catch (error) {
      if (error instanceof DOMException && error.name === 'AbortError') return;
      failed = true;
    }
  }

  function onkeydown(event: KeyboardEvent) {
    if (!hits.length) return;
    if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
      event.preventDefault();
      const step = event.key === 'ArrowDown' ? 1 : -1;
      active = (active + step + hits.length) % hits.length;
    } else if (event.key === 'Enter' && !event.isComposing && searched === query.trim()) {
      event.preventDefault();
      goto(i18n.href(resolve(`athletes/${hits[active].slug}`)));
    }
  }
</script>

<button
  type="button"
  onclick={open}
  aria-label={i18n.t('Search athletes')}
  aria-haspopup="dialog"
  class="flex size-11 items-center justify-center rounded-control text-secondary transition-colors hover:bg-surface hover:text-ink focus-visible:outline-2 focus-visible:outline-focus lg:size-9 {className}"
>
  <SearchIcon class="h-5 w-5" />
</button>

<dialog
  bind:this={dialog}
  aria-label={i18n.t('Search athletes')}
  onclick={(event) => event.target === dialog && dialog.close()}
  class="{OVERLAY_SURFACE} mx-auto mt-16 w-[min(36rem,calc(100vw-2rem))] p-0 backdrop:bg-overlay/40"
>
  <div class="flex items-center gap-2 border-b border-stroke p-3" role="presentation" {onkeydown}>
    <div class="min-w-0 flex-1">
      <SearchInput
        bind:value={query}
        placeholder={i18n.t('Search an athlete')}
        delay={150}
        onSearch={search}
      />
    </div>
    <button
      type="button"
      onclick={() => dialog.close()}
      aria-label={i18n.t('Close search')}
      class="flex size-control items-center justify-center rounded-control text-secondary transition-colors hover:bg-surface hover:text-ink focus-visible:outline-2 focus-visible:outline-focus"
    >
      <CloseIcon />
    </button>
  </div>

  <div class="max-h-[60vh] overflow-y-auto" aria-live="polite">
    {#if failed}
      <p class="px-gutter py-4 {TEXT.caption}">{i18n.t('Couldn’t search athletes')}</p>
    {:else if !searched}
      <p class="px-gutter py-4 {TEXT.caption}">{i18n.t('Type a name to find any athlete.')}</p>
    {:else if hits.length === 0}
      <p class="px-gutter py-4 {TEXT.caption}">
        {i18n.t('No athlete matches “{query}”.', { query: searched })}
      </p>
    {:else}
      <AthleteSearchList {hits} {active} onpick={() => dialog.close()} />
      {#if total > hits.length}
        <p class="px-gutter py-3 {TEXT.caption}">
          {i18n.t('{count} more. Type more of the name to narrow the list.', {
            count: total - hits.length,
          })}
        </p>
      {/if}
    {/if}
  </div>
</dialog>
