<script lang="ts" generics="Text extends Message">
  import type { Snippet } from 'svelte';
  import { useI18n, type Placeholders } from '#lib/i18n/index.js';
  import type { Message } from '#lib/i18n/en.js';

  const i18n = useI18n();

  let {
    message,
    values,
  }: {
    message: Text;
    values: Record<Placeholders<Text>, string | number | Snippet>;
  } = $props();
  const parts = $derived(i18n.template(message).split(/(\{\w+\})/g));
</script>

{#each parts as part, index (index)}
  {#if part.startsWith('{')}
    {@const value = values[part.slice(1, -1) as Placeholders<Text>]}
    {#if typeof value === 'function'}{@render value()}{:else}{value}{/if}
  {:else}{part}{/if}
{/each}
