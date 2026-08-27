<script lang="ts">
  import type { PageData } from './$types';
  import {
    Card,
    Breadcrumb,
    Pagination,
    Flag,
    SearchInput,
    Table,
    TABLE_CELL,
    TABLE_HEAD_CELL,
  } from '$lib/components/ui';
  import { resolve } from '$app/paths';
  import { page, navigating } from '$app/state';
  import {
    formatDate,
    formatLocation,
    countryName,
    slugify,
    formatWeight,
    formatScore,
  } from '$lib/utils';
  import { CELL, STATUS_FLAG, SORTED_COLUMN, NO_VALUE, NO_RESULT } from '$lib/constants/table';
  import { GitHubIcon } from '$lib/components/icons';
  import { RANKING_MOVEMENTS, RANKING_SORTS, RANKING_GENDERS } from '$lib/constants/ranking';
  import { RankingsTable } from '$lib/state/rankings-table.svelte';
  import type { RankingEntry } from '$lib/types/ranking';
  import type { Attempt } from '$lib/types/competition';
  import { FIELD, TEXT } from '$lib/constants/typography';

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
          published ? 'entries.csv' : 'meet.toml',
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

  const table = new RankingsTable({
    basePath: `/competitions/${data.competition.slug}`,
    initialUrl: page.url,
  });

  const rankings = $derived(data.initialRankings);
  const pagination = $derived(data.pagination);
  const busy = $derived(navigating.to?.url.pathname === page.url.pathname);

  const movements = RANKING_MOVEMENTS;
  const sorts = RANKING_SORTS;
  const genders = RANKING_GENDERS;

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

  // Four fixed slots per movement, the three attempts then the best. Fixing the
  // widths is what lets the bold figure land at the same offset on every row,
  // so a column can be read straight down to compare athletes.
  const ATTEMPT_ROW =
    'grid grid-cols-[2.5rem_2.5rem_2.5rem_2.9rem] gap-x-1.5 items-baseline tabular-nums';

  type LiftCell =
    /** The competition never contested the movement. */
    | { kind: 'absent' }
    /** Contested, and every attempt failed. */
    | { kind: 'bombed' }
    /** The source published a best with no attempt breakdown behind it. */
    | { kind: 'best'; best: number }
    | { kind: 'attempts'; attempts: Attempt[]; best: number | null };

  function liftCell(entry: RankingEntry, key: string, code: string, movement: string): LiftCell {
    if (!entry.event?.includes(code)) return { kind: 'absent' };

    const best = entry[key as 'muscleup' | 'pullup' | 'dips' | 'squat'];
    const lift = participants
      .get(entry.athlete.athlete_id)
      ?.lifts.find((candidate) => candidate.movement_name === movement);

    if (lift?.attempts.length) {
      const attempts = [...lift.attempts].sort((a, b) => a.attempt_number - b.attempt_number);
      return { kind: 'attempts', attempts, best };
    }

    return best === null ? { kind: 'bombed' } : { kind: 'best', best };
  }
</script>

<svelte:head>
  <title>{competition.name} - OpenStreetlifting</title>
  <meta name="description" content="Results and details for {competition.name}" />
</svelte:head>

