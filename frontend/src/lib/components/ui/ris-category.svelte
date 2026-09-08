<script lang="ts">
  import { TEXT } from '$lib/constants/typography';

  let {
    sex = $bindable('men'),
    prominent = false,
  }: { sex?: 'men' | 'women'; prominent?: boolean } = $props();
</script>

<fieldset>
  <legend class="mb-2 {prominent ? 'text-sm font-medium text-ink' : `${TEXT.meta} text-muted`}"
    >Category</legend
  >
  <div
    class="flex w-fit overflow-hidden rounded-lg border {prominent
      ? 'h-12 border-stroke-strong'
      : 'border-stroke'}"
  >
    {#each [['men', 'Men'], ['women', 'Women']] as const as [value, label] (value)}
      <button
        type="button"
        aria-pressed={sex === value}
        onclick={() => (sex = value)}
        class="px-4 py-2.5 text-xs font-medium transition-colors focus-visible:z-10 focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-secondary {sex ===
        value
          ? 'bg-ink text-canvas'
          : 'bg-canvas text-secondary hover:bg-surface hover:text-ink'}">{label}</button
      >
    {/each}
  </div>
</fieldset>
