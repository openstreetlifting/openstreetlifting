<script lang="ts">
  import InfoTip from './info-tip.svelte';
  import { REPORTED_GLYPH, REPORTED_MARK } from '$lib/constants/table';

  const RIS_SOURCE_URL = 'https://warisradji.com/ris/';
</script>

<!-- The note sits on the column rather than on each row, because a whole meet
     usually shares one provenance and repeating the explanation on every row
     would say the same thing dozens of times. -->
<span class="inline-flex items-center whitespace-nowrap">
  RIS
  <InfoTip label="What a RIS score is and where it came from">
    <p class="mb-3 font-medium text-zinc-100">What is RIS?</p>

    <!-- Quoted from the source rather than paraphrased, so it is marked as a
         quotation and attributed. The wording is not ours to present as ours. -->
    <blockquote cite={RIS_SOURCE_URL} class="mb-2 border-l-2 border-zinc-700 pl-3">
      The Relative Index for Streetlifting (RIS) provides the definitive method for comparing
      Streetlifting athletes across weight classes within the same gender. Developed specifically
      for All-4 competitions, this scoring system normalizes strength performance using
      comprehensive data from elite competitors. Updated annually to reflect evolving standards, RIS
      offers a fair benchmark for athletes, coaches, and competition organizers worldwide.
    </blockquote>
    <p class="mb-3 text-zinc-400">
      <a
        href={RIS_SOURCE_URL}
        target="_blank"
        rel="noopener noreferrer"
        class="underline transition-colors hover:text-zinc-200">warisradji.com</a
      >
    </p>

    <p class="mb-3">
      When a federation publishes the athlete's bodyweight, we recompute the score ourselves from
      that bodyweight and their total. When it does not, we trust the RIS the federation gave and
      record it as reported, marked <span class={REPORTED_MARK}>{REPORTED_GLYPH}</span>. A reported
      score is not a lesser result, only one we cannot restate on the same scale as the rest.
    </p>

    <p class="text-zinc-400">RIS formula by Waris Radji and Mathieu Ardoin.</p>
  </InfoTip>
</span>
