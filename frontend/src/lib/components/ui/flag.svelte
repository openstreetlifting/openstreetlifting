<script lang="ts">
  import { countryName } from '$lib/utils';

  interface Props {
    countryCode: string;
    background?: boolean;
    class?: string;
  }

  let { countryCode, background = false, class: className = '' }: Props = $props();

  const label = $derived(countryCode && countryCode.length === 2 ? countryName(countryCode) : null);

  const fileName = $derived(
    [...countryCode.toUpperCase()]
      .map((char) => (char.codePointAt(0)! + 127397).toString(16))
      .join('-')
  );
</script>

{#if label}
  {#if background}
    <svg
      class="h-full w-full {className}"
      viewBox="0 5 36 26"
      preserveAspectRatio="xMidYMid slice"
      aria-hidden="true"
    >
      <image href="/flags/{fileName}.svg" width="36" height="36" />
    </svg>
  {:else}
    <span
      class="flag {className}"
      style="background-image: url('/flags/{fileName}.svg')"
      role="img"
      aria-label={label}
      title={label}
    ></span>
  {/if}
{/if}

<style>
  /* The flag artwork sits inset on a square canvas rather than edge to edge,
     so `cover` on a flag-ratio box crops the padding away. */
  .flag {
    --height: var(--flag-height, 1.6em);
    display: inline-block;
    vertical-align: middle;
    width: calc(var(--height) * 36 / 26);
    height: var(--height);
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
    border-radius: 3px;
    /* Twemoji's national colours are faithful, which on a dark row leaves a
       navy like France's reading as a hole. Inverting around brightness lifts
       a channel that is already at zero, where brightness alone cannot, so the
       darks come up and the whites stay put. */
    filter: saturate(0.85) invert(1) brightness(0.85) invert(1);
  }
</style>
