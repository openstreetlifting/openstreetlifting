<script lang="ts" generics="T">
  import type { Snippet } from 'svelte';
  import type { ResolvedPathname } from '$app/types';
  import { CONTROL } from '#lib/constants/typography.js';

  let {
    items,
    key,
    selected,
    href,
    onselect,
    attributes,
    label,
    class: className = '',
    children,
  }: {
    items: readonly T[];
    key: (item: T) => string;
    selected: (item: T) => boolean;
    href?: (item: T) => ResolvedPathname;
    onselect?: (item: T) => void;
    attributes?: (item: T) => { title?: string; 'aria-label'?: string };
    label?: string;
    class?: string;
    children: Snippet<[T]>;
  } = $props();

  const SEGMENT = `${CONTROL} inline-flex h-full min-w-control shrink-0 items-center justify-center px-3 tabular-nums transition-colors focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2 focus-visible:ring-offset-canvas focus-visible:outline-none`;
  const ACTIVE = 'bg-surface-selected text-ink';
  const IDLE = 'text-secondary hover:bg-surface hover:text-ink';
</script>

<ul
  class="inline-flex h-control max-w-full overflow-x-auto rounded-control border border-stroke {className}"
  aria-label={label}
>
  {#each items as item (key(item))}
    {@const active = selected(item)}
    <li class="h-full">
      {#if href && active}
        <span aria-current="page" class="{SEGMENT} {ACTIVE}">{@render children(item)}</span>
      {:else if href}
        <!-- eslint-disable-next-line svelte/no-navigation-without-resolve -- callers supply resolved app paths -->
        <a href={href(item)} class="{SEGMENT} {IDLE}" {...attributes?.(item)}
          >{@render children(item)}</a
        >
      {:else}
        <button
          type="button"
          aria-pressed={active}
          onclick={() => onselect?.(item)}
          class="{SEGMENT} {active ? ACTIVE : IDLE}"
          {...attributes?.(item)}>{@render children(item)}</button
        >
      {/if}
    </li>
  {/each}
</ul>
