<script lang="ts" generics="Value extends string | number | null | undefined">
  import type { HTMLSelectAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { FIELD } from '#lib/constants/typography.js';
  import { cn } from '#lib/utils/index.js';
  import { ChevronIcon } from '#lib/components/icons/index.js';

  let {
    value = $bindable(),
    class: className,
    children,
    id,
    name = id,
    autocomplete = 'off',
    ...rest
  }: Omit<HTMLSelectAttributes, 'value' | 'children'> & {
    value?: Value;
    children: Snippet;
  } = $props();
</script>

<div class="relative inline-grid min-w-0">
  <select
    {id}
    {name}
    {autocomplete}
    bind:value
    class={cn(
      FIELD,
      'peer min-w-0 w-full cursor-pointer appearance-none py-2 pl-2.5',
      className,
      'py-1.5 pr-9'
    )}
    {...rest}
  >
    {@render children()}
  </select>
  <ChevronIcon
    class="pointer-events-none absolute top-1/2 right-3 h-3 w-3 -translate-y-1/2 text-secondary peer-disabled:text-faint"
  />
</div>
