<script lang="ts">
  interface Props {
    page: number;
    totalPages: number;
    disabled?: boolean;
    onNavigate: (page: number) => void;
  }

  let { page, totalPages, disabled = false, onNavigate }: Props = $props();

  // Always show first, last, current +/-1, and ellipses for the gaps.
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

  const buttonBase =
    'inline-flex h-8 min-w-8 items-center justify-center rounded-md px-2 text-xs font-medium sm:px-3 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-500 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950 disabled:pointer-events-none disabled:opacity-40';
</script>

<nav aria-label="Pagination" class="flex flex-wrap items-center justify-center gap-1">
  <button
    type="button"
    class="{buttonBase} gap-1 border border-zinc-800 text-zinc-300 hover:bg-zinc-800 hover:text-white"
    disabled={disabled || page <= 1}
    onclick={() => onNavigate(page - 1)}
  >
    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
      <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
    </svg>
    <span class="hidden sm:inline">Previous</span>
  </button>

  {#each items as item, index (index)}
    {#if item === 'ellipsis'}
      <span class="inline-flex h-8 min-w-8 items-center justify-center text-xs text-zinc-500"
        >&hellip;</span
      >
    {:else}
      <button
        type="button"
        class="{buttonBase} {item === page
          ? 'bg-white text-zinc-900'
          : 'text-zinc-300 hover:bg-zinc-800 hover:text-white'}"
        disabled={disabled || item === page}
        aria-current={item === page ? 'page' : undefined}
        onclick={() => onNavigate(item)}
      >
        {item}
      </button>
    {/if}
  {/each}

  <button
    type="button"
    class="{buttonBase} gap-1 border border-zinc-800 text-zinc-300 hover:bg-zinc-800 hover:text-white"
    disabled={disabled || page >= totalPages}
    onclick={() => onNavigate(page + 1)}
  >
    <span class="hidden sm:inline">Next</span>
    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
      <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
    </svg>
  </button>
</nav>
