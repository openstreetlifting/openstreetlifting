<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { resolve } from '$app/paths';

  const i18n = useI18n();

  type BreadcrumbItem = {
    label: string;
    href?: string;
  };

  let { items }: { items: BreadcrumbItem[] } = $props();

  // eslint-disable-next-line @typescript-eslint/no-explicit-any -- an href carrying a query string is not a literal route
  const target = (href: string) => i18n.href(resolve(href as any));
</script>

<nav aria-label={i18n.t('Breadcrumb')} class="mb-3 sm:mb-5">
  <ol class="flex flex-wrap items-center gap-1.5 text-meta text-muted sm:gap-2 sm:text-body">
    {#each items as item, index (index)}
      <li class="flex min-w-0 items-center gap-1.5 sm:gap-2">
        {#if index > 0}
          <svg
            aria-hidden="true"
            class="h-3 w-3 shrink-0 sm:h-4 sm:w-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            stroke-width="2"
          >
            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
          </svg>
        {/if}
        {#if item.href && index < items.length - 1}
          <a
            href={target(item.href)}
            class="rounded transition-colors hover:text-secondary focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2 focus-visible:ring-offset-canvas focus-visible:outline-none"
          >
            {item.label}
          </a>
        {:else}
          <span
            aria-current={index === items.length - 1 ? 'page' : undefined}
            class="text-secondary">{item.label}</span
          >
        {/if}
      </li>
    {/each}
  </ol>
</nav>