<div class="mx-auto max-w-[var(--content-max-width)] px-4 py-8 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[
      { label: 'Rankings', href: '/' },
      { label: 'Competitions', href: '/competitions' },
      { label: competition.name },
    ]}
  />

  <!-- Competition Header -->
  <div class="mb-12">
    <h1 class="mb-4 {TEXT.title} text-white">
      {competition.name}
    </h1>

    <div class="flex flex-wrap gap-x-6 gap-y-3 text-base text-zinc-400">
      <div class="flex items-center gap-2">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
          />
        </svg>
        <span>
          {#if competition.start_date}
            {formatDate(competition.start_date)}
            {#if competition.end_date && competition.start_date !== competition.end_date}
              - {formatDate(competition.end_date)}
            {/if}
          {/if}
        </span>
      </div>

      <div class="flex items-center gap-2">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
          />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
        <span>{formatLocation(competition.country, competition.region, competition.city)}</span>
      </div>

      <div class="flex items-center gap-2">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
          <circle cx="12" cy="9" r="6" />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M15.5 13.9 17 22l-5-3-5 3 1.5-8.1"
          />
        </svg>
        <span>{federationLabel}</span>
      </div>
    </div>

    {#if editPath}
      <a
        href={`https://github.com/openstreetlifting/openstreetlifting/edit/main/backend/data/competitions/${editPath}`}
        target="_blank"
        rel="noopener noreferrer"
        class="mt-6 inline-flex items-center gap-2 rounded-lg border border-zinc-800 bg-zinc-900/50 px-3 py-2 text-xs text-zinc-300 transition-colors hover:border-zinc-700 hover:text-white focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
      >
        <GitHubIcon class="h-4 w-4" />
        Edit on GitHub
      </a>
    {/if}
  </div>

  {#if published}
    <div
      class="mb-6 flex flex-wrap items-center gap-3 rounded-lg border border-zinc-800 bg-zinc-900/30 p-3"
    >
      <div class="w-full sm:w-64">
        <SearchInput
          bind:value={table.searchFilter}
          placeholder="Search an athlete"
          onSearch={() => table.handleFilterChange()}
        />
      </div>

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

      <div class="flex w-full items-center gap-2 sm:ml-auto sm:w-auto">
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
    </div>
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
          Page {pagination.page} of {pagination.total_pages} &middot; {pagination.total_items} athletes
        </span>
        <Pagination
          page={pagination.page}
          totalPages={pagination.total_pages}
          disabled={busy}
          onNavigate={(target) => table.goToPage(target)}
        />
      </div>
    {/snippet}

    <div class="mb-3">
      {@render paginationBar()}
    </div>

    <Table {busy}>
      {#snippet head()}
        <th class="{TABLE_HEAD_CELL} align-top text-zinc-400">Rank</th>
        <th class="{TABLE_HEAD_CELL} align-top text-zinc-400">Athlete</th>
        <th class="{TABLE_HEAD_CELL} align-top text-zinc-400">Sex</th>
        <th class="{TABLE_HEAD_CELL} align-top text-zinc-400">Class</th>
        {#each movements as movement (movement.value)}
          <th
            class="{TABLE_HEAD_CELL} align-top text-zinc-400 {table.movementFilter ===
            movement.value
              ? SORTED_COLUMN
              : ''}"
          >
            {movement.label}
            {#if movement.value !== 'total'}
              <span class="{ATTEMPT_ROW} mt-1 text-[0.6rem] font-normal text-zinc-600">
                <span class="text-right">1</span>
                <span class="text-right">2</span>
                <span class="text-right">3</span>
                <span class="text-right">Best</span>
              </span>
            {/if}
          </th>
        {/each}
        <th
          class="{TABLE_HEAD_CELL} align-top text-zinc-400 {table.movementFilter === 'ris'
            ? SORTED_COLUMN
            : ''}"
        >
          RIS
        </th>
      {/snippet}

      {#snippet body()}
        {#each rankings as entry (entry.rank + entry.athlete.athlete_id)}
          {@const participant = participants.get(entry.athlete.athlete_id)}
          <tr
            class="border-b border-zinc-800/50 transition-colors even:bg-zinc-900/60 hover:bg-zinc-800/50"
          >
            <td class="{TABLE_CELL} {CELL.identity}">
              {entry.rank}
            </td>
            <td class="{TABLE_CELL} {CELL.identity}">
              <a
                href={resolve(`/athletes/${entry.athlete.slug}`)}
                class="inline-flex max-w-[8rem] items-center gap-2.5 hover:text-zinc-300 sm:max-w-none"
              >
                <Flag
                  countryCode={entry.athlete.country}
                  class="-ml-1 shrink-0 [--flag-height:1.25em]"
                />
                <span class="truncate underline">
                  {entry.athlete.first_name}
                  {entry.athlete.last_name}
                </span>
              </a>
              {#if participant?.is_disqualified}
                <span
                  class="ml-1.5 align-middle text-[0.65rem] font-medium tracking-wide uppercase {STATUS_FLAG}"
                  title={participant.disqualified_reason ?? 'Disqualified'}
                >
                  DQ
                </span>
              {/if}
            </td>
            <td class="{TABLE_CELL} {CELL.data}">{entry.athlete.gender}</td>
            <td class="{TABLE_CELL} {CELL.data}">{entry.category}</td>
            {#each LIFTS as lift (lift.key)}
              {@const cell = liftCell(entry, lift.key, lift.code, lift.movement)}
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
            <td class="{TABLE_CELL} {CELL.counted}">{formatWeight(entry.total)}</td>
            <td class="{TABLE_CELL} {CELL.counted}">{formatScore(entry.ris)}</td>
          </tr>
        {/each}
      {/snippet}
    </Table>

    <div class="mt-3">
      {@render paginationBar()}
    </div>
  {/if}
</div>
