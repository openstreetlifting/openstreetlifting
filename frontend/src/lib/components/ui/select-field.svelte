<script lang="ts" generics="Value extends string | number | null | undefined">
  import type { HTMLSelectAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import Select from './select.svelte';
  import { cn } from '#lib/utils/index.js';
  import { TEXT } from '#lib/constants/typography.js';

  const fieldId = $props.id();
  let {
    label,
    value = $bindable(),
    class: className,
    id = fieldId,
    name = label,
    children,
    ...rest
  }: Omit<HTMLSelectAttributes, 'value' | 'children'> & {
    label: string;
    value?: Value;
    children: Snippet;
  } = $props();
</script>

<div class={cn('grid min-w-0 gap-1', className)}>
  <label for={id} class={TEXT.fieldLabel}>{label}</label>
  <Select {id} {name} bind:value class="w-full" {...rest}>{@render children()}</Select>
</div>
