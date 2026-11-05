<script lang="ts">
  import type { ResolvedPathname } from '$app/types';
  import { useI18n } from '#lib/i18n/index.js';
  import { SearchIcon } from '#lib/components/icons/index.js';
  import { EDGE_TO_EDGE } from '#lib/constants/table.js';
  import Button from './button.svelte';

  let {
    title,
    description,
    kind = 'empty',
    embedded = false,
    resetHref,
    retry,
  }: {
    title: string;
    description?: string;
    kind?: 'empty' | 'error';
    embedded?: boolean;
    resetHref?: ResolvedPathname;
    retry?: () => Promise<void>;
  } = $props();

  const i18n = useI18n();
  let retrying = $state(false);

  async function tryAgain() {
    if (!retry || retrying) return;
    retrying = true;
    try {
      await retry();
    } catch {
      // Keep the existing error and retry action available if revalidation fails.
    } finally {
      retrying = false;
    }
  }
</script>

<div
  class="{embedded
    ? ''
    : `${EDGE_TO_EDGE} rounded-panel border border-stroke-strong`} flex min-h-72 items-center bg-surface-subtle px-6 py-10 sm:min-h-80 sm:px-10 sm:py-14"
  aria-busy={retrying}
>
  <div class="mx-auto w-full max-w-md">
    <div
      class="mb-5 flex size-9 items-center justify-center border border-stroke-strong font-mono text-lg text-secondary"
      aria-hidden="true"
    >
      {#if kind === 'error'}!{:else}<SearchIcon class="size-4" />{/if}
    </div>
    <div role={kind === 'error' ? 'alert' : 'status'} aria-atomic="true">
      <h2 class="font-mono text-lg font-semibold text-balance text-ink sm:text-xl">{title}</h2>
      {#if description}<p
          class="mt-3 max-w-[42ch] text-sm leading-relaxed text-pretty text-secondary"
        >
          {description}
        </p>{/if}
    </div>
    {#if retry}
      <Button onclick={tryAgain} disabled={retrying} class="mt-6 min-h-11 w-full text-xs sm:w-auto">
        {retrying ? i18n.t('Retrying…') : i18n.t('Try again')}
      </Button>
    {:else if resetHref}
      <Button href={resetHref} class="mt-6 min-h-11 w-full text-xs sm:w-auto">
        {i18n.t('Clear search & filters')}
      </Button>
    {/if}
  </div>
</div>
