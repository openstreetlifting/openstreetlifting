<script lang="ts">
  import type { PageData } from './$types';
  import type {
    AthleteCompetitionSummary,
    MetricStanding,
    PersonalRecord,
  } from '$lib/types/athlete';
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
  import { rankingsHref } from '$lib/state/rankings-return.svelte';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import { RANKING_SORTS } from '$lib/constants/ranking';
  import {
    formatDate,
    formatWeight,
    formatScore,
    formatAthleteName,
    countryName,
  } from '$lib/utils';
  import Seo from '$lib/components/seo.svelte';
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

  // The badge is two letters, so the title carries the meaning. A reason from
  // the source is better than either, when there is one.
  const STATUS_LABEL: Record<string, string> = { disqualified: 'DQ', no_show: 'NS' };
  const STATUS_TITLE: Record<string, string> = {
    disqualified: 'Disqualified',
    no_show: 'Did not lift',
  };

  function statusTitle(status: string, reason: string | null): string {
    const name = STATUS_TITLE[status] ?? status;
    return reason ? `${name}: ${reason.toLowerCase()}` : name;
  }
  import { FIELD, TEXT } from '$lib/constants/typography';

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

  // The event code names the movements a competition contested, so a column with
  // no weight can be read as bombed rather than never lifted. Older rows carry no
  // event, and there the lifts themselves are all the history says.
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
      const attempts = [...made.attempts].sort((a, b) => a.attempt_number - b.attempt_number);
      return { kind: 'attempts', attempts, best: made.best_weight };
    }

    return made?.best_weight == null
      ? { kind: 'bombed' }
      : { kind: 'best', best: made.best_weight };
  }

  // A column no meet in the history ran would be dashes all the way down.
  const contested = $derived(
    LIFTS.filter((lift) =>
      athlete.competitions.some((competition) => contestedBy(competition, lift))
    )
  );

  function sortPersonalRecords(records: PersonalRecord[]) {
    const movementPriority: Record<string, number> = {
      'muscle up': 1,
      'muscle-up': 1,
      muscleup: 1,
      'pull up': 2,
      'pull-up': 2,
      pullup: 2,
      dips: 3,
      dip: 3,
      squat: 4,
      squats: 4,
    };

    const getPriority = (movementName: string): number => {
      const normalized = movementName.toLowerCase().trim();
      return movementPriority[normalized] ?? 999; // Unknown movements go to the end
    };

    return [...records].sort((a, b) => getPriority(a.movement_name) - getPriority(b.movement_name));
  }

  const GENDER_LABEL: Record<string, string> = { M: 'Men', F: 'Women' };

  const CARD_LABEL = `flex items-center gap-1.5 ${TEXT.micro} tracking-wider text-zinc-500 uppercase`;
  const CARD_FIGURE = 'font-mono text-xl font-semibold text-white sm:text-2xl';
  const CARD_CAPTION = 'text-xs text-zinc-500';
  const CARD_GRID = 'grid grid-cols-2 gap-3 sm:gap-4 md:grid-cols-4';
  const RANKING_CARD_GRID = 'grid grid-cols-2 gap-3 sm:gap-4';

  type RankingMetric = (typeof RANKING_SORTS)[number]['value'];
  type SelectedStanding = Omit<MetricStanding, 'class'> & { class?: string };
  let selectedMetric = $state<RankingMetric>('ris');

  const selectedMetricLabel = $derived(
    RANKING_SORTS.find((metric) => metric.value === selectedMetric)?.label ?? 'RIS'
  );

  const selectedStanding = $derived.by((): SelectedStanding | null => {
    if (!athlete.standing) return null;

    if (selectedMetric === 'ris') {
      const ris = athlete.standing.ris;
      return ris
        ? {
            value: ris.score ?? '',
            global: ris.global,
            country: ris.country,
          }
        : null;
    }

    return athlete.standing[selectedMetric] ?? null;
  });

  const selectedMetricValue = $derived(
    selectedStanding
      ? selectedMetric === 'ris'
        ? formatScore(selectedStanding.value)
        : `${formatWeight(selectedStanding.value)} kg`
      : 'Not ranked'
  );
  const selectedCountry = $derived(selectedStanding?.country.code ?? athlete.country);

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
    sortPersonalRecords(athlete.personal_records)
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

