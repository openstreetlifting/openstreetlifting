<script lang="ts">
  import type { PageData } from './$types';
  import {
    Card,
    Breadcrumb,
    Pagination,
    Flag,
    FilterBar,
    Table,
    RisHeader,
    RisScore,
    TABLE_CELL,
    TABLE_HEAD_CELL,
    FROZEN_CELL,
    FROZEN_HEAD_CELL,
    FROZEN_EDGE,
    FROZEN_RANK,
    ATHLETE_COLUMN,
    ATHLETE_CONTENT,
  } from '$lib/components/ui';
  import { resolve } from '$app/paths';
  import { page, navigating } from '$app/state';
  import { afterNavigate } from '$app/navigation';
  import {
    formatDate,
    formatLongDate,
    formatLocation,
    countryName,
    slugify,
    formatWeight,
    formatAthleteName,
  } from '$lib/utils';
  import {
    ATTEMPT_ROW,
    CELL,
    STATUS_FLAG,
    SORTED_COLUMN,
    NO_VALUE,
    NO_RESULT,
  } from '$lib/constants/table';
  import { GitHubIcon } from '$lib/components/icons';
  import {
    RANKING_SORTS,
    RANKING_SORTS_NO_RIS,
    RANKING_GENDERS,
    defaultRankingSort,
    hasRis,
  } from '$lib/constants/ranking';
  import { RankingsTable } from '$lib/state/rankings-table.svelte';
  import type { RankingEntry } from '$lib/types/ranking';
  import type { Attempt, Participant, CategoryDetail } from '$lib/types/competition';
  import { FIELD, TEXT } from '$lib/constants/typography';
  import Seo from '$lib/components/seo.svelte';
  import { breadcrumbLd, competitionLd, listingSeo } from '$lib/seo';

  let { data }: { data: PageData } = $props();
  const competition = $derived(data.competition);

  const published = $derived(competition.categories.length > 0);

  // Files live at data/competitions/{federation}/{year}/{slug}/, which the
  // importer enforces. The year is read off the string rather than a Date, so a
  // viewer west of UTC does not shift a January competition into the previous year.
  const editPath = $derived(
    competition.start_date
      ? [
          slugify(competition.federation.name),
          competition.start_date.slice(0, 4),
          competition.slug,
          published ? 'entries.csv' : 'competition.toml',
        ].join('/')
      : null
  );

  // Most federations are known by an acronym alone, so there is nothing to put
  // in brackets after it.
  const federationLabel = $derived(
    competition.federation.abbreviation &&
      competition.federation.abbreviation !== competition.federation.name
      ? `${competition.federation.name} (${competition.federation.abbreviation})`
      : competition.federation.name
  );

  const risAvailable = $derived(hasRis(data.competition.movements.length));

  const table = new RankingsTable({
    basePath: `/competitions/${data.competition.slug}`,
    initialUrl: page.url,
    defaultSort: defaultRankingSort(data.competition.movements.length),
  });

  afterNavigate(() => table.syncFromUrl(page.url));

  const rankings = $derived(data.initialRankings);
  const pagination = $derived(data.pagination);
  const busy = $derived(navigating.to?.url.pathname === page.url.pathname);

  const sorts = $derived(risAvailable ? RANKING_SORTS : RANKING_SORTS_NO_RIS);
  const genders = RANKING_GENDERS;

  const sorted = (column: string) => (table.movementFilter === column ? SORTED_COLUMN : '');

  const activeFilters = $derived(
    [table.countryFilter, table.genderFilter, table.categoryFilter].filter(Boolean).length
  );

  // The event code names the movements a competition contested, so a column with no
  // weight can be read as bombed rather than never lifted.
  const LIFTS = [
    { key: 'muscleup', code: 'M', movement: 'Muscle-up', label: 'Muscle Up' },
    { key: 'pullup', code: 'P', movement: 'Pull-up', label: 'Pull Up' },
    { key: 'dips', code: 'D', movement: 'Dips', label: 'Dips' },
    { key: 'squat', code: 'S', movement: 'Squat', label: 'Squat' },
  ] as const;

  // The rankings response carries a best per movement; the attempts behind it
  // arrive with the competition itself, keyed by athlete.
  const participants = $derived(
    new Map(
      competition.categories
        .flatMap((category) => category.participants)
        .map((participant) => [participant.athlete.athlete_id, participant])
    )
  );

  type LiftCell =
    /** The competition never contested the movement. */
    | { kind: 'absent' }
    /** Contested, and every attempt failed. */
    | { kind: 'bombed' }
    /** The source published a best with no attempt breakdown behind it. */
    | { kind: 'best'; best: number }
    | { kind: 'attempts'; attempts: Attempt[]; best: number | null };

  function liftCell(
    participant: Participant | undefined,
    best: number | null,
    code: string,
    movement: string
  ): LiftCell {
    if (!event?.includes(code)) return { kind: 'absent' };

    const lift = participant?.lifts.find((candidate) => candidate.movement_name === movement);

    if (lift?.attempts.length) {
      const attempts = [...lift.attempts].sort((a, b) => a.attempt_number - b.attempt_number);
      return { kind: 'attempts', attempts, best };
    }

    return best === null ? { kind: 'bombed' } : { kind: 'best', best };
  }

  function rankedCell(entry: RankingEntry, key: string, code: string, movement: string): LiftCell {
    const best = entry[key as 'muscleup' | 'pullup' | 'dips' | 'squat'];
    return liftCell(participants.get(entry.athlete.athlete_id), best, code, movement);
  }

  function participantCell(participant: Participant, code: string, movement: string): LiftCell {
    const lift = participant.lifts.find((candidate) => candidate.movement_name === movement);
    const best = lift?.best_weight == null ? null : Number(lift.best_weight);
    return liftCell(participant, best, code, movement);
  }

  // Which movements the competition ran. Every row shares it, and a lifter with
  // no lifts at all has nothing of their own to read it from.
  const event = $derived(competition.event_code ?? null);

  const contested = $derived(
    event ? LIFTS.filter((lift) => event.includes(lift.code)) : [...LIFTS]
  );

  const eventLegend = $derived(
    competition.movements
      .map((movement) => `${movement.code ?? '?'} ${movement.movement_name}`)
      .join(' · ')
  );

  // The rankings query joins through lifts and keeps only competed lifters, so
  // it can never return these. The competition's own results list them, and a
  // competition page is a record of who turned up, not a leaderboard.
  // Disqualified before no_show: one turned up and lifted, the other never did.
  const NOT_PLACED_ORDER = ['disqualified', 'no_show'];

  const notPlaced = $derived(
    competition.categories
      .flatMap((category: CategoryDetail) =>
        category.participants
          .filter((participant) => participant.status !== 'competed')
          .map((participant) => ({ category, participant }))
      )
      // The ranked half is filtered by the server, so these have to answer the
      // same filters or the table would contradict itself.
      .filter(({ category, participant }) => {
        const { athlete } = participant;
        const search = table.searchFilter.trim().toLowerCase();
        return (
          (!table.genderFilter || athlete.gender === table.genderFilter) &&
          (!table.countryFilter || athlete.country === table.countryFilter) &&
          (!table.categoryFilter || category.category.weight_class === table.categoryFilter) &&
          (!search || formatAthleteName(athlete).toLowerCase().includes(search))
        );
      })
      .sort(
        (a, b) =>
          NOT_PLACED_ORDER.indexOf(a.participant.status) -
            NOT_PLACED_ORDER.indexOf(b.participant.status) ||
          a.participant.athlete.last_name.localeCompare(b.participant.athlete.last_name)
      )
  );

  // The pagination count comes from the ranked query, so it has to be told
  // about the lifters the table shows underneath it.
  const fieldSize = $derived(pagination.total_items + notPlaced.length);

  const onLastPage = $derived(pagination.page >= pagination.total_pages);

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

  const seo = $derived(listingSeo(page.url));

  // Half the meets are named after the federation alone, so the year is what
  // tells one edition from the next in a search result.
  const seoYear = $derived(competition.start_date?.slice(0, 4) ?? '');
  const seoName = $derived(
    seoYear && !competition.name.includes(seoYear)
      ? `${competition.name} ${seoYear}`
      : competition.name
  );

  const lifterCount = $derived(
    competition.categories.reduce((total, category) => total + category.participants.length, 0)
  );

  const seoWhere = $derived(
    formatLocation(competition.city, competition.country && countryName(competition.country))
  );

  const seoMovements = $derived(
    competition.movements.map((movement) => movement.movement_name.toLowerCase()).join(', ')
  );

  const seoDescription = $derived(
    [
      published
        ? `Full results and standings from ${competition.name}`
        : `${competition.name}, a ${federationLabel} streetlifting competition`,
      competition.start_date ? `, ${formatLongDate(competition.start_date)}` : '',
      seoWhere ? `, ${seoWhere}` : '',
      published
        ? `. ${lifterCount} lifters ranked on ${seoMovements}.`
        : '. Entered in the OpenStreetlifting calendar.',
    ].join('')
  );
