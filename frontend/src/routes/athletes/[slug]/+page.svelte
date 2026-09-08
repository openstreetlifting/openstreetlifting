<script lang="ts">
  import type { PageData } from './$types';
  import type { AthleteCompetitionSummary } from '$lib/types/athlete';
  import type { Attempt } from '$lib/types/competition';
  import {
    Card,
    Breadcrumb,
    Flag,
    Table,
    RisScore,
    RisHeader,
    TABLE_CELL,
    TABLE_HEAD_CELL,
    FROZEN_CELL,
    FROZEN_HEAD_CELL,
    FROZEN_EDGE,
    FROZEN_RANK,
  } from '$lib/components/ui';
  import { ChevronIcon, GlobeIcon, InstagramIcon } from '$lib/components/icons';
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { athleteFilters } from '$lib/utils/athlete-filters';
  import { totalFormats } from '$lib/utils/athlete-progress';
  import { rankingsHref } from '$lib/state/rankings-return.svelte';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import { RANKING_SORTS } from '$lib/constants/ranking';
  import { formatDate, formatWeight, formatAthleteName, countryName } from '$lib/utils';
  import Seo from '$lib/components/seo.svelte';
  import AthleteProgress from '$lib/components/athlete-progress.svelte';
  import AthleteStrength from '$lib/components/athlete-strength.svelte';
  import { absolute, athleteLd, breadcrumbLd } from '$lib/seo';
  import {
    ATTEMPT_ROW,
    CELL,
    FIGURE,
    STATUS_FLAG,
    NO_VALUE,
    NO_RESULT,
    TEXT_CELL,
  } from '$lib/constants/table';
  import { FIELD, TEXT } from '$lib/constants/typography';

  const STATUS_LABEL: Record<string, string> = { disqualified: 'DQ', no_show: 'NS' };
  const STATUS_TITLE: Record<string, string> = {
    disqualified: 'Disqualified',
    no_show: 'Did not lift',
  };

  let { data }: { data: PageData } = $props();
  const { athlete } = $derived(data);
  const showsDivision = $derived(athlete.competitions.some((c) => c.division));

  const LIFTS = [
    { key: 'muscleup', code: 'M', movement: 'Muscle-up', label: 'Muscle Up' },
    { key: 'pullup', code: 'P', movement: 'Pull-up', label: 'Pull Up' },
    { key: 'dips', code: 'D', movement: 'Dips', label: 'Dips' },
    { key: 'squat', code: 'S', movement: 'Squat', label: 'Squat' },
  ] as const;

  type Lift = (typeof LIFTS)[number];

  // Without an event code, recorded lifts are the only evidence of contested movements.
  function contestedBy(competition: AthleteCompetitionSummary, lift: Lift): boolean {
    return competition.event
      ? competition.event.includes(lift.code)
      : competition.lifts.some((candidate) => candidate.movement_name === lift.movement);
  }

  type LiftCell =
    /** The competition never contested the movement. */
    | { kind: 'absent' }
    /** Contested, and every attempt failed, with nothing recorded behind it. */
    | { kind: 'bombed' }
    /** The source published a best with no attempt breakdown behind it. */
    | { kind: 'best'; best: string }
    | { kind: 'attempts'; attempts: Attempt[]; best: string | null };

  function liftCell(competition: AthleteCompetitionSummary, lift: Lift): LiftCell {
    if (!contestedBy(competition, lift)) return { kind: 'absent' };

    const made = competition.lifts.find((candidate) => candidate.movement_name === lift.movement);

    if (made?.attempts.length) {
      return { kind: 'attempts', attempts: made.attempts, best: made.best_weight };
    }

    return made?.best_weight == null
      ? { kind: 'bombed' }
      : { kind: 'best', best: made.best_weight };
  }

  const contested = $derived(
    LIFTS.filter((lift) =>
      athlete.competitions.some((competition) => contestedBy(competition, lift))
    )
  );

  const GENDER_LABEL: Record<string, string> = { M: 'Men', F: 'Women' };
  const genderLabel = $derived(GENDER_LABEL[athlete.gender] ?? athlete.gender);
  const latestCategory = $derived(
    athlete.strength_profile?.category ??
      athlete.competitions.find((competition) => competition.status === 'competed')?.category_name
  );
  const latestWeightClass = $derived(
    latestCategory?.startsWith(`${genderLabel} `)
      ? latestCategory.slice(genderLabel.length + 1)
      : latestCategory
  );

  const CARD_LABEL =
    'flex items-center gap-1.5 text-xs font-medium tracking-wider text-secondary uppercase';
  const CARD_FIGURE = 'font-mono text-xl font-semibold text-ink sm:text-2xl';
  const CARD_CAPTION = 'text-xs text-muted';
  const CARD_SURFACE = 'border border-stroke bg-surface';
  const CARD_GRID = 'grid max-w-records grid-cols-2 gap-3 sm:gap-4 md:grid-cols-4';
  const RANKING_CARD_GRID = 'grid grid-cols-2 gap-3 sm:gap-4';

  const formats = $derived(totalFormats(athlete.competitions));
  const filters = $derived(athleteFilters(page.url.searchParams, formats));
  const selectedMetric = $derived(filters.ranking);

  function updateFilter(key: 'ranking' | 'performance' | 'event', value: string) {
    const params = new SvelteURLSearchParams(page.url.searchParams);
    const defaults = {
      ranking: 'ris',
      performance: 'total',
      event: formats[0] ?? 'MPDS',
    };
    if (value === defaults[key]) params.delete(key);
    else params.set(key, value);
    const query = params.toString();
    return goto(resolve(`/athletes/${athlete.slug}${query ? `?${query}` : ''}${page.url.hash}`), {
      replaceState: true,
      keepFocus: true,
      noScroll: true,
    });
  }

  const selectedStanding = $derived(athlete.standing?.[selectedMetric] ?? null);
  const selectedCountry = $derived(selectedStanding?.country.code ?? athlete.country);
  const selectedBoardFilters = $derived.by((): Record<string, string> =>
    selectedMetric === 'ris'
      ? {}
      : {
          movement: selectedMetric,
          gender: athlete.gender,
          category: selectedStanding?.class ?? '',
        }
  );

  const RANKING_PAGE_SIZE = 50;

  function boardQuery(place: number, filters: Record<string, string> = {}): string {
    const params = new SvelteURLSearchParams(filters);
    const target = Math.ceil(place / RANKING_PAGE_SIZE);
    if (target > 1) params.set('page', String(target));
    params.set('athlete', athlete.slug);
    return params.toString();
  }

  const athleteName = $derived(formatAthleteName(athlete));

  const seoBests = $derived(
    athlete.personal_records
      .map(
        (record) => `${record.movement_name.toLowerCase()} ${formatWeight(record.max_weight)} kg`
      )
      .join(', ')
  );

  const seoDescription = $derived(
    [
      athleteName,
      athlete.native_name ? ` (${athlete.native_name})` : '',
      athlete.country ? `, ${countryName(athlete.country)}` : '',
      '. Streetlifting results and personal records',
      seoBests ? `: ${seoBests}` : '',
      `. ${athlete.total_competitions} ${athlete.total_competitions === 1 ? 'competition' : 'competitions'} in the OpenStreetlifting archive.`,
    ].join('')
  );
