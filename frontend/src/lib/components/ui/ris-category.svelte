<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { TEXT } from '#lib/constants/typography.js';

  const i18n = useI18n();

  let {
    sex = $bindable('men'),
    prominent = false,
  }: { sex?: 'men' | 'women'; prominent?: boolean } = $props();
</script>

<fieldset>
  <legend class="mb-2 {prominent ? 'text-sm font-medium text-ink' : `${TEXT.meta} text-muted`}"
    >{i18n.t('Category')}</legend
  >
  <div
    class="flex w-fit overflow-hidden rounded-lg border {prominent
      ? 'h-12 border-stroke-strong'
      : 'border-stroke'}"
  >
    {#each [['men', i18n.t('Men')], ['women', i18n.t('Women')]] as const as [value, label] (value)}
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