</script>

<Seo
  title={published ? `${seoName} results` : seoName}
  description={seoDescription}
  canonical={seo.canonical}
  noindex={seo.noindex}
  jsonLd={[
    competitionLd(competition, seoDescription),
    breadcrumbLd([
      { name: 'Rankings', path: '/' },
      { name: 'Competitions', path: '/competitions' },
      { name: competition.name, path: `/competitions/${competition.slug}` },
    ]),
  ]}
/>

<div class="mx-auto max-w-[var(--content-max-width)] px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[
      { label: 'Rankings', href: '/' },
      { label: 'Competitions', href: '/competitions' },
      { label: competition.name },
    ]}
  />

  <div class="mb-6 sm:mb-10">
    <h1 class="{TEXT.title} flex min-w-0 items-center gap-3 text-white">
      {#if competition.country}
        <Flag countryCode={competition.country} class="shrink-0 [--flag-height:0.8em]" />
      {/if}
      <span class="truncate">{competition.name}</span>
    </h1>

    <p class="mt-2 flex flex-wrap items-center gap-x-2 text-xs text-zinc-400 sm:text-sm">
      {#if competition.start_date}
        <span class="whitespace-nowrap">
          {formatDate(competition.start_date)}
          {#if competition.end_date && competition.end_date !== competition.start_date}
            - {formatDate(competition.end_date)}
          {/if}
        </span>
      {/if}
      {#if formatLocation(competition.city, competition.region)}
        <span aria-hidden="true">&middot;</span>
        <span>{formatLocation(competition.city, competition.region)}</span>
      {/if}
      <span aria-hidden="true">&middot;</span>
      <span title={competition.federation.name}>{federationLabel}</span>
      {#if published}
        <span aria-hidden="true">&middot;</span>
        <span class="whitespace-nowrap">{lifterCount} lifters</span>
      {/if}
    </p>

    {#if event}
      <p class="mt-2 flex items-center gap-2 text-xs text-zinc-500 sm:text-sm">
        Format
        <span
          class="inline-flex items-center rounded border border-zinc-800 px-1.5 py-0.5 font-mono text-[0.7rem] tracking-wider text-zinc-400 sm:text-xs"
          title={eventLegend}
          aria-label="Format: {eventLegend}"
        >
          {event}
        </span>
      </p>
    {/if}

    {#if editPath}
      <a
        href={`https://github.com/openstreetlifting/openstreetlifting/edit/main/backend/data/competitions/${editPath}`}
        target="_blank"
        rel="noopener noreferrer"
        class="mt-3 inline-flex items-center gap-1.5 rounded-md border border-zinc-800 px-2 py-1 text-xs text-zinc-400 transition-colors hover:border-zinc-700 hover:bg-zinc-900 hover:text-white focus:ring-2 focus:ring-zinc-500 focus:outline-none sm:px-2.5 sm:py-1.5 sm:text-sm"
      >
        <GitHubIcon class="h-3.5 w-3.5 sm:h-4 sm:w-4" />
        Edit on GitHub
      </a>
    {/if}
  </div>

  {#if published}
    <FilterBar
      bind:search={table.searchFilter}
      placeholder="Search an athlete"
      onSearch={() => table.handleFilterChange()}
      activeCount={activeFilters}
      onClear={() => table.clearFilters()}
      clearable={table.narrowed}
    >
      <select
        bind:value={table.countryFilter}
        onchange={() => table.handleFilterChange()}
        class="w-full {FIELD} px-3 py-2 sm:w-auto"
      >
        <option value={null}>All Countries</option>
        {#each data.countries as countryOption (countryOption)}
          <option value={countryOption}>{countryName(countryOption)}</option>
        {/each}
      </select>
      <select
        bind:value={table.genderFilter}
        onchange={() => {
          table.categoryFilter = null;
          table.handleFilterChange();
        }}
        class="w-full {FIELD} px-3 py-2 sm:w-auto"
      >
        {#each genders as gender (gender.label)}
          <option value={gender.value}>{gender.label}</option>
        {/each}
      </select>
      <select
        bind:value={table.categoryFilter}
        onchange={() => table.handleFilterChange()}
        class="w-full {FIELD} px-3 py-2 sm:w-auto"
      >
        <option value={null}>All Classes</option>
        {#each data.classes as classOption (classOption)}
          <option value={classOption}>{classOption}</option>
        {/each}
      </select>

      <div class="flex w-full items-center gap-2 sm:w-auto">
        <label for="sort-by" class="text-sm text-zinc-500">Sort by</label>
        <select
          id="sort-by"
          bind:value={table.movementFilter}
          onchange={() => table.handleFilterChange()}
          class="flex-1 {FIELD} px-3 py-2 sm:flex-none"
        >
          {#each sorts as sort (sort.value)}
            <option value={sort.value}>{sort.label}</option>
          {/each}
        </select>
      </div>
    </FilterBar>
  {/if}

  {#if !published}
    <div class="border-l-2 border-zinc-700 py-1 pl-5">
      <p class="text-lg text-white">This competition has not been lifted yet.</p>
      <p class="mt-2 max-w-2xl text-sm leading-relaxed text-zinc-400">
        {competition.name} is scheduled for {formatDate(competition.start_date)}. There is nothing
        to rank until the platform closes, and the results will land on this page once they are in.
      </p>
    </div>
  {:else if rankings.length === 0 && !busy}
    <Card class="p-8">
      <div class="text-center">
        <p class="text-zinc-400">No results found for the selected filters</p>
        <button
          onclick={() => table.clearFilters()}
          class="mt-4 text-sm text-zinc-500 underline hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
        >
          Clear filters
        </button>
      </div>
    </Card>
  {:else}
    {#snippet paginationBar()}
      <div class="flex flex-wrap items-center justify-between gap-3">
        <span class="text-xs text-zinc-500">
          Page {pagination.page} of {pagination.total_pages} &middot; {fieldSize} athletes
        </span>
        <Pagination
          page={pagination.page}
          totalPages={pagination.total_pages}
          disabled={busy}
          onNavigate={(target) => table.goToPage(target)}
        />
      </div>
    {/snippet}

    <div class="hidden sm:mb-3 sm:block">
      {@render paginationBar()}
    </div>

    <Table {busy}>
      {#snippet head()}
        <th
          class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} {FROZEN_EDGE} align-top text-zinc-400"
          >Rank</th
        >
        <th class="{TABLE_HEAD_CELL} {ATHLETE_COLUMN} align-top text-zinc-400">Athlete</th>
        <th class="{TABLE_HEAD_CELL} align-top text-zinc-400 {sorted('total')}">Total</th>
        {#if risAvailable}
          <th class="{TABLE_HEAD_CELL} align-top text-zinc-400 {sorted('ris')}">
            <RisHeader />
          </th>
        {/if}
        {#each contested as lift (lift.key)}
          <th class="{TABLE_HEAD_CELL} align-top text-zinc-400 {sorted(lift.key)}">
            {lift.label}
            <span class="{ATTEMPT_ROW} mt-1 text-[0.6rem] font-normal text-zinc-600">
              <span class="text-right">1</span>
              <span class="text-right">2</span>
              <span class="text-right">3</span>
              <span class="text-right">Best</span>
            </span>
          </th>
        {/each}
        <th class="{TABLE_HEAD_CELL} align-top text-zinc-400">Sex</th>
        <th class="{TABLE_HEAD_CELL} align-top text-zinc-400">Class</th>
      {/snippet}

      {#snippet body()}
        {#each rankings as entry (entry.rank + entry.athlete.athlete_id)}
          {@const participant = participants.get(entry.athlete.athlete_id)}
          <tr class="border-b border-zinc-800/50 transition-colors">
            <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {FROZEN_EDGE} {CELL.identity}">
              {entry.rank}
            </td>
            <td class="{TABLE_CELL} {ATHLETE_COLUMN} {CELL.identity}">
              <span class="flex items-center gap-1.5 {ATHLETE_CONTENT}">
                <a
                  href={resolve(`/athletes/${entry.athlete.slug}`)}
                  class="flex min-w-0 items-center gap-2.5 hover:text-zinc-300"
                >
                  <Flag
                    countryCode={entry.athlete.country}
                    class="shrink-0 [--flag-height:1.25em]"
                  />
                  <span class="truncate underline">
                    {formatAthleteName(entry.athlete)}
                  </span>
                </a>
                {#if participant && participant.status !== 'competed'}
                  <span
                    class="shrink-0 text-[0.65rem] font-medium tracking-wide uppercase {STATUS_FLAG}"
                    title={statusTitle(participant.status, participant.status_reason)}
                  >
                    {STATUS_LABEL[participant.status]}
                  </span>
                {/if}
              </span>
            </td>
            <td class="{TABLE_CELL} {CELL.counted}">{formatWeight(entry.total)}</td>
            {#if risAvailable}
              <td class="{TABLE_CELL} {CELL.counted}">
                <RisScore value={entry.ris} source={entry.ris_source} />
              </td>
            {/if}
            {#each contested as lift (lift.key)}
              {@const cell = rankedCell(entry, lift.key, lift.code, lift.movement)}
              <td class="{TABLE_CELL} whitespace-nowrap">
                <span class={ATTEMPT_ROW}>
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

                  <!-- The best sits in its own slot at a fixed offset, so the
                       column can be read straight down to compare athletes. -->
                  <span class="text-right {CELL.counted}">
                    {#if cell.kind === 'absent'}
                      <span class="font-normal {CELL.absent}">{NO_VALUE}</span>
                    {:else if cell.kind === 'bombed'}
                      <span
                        class="font-normal {CELL.nothing}"
                        title="No successful {lift.label.toLowerCase()}">{NO_RESULT}</span
                      >
                    {:else}
                      {cell.best}
                    {/if}
                  </span>
                </span>
              </td>
            {/each}
            <td class="{TABLE_CELL} {CELL.data}">{entry.athlete.gender}</td>
            <td class="{TABLE_CELL} {CELL.data}">{entry.category}</td>
          </tr>
        {/each}

        {#if onLastPage}
          {#each notPlaced as { category, participant } (participant.athlete.athlete_id)}
            <tr class="border-b border-zinc-800/50 transition-colors">
              <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {FROZEN_EDGE} {CELL.absent}"
                >{NO_VALUE}</td
              >
              <td class="{TABLE_CELL} {ATHLETE_COLUMN} {CELL.identity}">
                <span class="flex items-center gap-1.5 {ATHLETE_CONTENT}">
                  <a
                    href={resolve(`/athletes/${participant.athlete.slug}`)}
                    class="flex min-w-0 items-center gap-2.5 hover:text-zinc-300"
                  >
                    <Flag
                      countryCode={participant.athlete.country}
                      class="shrink-0 [--flag-height:1.25em]"
                    />
                    <span class="truncate underline">
                      {formatAthleteName(participant.athlete)}
                    </span>
                  </a>
                  <span
                    class="shrink-0 text-[0.65rem] font-medium tracking-wide uppercase {STATUS_FLAG}"
                    title={statusTitle(participant.status, participant.status_reason)}
                  >
                    {STATUS_LABEL[participant.status]}
                  </span>
                </span>
              </td>
              <td class="{TABLE_CELL} {CELL.nothing}">{NO_RESULT}</td>
              {#if risAvailable}
                <td class="{TABLE_CELL} {CELL.nothing}">{NO_RESULT}</td>
              {/if}
              {#each contested as lift (lift.key)}
                {@const cell = participantCell(participant, lift.code, lift.movement)}
                <td class="{TABLE_CELL} whitespace-nowrap">
                  <span class={ATTEMPT_ROW}>
                    {#if cell.kind === 'attempts'}
                      {#each [1, 2, 3] as slot (slot)}
                        {@const attempt = cell.attempts.find((a) => a.attempt_number === slot)}
                        <span
                          class="text-right {attempt && !attempt.is_successful
                            ? CELL.discounted
                            : CELL.data}"
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
                      {:else if cell.kind === 'bombed'}
                        <span class="font-normal {CELL.nothing}">{NO_RESULT}</span>
                      {:else}
                        {cell.best}
                      {/if}
                    </span>
                  </span>
                </td>
              {/each}
              <td class="{TABLE_CELL} {CELL.data}">{category.category.gender}</td>
              <td class="{TABLE_CELL} {CELL.data}">{category.category.weight_class}</td>
            </tr>
          {/each}
        {/if}
      {/snippet}
    </Table>

    <div class="mt-3">
      {@render paginationBar()}
    </div>
  {/if}
</div>
