<script lang="ts">
  import type { PageData } from './$types';
  import type { PersonalRecord } from '$lib/types/athlete';
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
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import {
    formatDate,
    formatWeight,
    formatScore,
    formatAthleteName,
    countryName,
  } from '$lib/utils';
  import Seo from '$lib/components/seo.svelte';
  import { absolute, athleteLd, breadcrumbLd } from '$lib/seo';
  import { CELL, STATUS_FLAG, NO_VALUE, TEXT_CELL } from '$lib/constants/table';

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
  import { TEXT } from '$lib/constants/typography';

  let { data }: { data: PageData } = $props();
  const { athlete } = $derived(data);
  const showsDivision = $derived(athlete.competitions.some((c) => c.division));

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

  // Every card on this page is a label, a figure and a caption, at the same
  // three sizes, so a placing and a personal record read as the same kind of
  // fact rather than two designs sharing a page.
  const CARD_LABEL = `flex items-center gap-1.5 ${TEXT.micro} tracking-wider text-zinc-500 uppercase`;
  const CARD_FIGURE = 'text-xl font-semibold text-white tabular-nums sm:text-2xl';
  const CARD_CAPTION = 'text-xs text-zinc-500';
  const CARD_GRID = 'grid grid-cols-2 gap-3 sm:gap-4 md:grid-cols-4';

  // The board pages fifty at a time, so a place says which page it is on.
  const RANKING_PAGE_SIZE = 50;

  /**
   * A standing is a row on the board, so the card opens the board at it: the
   * same filters the place was computed under, the page it falls on, and the
   * slug that marks the row once there.
   */
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
  <Breadcrumb items={[{ label: 'Rankings', href: '/' }, { label: athleteName }]} />

  <div class="mb-6 sm:mb-10">
    <div class="flex items-center gap-3">
      <!-- The flag is sized off the heading rather than a fixed pixel height, so
           it lands on the cap height of the name at either breakpoint. -->
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

  <!-- The flag does the reading a country name makes you do. There is no flag
       for everyone, so the global card takes the globe from our own icon set
       rather than borrowing one that would look like a country.
       Scope and measure sit on the same line: a place is meaningless until you
       know which board it was taken on, so the card says both before the number. -->
  <!-- The flag says whose board it is and the glyph says everyone's, so neither
       needs spelling out; what does need saying is which of the two measures the
       place was taken on, which is why that sits on the label rather than under
       the number. There is no flag for everyone, so the global card takes the
       globe from our own icon set rather than one that would read as a country. -->
  {#snippet standing(
    country: string | null,
    basis: string,
    place: number,
    field: number,
    query: string
  )}
    <a
      href={resolve(`/?${query}`)}
      class="group block rounded-xl border border-zinc-800/60 bg-zinc-900/30 p-3 transition-colors hover:border-zinc-700 hover:bg-zinc-900/60 focus:ring-2 focus:ring-zinc-500 focus:outline-none"
    >
      <div class={CARD_LABEL}>
        {#if country}
          <Flag countryCode={country} class="shrink-0 [--flag-height:1.2em]" />
        {:else}
          <GlobeIcon class="h-3.5 w-3.5 shrink-0 text-zinc-400" />
          <span class="sr-only">Global</span>
        {/if}
        <span class="truncate text-zinc-400">{basis}</span>
        <ChevronIcon
          class="ml-auto h-3 w-3 shrink-0 -rotate-90 text-zinc-700 transition-colors group-hover:text-zinc-400"
        />
      </div>
      <div class="mt-1 flex items-baseline gap-1.5">
        <span class={CARD_FIGURE}>#{place}</span>
        <span class="{CARD_CAPTION} tabular-nums">/ {field}</span>
      </div>
    </a>
  {/snippet}

  {#if athlete.standing?.ris || athlete.standing?.weight_class}
    {@const ris = athlete.standing.ris}
    {@const inClass = athlete.standing.weight_class}
    {@const risBasis = ris?.score ? `RIS ${formatScore(ris.score)}` : 'RIS'}
    {@const inClassFilters = inClass
      ? {
          movement: 'total',
          category: inClass.class,
          ...(athlete.gender ? { gender: athlete.gender } : {}),
        }
      : {}}
    {@const classBasis = inClass?.total
      ? `${inClass.class} · ${formatWeight(inClass.total)} kg`
      : (inClass?.class ?? '')}
    <div class="mb-6 sm:mb-8">
      <h2 class="{CARD_LABEL} mb-2">Ranking</h2>
      <div class={CARD_GRID}>
        {#if ris}
          {@render standing(
            null,
            risBasis,
            ris.global.place,
            ris.global.field,
            boardQuery(ris.global.place)
          )}
        {/if}
        {#if inClass}
          {@render standing(
            null,
            classBasis,
            inClass.global.place,
            inClass.global.field,
            boardQuery(inClass.global.place, inClassFilters)
          )}
        {/if}

        {#if ris}
          {@render standing(
            ris.country.code,
            risBasis,
            ris.country.place,
            ris.country.field,
            boardQuery(ris.country.place, { country: ris.country.code })
          )}
        {/if}
        {#if inClass}
          {@render standing(
            inClass.country.code,
            classBasis,
            inClass.country.place,
            inClass.country.field,
            boardQuery(inClass.country.place, {
              ...inClassFilters,
              country: inClass.country.code,
            })
          )}
        {/if}
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
      <!-- The same table the rankings board is, at every width: the place stays
           pinned while the rest pans, and what the result was worth is read
           before the meta that describes it. -->
      <Table>
        {#snippet head()}
          <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} {FROZEN_EDGE} text-zinc-400"
            >Rank</th
          >
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Competition</th>
          <th class="{TABLE_HEAD_CELL} text-zinc-400">Total</th>
          <th class="{TABLE_HEAD_CELL} text-zinc-400"><RisHeader /></th>
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
              <td class="{TABLE_CELL} {CELL.counted}">
                {formatWeight(competition.total)}
              </td>
              <td class="{TABLE_CELL} {CELL.counted}">
                <RisScore value={competition.ris_score} source={competition.ris_source} />
              </td>
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
