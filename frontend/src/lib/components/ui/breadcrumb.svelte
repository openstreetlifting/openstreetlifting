<script lang="ts">
  import { resolve } from '$app/paths';

  type BreadcrumbItem = {
    label: string;
    href?: string;
  };

  let { items }: { items: BreadcrumbItem[] } = $props();
</script>

<nav aria-label="Breadcrumb" class="mb-3 sm:mb-5">
  <ol class="flex flex-wrap items-center gap-1.5 text-xs text-zinc-500 sm:gap-2 sm:text-sm">
    {#each items as item, index (item.label)}
      {#if index > 0}
        <li>
          <svg
            class="h-3 w-3 sm:h-4 sm:w-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            stroke-width="2"
          >
            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
          </svg>
        </li>
      {/if}
      <li>
        {#if item.href && index < items.length - 1}
          <a
            href={resolve(item.href)}
            class="rounded transition-colors hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
          >
            {item.label}
          </a>
        {:else}
          <span class="text-zinc-400">{item.label}</span>
        {/if}
      </li>
    {/each}
  </ol>
</nav>
