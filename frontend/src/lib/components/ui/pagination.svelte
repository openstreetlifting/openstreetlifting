<script lang="ts">
  interface Props {
    page: number;
    totalPages: number;
    disabled?: boolean;
    /** A URL already resolved by the caller with `$app/paths.resolve`. */
    pageHref: (page: number) => string;
    replaceState?: boolean;
  }

  let { page, totalPages, disabled = false, pageHref, replaceState = false }: Props = $props();

  const items = $derived.by(() => {
    const result: (number | 'ellipsis')[] = [];
    const add = (value: number | 'ellipsis') => result.push(value);

    if (totalPages <= 7) {
      for (let i = 1; i <= totalPages; i++) add(i);
      return result;
    }

    add(1);
    if (page > 3) add('ellipsis');

    const start = Math.max(2, page - 1);
    const end = Math.min(totalPages - 1, page + 1);
    for (let i = start; i <= end; i++) add(i);

    if (page < totalPages - 2) add('ellipsis');
    add(totalPages);

    return result;
  });

  const linkBase =
    'inline-flex h-8 min-w-8 items-center justify-center rounded-md px-2 text-xs font-medium sm:px-3 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2 focus-visible:ring-offset-canvas aria-disabled:pointer-events-none';
</script>

<!-- eslint-disable svelte/no-navigation-without-resolve -- pageHref returns URLs resolved by the caller -->
<nav
  aria-label="Pagination"
  class="flex flex-wrap items-center justify-center gap-1"
  data-sveltekit-keepfocus
  data-sveltekit-noscroll
  data-sveltekit-replacestate={replaceState}
>
  <a
    href={disabled || page <= 1 ? undefined : pageHref(page - 1)}
    class="{linkBase} gap-1 border border-stroke text-secondary hover:bg-surface-hover hover:text-ink aria-disabled:opacity-40"
    aria-label="Previous page"
    aria-disabled={disabled || page <= 1}
    tabindex={disabled || page <= 1 ? -1 : undefined}
  >
    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
      <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
    </svg>
    <span class="hidden sm:inline">Previous</span>
  </a>

  {#each items as item, index (index)}
    {#if item === 'ellipsis'}
      <span class="inline-flex h-8 min-w-8 items-center justify-center text-xs text-muted"
        >&hellip;</span
      >
    {:else}
      <a
        href={disabled || item === page ? undefined : pageHref(item)}
        class="{linkBase} border {item === page
          ? 'border-stroke-strong bg-surface-selected text-ink'
          : 'border-transparent text-secondary hover:bg-surface hover:text-ink aria-disabled:opacity-40'}"
        aria-disabled={disabled || item === page}
        tabindex={disabled || item === page ? -1 : undefined}
        aria-current={item === page ? 'page' : undefined}
      >
        {item}
      </a>
    {/if}
  {/each}

  <a
    href={disabled || page >= totalPages ? undefined : pageHref(page + 1)}
    class="{linkBase} gap-1 border border-stroke text-secondary hover:bg-surface-hover hover:text-ink aria-disabled:opacity-40"
    aria-label="Next page"
    aria-disabled={disabled || page >= totalPages}
    tabindex={disabled || page >= totalPages ? -1 : undefined}
  >
    <span class="hidden sm:inline">Next</span>
    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
      <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
    </svg>
  </a>
</nav>
