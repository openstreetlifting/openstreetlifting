<script lang="ts">
  import { resolve } from '$app/paths';
  import InfoTip from './info-tip.svelte';
  import { REPORTED_GLYPH, REPORTED_MARK } from '$lib/constants/table';

  const RIS_SOURCE_URL = 'https://warisradji.com/ris/';
</script>

<span class="inline-flex items-center align-middle whitespace-nowrap">
  RIS
  <InfoTip label="What a RIS score is and where it came from">
    <p class="mb-3 font-medium text-ink">What is RIS?</p>

    <blockquote cite={RIS_SOURCE_URL} class="mb-2 border-l-2 border-stroke-strong pl-3">
      The Relative Index for Streetlifting (RIS) provides the definitive method for comparing
      Streetlifting athletes across weight classes within the same gender. Developed specifically
      for All-4 competitions, this scoring system normalizes strength performance using
      comprehensive data from elite competitors. Updated annually to reflect evolving standards, RIS
      offers a fair benchmark for athletes, coaches, and competition organizers worldwide.
    </blockquote>
    <p class="mb-3 text-secondary">
      <a
        href={RIS_SOURCE_URL}
        target="_blank"
        rel="noopener noreferrer"
        class="underline transition-colors hover:text-ink">warisradji.com</a
      >
    </p>

    <p class="mb-3">
      We recalculate RIS with the current formula, so scores may differ from published results.
    </p>

    <p class="mb-3">
      When bodyweight is missing, we estimate it from the published RIS where possible. Otherwise,
      we keep the reported score, marked <span class={REPORTED_MARK}>{REPORTED_GLYPH}</span>.
    </p>

    <p class="text-secondary">
      <a href={resolve('/ris')} class="underline transition-colors hover:text-ink"
        >Learn more on the ris page of OpenStreetlifting</a
      >
    </p>
  </InfoTip>
</span>