<div class="mx-auto max-w-[var(--content-max-width)] px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb items={[{ label: 'Rankings', href: rankingsHref() }, { label: athleteName }]} />

  <div class="mb-6 sm:mb-10">
    <div class="flex items-center gap-3">
      <h1 class="{TEXT.title} flex min-w-0 items-center gap-3 text-white">
        <Flag countryCode={athlete.country} class="shrink-0 [--flag-height:0.8em]" />
        <span class="truncate">{athleteName}</span>
      </h1>

      {#if athlete.instagram_handle}
        <a
          href={`https://www.instagram.com/${athlete.instagram_handle}`}
          target="_blank"
          rel="noopener noreferrer"
          class="shrink-0 text-white transition-colors hover:text-zinc-300"
          aria-label="{athleteName} on Instagram"
          title="@{athlete.instagram_handle}"
        >
          <InstagramIcon class="h-5 w-5 sm:h-6 sm:w-6" />
        </a>
      {/if}
    </div>

    {#if athlete.native_name}
      <p class="mt-1 text-base text-zinc-400 sm:text-xl">{athlete.native_name}</p>
    {/if}

    <p class="mt-2 text-xs text-zinc-400 sm:text-sm">
      {GENDER_LABEL[athlete.gender] ?? athlete.gender}
      &middot;
      {athlete.total_competitions}
      {athlete.total_competitions === 1 ? 'competition' : 'competitions'}
    </p>
  </div>

  {#snippet standingContent(
    country: string | null,
    scope: string,
    place: number | undefined,
    field: number | undefined
  )}
    <div class={CARD_LABEL}>
      {#if country}
        <Flag countryCode={country} class="shrink-0 [--flag-height:1.2em]" />
      {:else}
        <GlobeIcon class="h-3.5 w-3.5 shrink-0 text-zinc-400" />
      {/if}
      <span class="truncate text-zinc-400">{scope}</span>
      {#if place !== undefined}
        <ChevronIcon
          class="ml-auto h-3 w-3 shrink-0 -rotate-90 text-zinc-700 transition-colors group-hover:text-zinc-400"
        />
      {/if}
    </div>
    <div class="mt-1 flex items-baseline gap-1.5">
      {#if place !== undefined && field !== undefined}
        <span class={CARD_FIGURE}>#{place}</span>
        <span class="{CARD_CAPTION} {FIGURE}">/ {field}</span>
      {:else}
        <span class="text-sm font-medium text-zinc-400">Not ranked</span>
      {/if}
    </div>
    <div class="mt-1 {CARD_CAPTION}">
      {selectedMetricLabel} · {selectedMetricValue}{#if selectedStanding?.class}
        · {selectedStanding.class}
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
        class="group block rounded-xl border border-zinc-800/60 bg-zinc-900/30 p-3 transition-colors hover:border-zinc-700 hover:bg-zinc-900/60 focus:ring-2 focus:ring-zinc-500 focus:outline-none"
      >
        {@render standingContent(country, scope, place, field)}
      </a>
    {:else}
      <div class="rounded-xl border border-zinc-800/60 bg-zinc-900/30 p-3">
        {@render standingContent(country, scope, place, field)}
      </div>
    {/if}
  {/snippet}

  {#if athlete.standing}
    <div class="mb-6 sm:mb-8">
      <div class="mb-2 flex items-center justify-between gap-3">
        <h2 class={CARD_LABEL}>Ranking</h2>
        <label class="flex items-center gap-2 text-xs text-zinc-500">
          <span>Metric</span>
          <select bind:value={selectedMetric} class="{FIELD} px-2.5 py-1.5">
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
            ? boardQuery(selectedStanding.global.place, {
                ...(selectedMetric === 'ris'
                  ? {}
                  : {
                      movement: selectedMetric,
                      gender: athlete.gender,
                      category: selectedStanding.class ?? '',
                    }),
              })
            : undefined
        )}
        {@render standing(
          selectedCountry,
          countryName(selectedCountry),
          selectedStanding?.country.place,
          selectedStanding?.country.field,
          selectedStanding
            ? boardQuery(selectedStanding.country.place, {
                ...(selectedMetric === 'ris'
                  ? {}
                  : {
                      movement: selectedMetric,
                      gender: athlete.gender,
                      category: selectedStanding.class ?? '',
                    }),
                country: selectedCountry,
              })
            : undefined
        )}
      </div>
    </div>
  {/if}

  {#if athlete.personal_records && athlete.personal_records.length > 0}
    <div class="mb-6 sm:mb-8">
      <h2 class="mb-2 {CARD_LABEL}">Personal records</h2>
      <div class={CARD_GRID}>
        {#each sortPersonalRecords(athlete.personal_records) as pr (pr.movement_name)}
          <Card class="p-3 transition-colors hover:border-zinc-700/60">
            <div class={CARD_LABEL}>{pr.movement_name}</div>
            <div class="mt-0.5 {CARD_FIGURE}">{formatWeight(pr.max_weight)}</div>
            <div class={CARD_CAPTION}>
              <a
                href={resolve(`/competitions/${pr.competition_slug}`)}
                class="underline hover:text-zinc-300"
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

  <div class="mt-8 sm:mt-10">
    <h2 class="mb-2 {CARD_LABEL}">Competition history</h2>
    {#if athlete.competitions && athlete.competitions.length > 0}
      <Table>
        {#snippet head()}
          <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} {FROZEN_EDGE} text-zinc-400"
            >Rank</th
          >
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Competition</th>
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Total</th>
          <th class="{TABLE_HEAD_CELL} text-zinc-400"><RisHeader /></th>
          {#each contested as lift (lift.key)}
            <th class="{TABLE_HEAD_CELL} align-top text-zinc-400">
              {lift.label}
              <span class="{ATTEMPT_ROW} mt-1 text-[0.6rem] font-normal text-zinc-600">
                <span class="text-right">1</span>
                <span class="text-right">2</span>
                <span class="text-right">3</span>
                <span class="text-right">Best</span>
              </span>
            </th>
          {/each}
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Date</th>
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Class</th>
          {#if showsDivision}
            <th class="{TABLE_HEAD_CELL} text-zinc-400">Division</th>
          {/if}
        {/snippet}

        {#snippet body()}
          {#each athlete.competitions as competition (competition.competition_id)}
            <tr
              class="border-b border-zinc-800/50 transition-colors {competition.status !==
              'competed'
                ? 'opacity-50'
                : ''}"
            >
              <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {FROZEN_EDGE} {CELL.identity}">
                {#if competition.status !== 'competed'}
                  <span class={STATUS_FLAG} title={statusTitle(competition.status, null)}
                    >{STATUS_LABEL[competition.status]}</span
                  >
                {:else}
                  {competition.rank || NO_VALUE}
                {/if}
              </td>
              <td class={TABLE_CELL}>
                <a
                  href={resolve(`/competitions/${competition.competition_slug}`)}
                  class="{TEXT_CELL.competition} text-white underline hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:outline-none"
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
      <Card class="p-8">
        <p class="text-center text-zinc-400">No competition history available</p>
      </Card>
    {/if}
  </div>
</div>
