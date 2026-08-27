<script lang="ts">
  import InfoTip from './info-tip.svelte';
  import { REPORTED_GLYPH, REPORTED_MARK } from '$lib/constants/table';
</script>

<!-- The note sits on the column rather than on each row, because a whole meet
     usually shares one provenance and repeating the explanation on every row
     would say the same thing dozens of times. -->
<span class="inline-flex items-center whitespace-nowrap">
  RIS
  <InfoTip label="What a RIS score is and where it came from">
    <p class="mb-3 font-medium text-zinc-100">How this score was arrived at</p>
    <p class="mb-3">
      RIS puts lifters of different bodyweights on one scale, so totals can be compared across
      weight classes. Normally it is computed here, from the athlete's bodyweight and total.
    </p>
    <p>
      A score marked <span class={REPORTED_MARK}>{REPORTED_GLYPH}</span> was reported by the federation
      instead. They published the number without a bodyweight, so there is nothing to recompute it from.
      It is not a lesser result, only one that cannot be restated on the same scale as the rest.
    </p>
  </InfoTip>
</span>
