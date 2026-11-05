<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { ChevronIcon } from '#lib/components/icons/index.js';

  const i18n = useI18n();
  interface Props {
    page: number;
    totalPages: number;
    disabled?: boolean;
    /** A URL already resolved by the caller with `$app/paths.resolve`. */
    pageHref: (page: number) => string;
    replaceState?: boolean;
    label?: string;
  }

  let {
    page,
    totalPages,
    disabled = false,
    pageHref,
    replaceState = false,
    label,
  }: Props = $props();

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
    'inline-flex h-11 min-w-11 items-center justify-center rounded-control px-2 font-mono text-meta font-medium sm:h-8 sm:min-w-8 sm:px-3 transition-colors active:bg-surface-selected focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2 focus-visible:ring-offset-canvas aria-disabled:pointer-events-none';
</script>

<!-- eslint-disable svelte/no-navigation-without-resolve -- pageHref returns URLs resolved by the caller -->
<nav
  aria-label={label ?? i18n.t('Pagination')}
  class="flex shrink-0 items-center justify-center gap-1"
  data-sveltekit-reset={false}
  data-sveltekit-replacestate={replaceState}
>
  <a
    href={disabled || page <= 1 ? undefined : pageHref(page - 1)}
    role={disabled || page <= 1 ? 'link' : undefined}
    class="{linkBase} gap-1 border border-stroke text-secondary hover:bg-surface-hover hover:text-ink aria-disabled:opacity-40"
    aria-label={i18n.t('Previous page')}
    aria-disabled={disabled || page <= 1}
    tabindex={disabled || page <= 1 ? -1 : undefined}
  >
    <ChevronIcon class="h-3.5 w-3.5 rotate-90" />
    <span class="hidden sm:inline">{i18n.t('Previous')}</span>
  </a>

  <span class="px-1 font-mono text-micro whitespace-nowrap text-secondary sm:hidden">
    {i18n.t('Page {page} of {pages}.', {
      page: i18n.number(page, 0),
      pages: i18n.number(totalPages, 0),
    })}
  </span>

  <div class="hidden items-center gap-1 sm:flex">
    {#each items as item, index (index)}
      {#if item === 'ellipsis'}
        <span class="inline-flex h-8 min-w-8 items-center justify-center text-meta text-muted"
          >&hellip;</span
        >
      {:else}
        <a
          href={disabled || item === page ? undefined : pageHref(item)}
          role={disabled || item === page ? 'link' : undefined}
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
  </div>

  <a
    href={disabled || page >= totalPages ? undefined : pageHref(page + 1)}
    role={disabled || page >= totalPages ? 'link' : undefined}
    class="{linkBase} gap-1 border border-stroke text-secondary hover:bg-surface-hover hover:text-ink aria-disabled:opacity-40"
    aria-label={i18n.t('Next page')}
    aria-disabled={disabled || page >= totalPages}
    tabindex={disabled || page >= totalPages ? -1 : undefined}
  >
    <span class="hidden sm:inline">{i18n.t('Next')}</span>
    <ChevronIcon class="h-3.5 w-3.5 -rotate-90" />
  </a>
</nav>