</script>

<Seo
  title="{athleteName} - Streetlifting results"
  description={seoDescription}
  canonical={absolute(`/athletes/${athlete.slug}`)}
  type="profile"
  jsonLd={[
    athleteLd(athlete, seoDescription),
    breadcrumbLd([
      { name: 'Rankings', path: '/' },
      { name: athleteName, path: `/athletes/${athlete.slug}` },
    ]),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb items={[{ label: 'Rankings', href: rankingsHref() }, { label: athleteName }]} />

  <div class="mb-6 sm:mb-10">
    <div class="flex items-center gap-3">
      <h1 class="{TEXT.title} flex min-w-0 items-center gap-3 text-ink">
        <Flag countryCode={athlete.country} class="shrink-0 [--flag-height:0.8em]" />
        <span class="truncate">{athleteName}</span>
      </h1>

      {#if athlete.instagram_handle}
        <a
          href={`https://www.instagram.com/${athlete.instagram_handle}`}
          target="_blank"
          rel="noopener noreferrer"
          class="shrink-0 text-ink transition-colors hover:text-secondary"
          aria-label="{athleteName} on Instagram"
          title="@{athlete.instagram_handle}"
        >
          <InstagramIcon class="h-5 w-5 sm:h-6 sm:w-6" />
        </a>
      {/if}
    </div>

    {#if athlete.native_name}
      <p class="mt-1 text-base text-secondary sm:text-xl">{athlete.native_name}</p>
    {/if}

    <p class="mt-2 {TEXT.heading} text-secondary">
      {genderLabel}
      {#if latestWeightClass}
        {latestWeightClass.replace(/(\d)\s*(?:kg)?$/i, '$1 kg')}
      {/if}
    </p>
  </div>

  {#snippet standingContent(
    country: string | null,
    scope: string,
    place: number | undefined,
    field: number | undefined
  )}
    <div class="ranking-art" class:ranking-flag={country !== null} aria-hidden="true">
      {#if country}
        <Flag countryCode={country} background />
      {:else}
        <GlobeIcon class="h-40 w-40 shrink-0 text-secondary" />
      {/if}
    </div>
    <div class="relative">
      <div class={CARD_LABEL}>
        <span class="truncate text-secondary">{scope}</span>
        {#if place !== undefined}
          <ChevronIcon
            class="ml-auto h-3 w-3 shrink-0 -rotate-90 text-faint transition-colors group-hover:text-secondary"
          />
        {/if}
      </div>
      <div class="mt-1 flex items-baseline gap-1.5">
        {#if place !== undefined && field !== undefined}
          <span class={CARD_FIGURE}>#{place}</span>
          <span class="{CARD_CAPTION} {FIGURE}">/ {field}</span>
        {:else}
          <span class="text-sm font-medium text-secondary">Not ranked</span>
        {/if}
      </div>
      {#if selectedStanding?.class}
        <div class="mt-1 {CARD_CAPTION}">in category {selectedStanding.class}</div>
      {/if}
    </div>
  {/snippet}

  {#snippet standing(
    country: string | null,
    scope: string,
    place: number | undefined,
    field: number | undefined,
    query: string | undefined
  )}
    {#if query}
      <a
        href={resolve(`/?${query}`)}
        class="group relative isolate block overflow-hidden rounded-xl {CARD_SURFACE} p-3 transition-colors hover:border-stroke-strong hover:bg-surface-hover focus:ring-2 focus:ring-focus focus:outline-none"
      >
        {@render standingContent(country, scope, place, field)}
      </a>
    {:else}
      <div class="relative isolate overflow-hidden rounded-xl {CARD_SURFACE} p-3">
        {@render standingContent(country, scope, place, field)}
      </div>
    {/if}
  {/snippet}

  {#if athlete.standing}
    <div class="mb-6 max-w-summary sm:mb-8">
      <div class="mb-3 flex items-center justify-between gap-3">
        <h2 class="{TEXT.heading} text-ink">Ranking</h2>
        <label class="flex items-center gap-2 text-xs text-muted">
          <span class="sr-only">Metric</span>
          <select
            value={selectedMetric}
            onchange={(event) => updateFilter('ranking', event.currentTarget.value)}
            class="{FIELD} px-2.5 py-1.5"
          >
            {#each RANKING_SORTS as metric (metric.value)}
              <option value={metric.value}>{metric.label}</option>
            {/each}
          </select>
        </label>
      </div>
      <div class={RANKING_CARD_GRID}>
        {@render standing(
          null,
          'Global',
          selectedStanding?.global.place,
          selectedStanding?.global.field,
          selectedStanding
            ? boardQuery(selectedStanding.global.place, selectedBoardFilters)
            : undefined
        )}
        {@render standing(
          selectedCountry,
          countryName(selectedCountry),
          selectedStanding?.country.place,
          selectedStanding?.country.field,
          selectedStanding
            ? boardQuery(selectedStanding.country.place, {
                ...selectedBoardFilters,
                country: selectedCountry,
              })
            : undefined
        )}
      </div>
    </div>
  {/if}

  {#if athlete.personal_records.length > 0}
    <div class="mb-6 sm:mb-8">
      <h2 class="mb-3 {TEXT.heading} text-ink">Personal records</h2>
      <div class={CARD_GRID}>
        {#each athlete.personal_records as pr (pr.movement_name)}
          <Card class="p-3">
            <div class={CARD_LABEL}>{pr.movement_name}</div>
            <div class="mt-0.5 flex items-baseline gap-1.5">
              <span class="font-mono text-xl sm:text-2xl {CELL.counted}"
                >{formatWeight(pr.max_weight)}</span
              >
              <span class="text-xs text-muted">kg</span>
            </div>
            <div class={CARD_CAPTION}>
              <a
                href={resolve(`/competitions/${pr.competition_slug}`)}
                class="underline hover:text-secondary"
              >
                {pr.competition_name}
              </a>
              {#if pr.date}
                <span class="mx-1">•</span>
                {formatDate(pr.date)}
              {/if}
            </div>
          </Card>
        {/each}
      </div>
    </div>
  {/if}

  <div class="mt-8 grid items-start gap-8 sm:mt-10 sm:gap-10 2xl:grid-cols-2">
    <AthleteProgress
      athleteSlug={athlete.slug}
      competitions={athlete.competitions}
      {formats}
      metric={filters.performance}
      format={filters.event}
      onMetricChange={(value) => updateFilter('performance', value)}
      onFormatChange={(value) => updateFilter('event', value)}
    />

    <AthleteStrength profile={athlete.strength_profile} />
  </div>

  <div class="mt-8 sm:mt-10">
    <h2 class="mb-3 {TEXT.heading} text-ink">Competition history</h2>
    {#if athlete.competitions.length > 0}
      <Table>
        {#snippet head()}
          <th
            class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} {FROZEN_EDGE} text-secondary"
            >Rank</th
          >
          <th class="{TABLE_HEAD_CELL} text-secondary">Competition</th>
          <th class="{TABLE_HEAD_CELL} text-secondary">Total</th>
          <th class="{TABLE_HEAD_CELL} text-secondary"><RisHeader /></th>
          {#each contested as lift (lift.key)}
            <th class="{TABLE_HEAD_CELL} text-secondary">
              {lift.label}
              <span class="{ATTEMPT_ROW} mt-1 text-[0.65rem] font-normal text-muted">
                <span class="text-right">1</span>
                <span class="text-right">2</span>
                <span class="text-right">3</span>
                <span class="text-right">Best</span>
              </span>
            </th>
          {/each}
          <th class="{TABLE_HEAD_CELL} text-secondary">Date</th>
          <th class="{TABLE_HEAD_CELL} text-secondary">Class</th>
          {#if showsDivision}
            <th class="{TABLE_HEAD_CELL} text-secondary">Division</th>
          {/if}
        {/snippet}

        {#snippet body()}
          {#each athlete.competitions as competition (`${competition.competition_id}:${competition.category_name}:${competition.division ?? ''}`)}
            <tr class="transition-colors {competition.status !== 'competed' ? 'opacity-50' : ''}">
              <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {FROZEN_EDGE} {CELL.identity}">
                {#if competition.status !== 'competed'}
                  <span class={STATUS_FLAG} title={STATUS_TITLE[competition.status]}
                    >{STATUS_LABEL[competition.status]}</span
                  >
                {:else}
                  {competition.rank || NO_VALUE}
                {/if}
              </td>
              <td class={TABLE_CELL}>
                <a
                  href={resolve(`/competitions/${competition.competition_slug}`)}
                  class="{TEXT_CELL.competition} text-ink underline hover:text-secondary focus:ring-2 focus:ring-focus focus:outline-none"
                >
                  {competition.competition_name}
                </a>
              </td>
              <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">
                {formatWeight(competition.total)}
              </td>
              <td class="{TABLE_CELL} {CELL.counted} {FIGURE}">
                <RisScore value={competition.ris_score} source={competition.ris_source} />
              </td>
              {#each contested as lift (lift.key)}
                {@const cell = liftCell(competition, lift)}
                <td class="{TABLE_CELL} whitespace-nowrap">
                  <span class="{ATTEMPT_ROW} {FIGURE}">
                    {#if cell.kind === 'attempts'}
                      {#each [1, 2, 3] as slot (slot)}
                        {@const attempt = cell.attempts.find((a) => a.attempt_number === slot)}
                        <span
                          class="text-right {attempt && !attempt.is_successful
                            ? CELL.discounted
                            : CELL.data}"
                          title={attempt
                            ? `Attempt ${slot}: ${attempt.weight} kg, ${attempt.is_successful ? 'good lift' : 'no lift'}`
                            : `Attempt ${slot} not recorded`}
                        >
                          {attempt ? attempt.weight : ''}
                        </span>
                      {/each}
                    {:else}
                      <span></span>
                      <span></span>
                      <span></span>
                    {/if}

                    <span class="text-right {CELL.counted}">
                      {#if cell.kind === 'absent'}
                        <span class="font-normal {CELL.absent}">{NO_VALUE}</span>
                      {:else if cell.kind === 'bombed' || cell.best === null}
                        <span
                          class="font-normal {CELL.nothing}"
                          title="No successful {lift.label.toLowerCase()}">{NO_RESULT}</span
                        >
                      {:else}
                        {formatWeight(cell.best)}
                      {/if}
                    </span>
                  </span>
                </td>
              {/each}
              <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap">
                {formatDate(competition.competition_date)}
              </td>
              <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap">
                {competition.category_name}
              </td>
              {#if showsDivision}
                <td class="{TABLE_CELL} {CELL.data}">{competition.division || NO_VALUE}</td>
              {/if}
            </tr>
          {/each}
        {/snippet}
      </Table>
    {:else}
      <Card class="max-w-summary p-8">
        <p class="text-center text-secondary">No competition history available</p>
      </Card>
    {/if}
  </div>
</div>

<style>
  .ranking-art {
    position: absolute;
    inset: 0 -1rem 0 auto;
    display: flex;
    width: 55%;
    align-items: center;
    justify-content: flex-end;
    pointer-events: none;
    opacity: 0.65;
    mask-image: linear-gradient(to right, transparent, black 75%);
  }

  .ranking-art :global(svg) {
    stroke-width: 0.7;
  }

  .ranking-flag {
    inset: -15% -1rem auto auto;
    width: auto;
    height: 130%;
    aspect-ratio: 36 / 26;
  }

  @media (width < 40rem) {
    .ranking-flag {
      inset: 0;
      width: 100%;
      height: 100%;
      aspect-ratio: auto;
      opacity: 0.2;
      mask-image: none;
    }
  }
</style>
