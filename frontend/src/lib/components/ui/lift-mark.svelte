<script lang="ts">
  import { liftTone } from '#lib/constants/lift.js';

  let { movement, class: className = '' }: { movement: string; class?: string } = $props();

  const tone = $derived(liftTone(movement));
</script>

{#if tone}<span
    class="inline-block size-2 shrink-0 rounded-sm align-middle {className}"
    style:background={tone}
    aria-hidden="true"
  ></span>{/if}
