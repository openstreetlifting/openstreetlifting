<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import type { Snippet } from 'svelte';
  import CloseIcon from '#lib/components/icons/CloseIcon.svelte';
  import { OVERLAY_SURFACE } from '#lib/constants/typography.js';

  const i18n = useI18n();

  let {
    label,
    children,
  }: {
    label: string;
    children: Snippet;
  } = $props();

  const panelId = $props.id();
</script>

<button
  type="button"
  popovertarget={panelId}
  aria-label={label}
  class="ml-1 inline-flex h-3.5 w-3.5 shrink-0 cursor-pointer items-center justify-center rounded-full border border-stroke-strong align-middle text-micro leading-none font-medium text-secondary transition-colors hover:border-focus hover:text-ink"
>
  ?
</button>

<div
  id={panelId}
  popover="auto"
  class="{OVERLAY_SURFACE} m-auto max-h-[calc(100dvh-2rem)] w-[min(30rem,calc(100vw-2rem))] overflow-y-auto overscroll-contain whitespace-normal p-5 text-left text-body leading-relaxed font-normal text-secondary backdrop:bg-overlay/70 sm:p-6"
>
  <button
    type="button"
    popovertarget={panelId}
    popovertargetaction="hide"
    aria-label={i18n.t('Close information')}
    class="float-right -mt-1 -mr-1 mb-1 ml-3 inline-flex h-11 w-11 cursor-pointer items-center justify-center rounded-control text-muted transition-colors hover:bg-surface-hover hover:text-ink focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus active:opacity-80 sm:h-8 sm:w-8"
  >
    <CloseIcon class="h-4 w-4" />
  </button>
  {@render children()}
</div>
