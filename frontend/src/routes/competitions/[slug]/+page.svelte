<script lang="ts">
  import type { PageData } from './$types';
  import {
    RankingsEmpty,
    Breadcrumb,
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
  import { rankingsHref } from '$lib/state/rankings-return.svelte';
  import { slowNavigation } from '$lib/state/slow-navigation.svelte';
  import { page, navigating } from '$app/state';
  import { afterNavigate } from '$app/navigation';
  import {
    formatDate,
    formatLongDate,
    formatLocation,
    countryName,
    slugify,
    federationPath,
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
    REPORTED_MARK,
    REPORTED_GLYPH,
    REPORTED_TITLE,
  } from '$lib/constants/table';
  import { GitHubIcon } from '$lib/components/icons';
  import {
    RANKING_SORTS,
    RANKING_SORTS_NO_RIS,
    RANKING_GENDERS,
    defaultRankingSort,
  } from '$lib/constants/ranking';
  import { RankingsTable } from '$lib/state/rankings-table.svelte';
  import type { RankingEntry } from '$lib/types/ranking';
  import type { Attempt, Participant, CategoryDetail } from '$lib/types/competition';
  import { GENDERS, type AthleteStatus } from '$lib/types/enums';
  import { ATHLETE_STATUS_LABEL, athleteStatusTitle } from '$lib/constants/athlete-status';
  import { FIELD, TEXT } from '$lib/constants/typography';
  import Seo from '$lib/components/seo.svelte';
  import {
    breadcrumbLd,
    competitionLd,
    competitionSeoName,
    competitionTitle,
    listingSeo,
  } from '$lib/seo';

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

  const federationLabel = $derived(
    competition.federation.abbreviation &&
      competition.federation.abbreviation !== competition.federation.name
      ? `${competition.federation.name} (${competition.federation.abbreviation})`
      : competition.federation.name
  );

  // The column follows the format the meet contested, the sort follows the data
  // we hold, so an All4 meet with no published bodyweight keeps an empty column.
  const risColumn = $derived(data.ris !== 'not-contested');
  const risSortable = $derived(data.ris === 'recomputed' || data.ris === 'reported');

  const classed = $derived(
    competition.categories.some(({ category }) => category.weight_class !== '')
  );

  const table = new RankingsTable({
    basePath: () => `/competitions/${data.competition.slug}`,
    initialUrl: page.url,
    defaultSort: () => defaultRankingSort(data.ris),
  });

  afterNavigate(({ type }) => {
    table.syncFromUrl(page.url, type);
    if (table.focusedAthlete) {
      requestAnimationFrame(() => {
        document.querySelector('[data-focused]')?.scrollIntoView({ block: 'center' });
      });
    }
  });

  const rankings = $derived(data.initialRankings);
  const pagination = $derived(data.pagination);
  const loading = slowNavigation(() => navigating.to?.url.pathname === page.url.pathname);
  const busy = $derived(loading.current);

  const sorts = $derived(risSortable ? RANKING_SORTS : RANKING_SORTS_NO_RIS);
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

  const formatLabel = $derived(
    competition.movements.map((movement) => movement.movement_name).join(', ')
  );

  const competitionDate = new Intl.DateTimeFormat('en-GB', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
    timeZone: 'UTC',
  });

  const location = $derived(formatLocation(competition.city, competition.region));

  // Use the complete results, not the filtered ranking facets. A class can
  // appear in several divisions but only needs to be listed once per sex.
  const representedClasses = $derived(
    GENDERS.map((gender) => {
      const categories = competition.categories
        .filter(({ category }) => category.gender === gender)
        .map(({ category }) => category)
        .sort(
          (a, b) =>
            Number(a.weight_class_max ?? Infinity) - Number(b.weight_class_max ?? Infinity) ||
            Number(a.weight_class_min ?? 0) - Number(b.weight_class_min ?? 0)
        );

      return {
        gender,
        label: { M: 'Men', F: 'Women', MX: 'Mixed' }[gender],
        classes: [...new Set(categories.map((category) => category.weight_class))],
      };
    }).filter(({ classes }) => classes.length > 0)
  );

  // The rankings query joins through lifts and keeps only competed lifters, so
  // it can never return these. The competition's own results list them, and a
  // competition page is a record of who turned up, not a leaderboard.
  // Disqualified before no_show: one turned up and lifted, the other never did.
  const NOT_PLACED_ORDER: AthleteStatus[] = ['disqualified', 'no_show'];

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
          (!classed ||
            !table.categoryFilter ||
            category.category.weight_class === table.categoryFilter) &&
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

  const fieldSize = $derived(pagination.total_items + notPlaced.length);

  const fieldPages = $derived(Math.max(1, Math.ceil(fieldSize / pagination.page_size)));
  const unplacedOnPage = $derived(
    notPlaced.slice(
      Math.max(0, (pagination.page - 1) * pagination.page_size - pagination.total_items),
      Math.max(0, pagination.page * pagination.page_size - pagination.total_items)
    )
  );

  const seo = $derived(listingSeo(page.url));

  const seoName = $derived(competitionSeoName(competition));
  const federationHref = $derived(federationPath(competition.federation.name));

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
        : `${competition.name}, a ${federationLabel} Streetlifting competition`,
      competition.start_date ? `, ${formatLongDate(competition.start_date)}` : '',
      seoWhere ? `, ${seoWhere}` : '',
      published
        ? `. ${lifterCount} lifters ranked on ${seoMovements}.`
        : '. Entered in the OpenStreetlifting calendar.',
    ].join('')
  );
</script>

<Seo
  title={competitionTitle(seoName, published)}
  description={seoDescription}
  canonical={seo.canonical}
  noindex={seo.noindex}
  jsonLd={[
    competitionLd(competition, seoDescription),
    breadcrumbLd([
      { name: 'Rankings', path: '/' },
      { name: 'Competitions', path: '/competitions' },
      { name: competition.federation.name, path: federationHref },
      { name: competition.name, path: `/competitions/${competition.slug}` },
    ]),
  ]}
/>

<div class="mx-auto max-w-page px-4 py-4 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[
      { label: 'Rankings', href: rankingsHref() },
      { label: 'Competitions', href: '/competitions' },
      { label: competition.federation.name, href: federationHref },
      { label: competition.name },
    ]}
  />

  <header
    class="mb-6 grid grid-cols-1 gap-y-4 border-b border-stroke pb-5 sm:grid-cols-[minmax(0,1fr)_auto] sm:gap-x-6"
  >
    <h1 class="{TEXT.title} flex min-w-0 items-start gap-3 text-ink">
      {#if competition.country}
        <Flag
          countryCode={competition.country}
          link
          class="mt-[0.2em] shrink-0 [--flag-height:0.8em]"
        />
      {/if}
      <span class="min-w-0 break-words">{competition.name}</span>
    </h1>

    <dl
      class="grid grid-cols-[6rem_minmax(0,1fr)] gap-x-3 gap-y-1.5 text-sm sm:col-span-2 sm:grid-cols-[7rem_minmax(0,1fr)]"
    >
      {#if competition.start_date}
        <dt class="text-muted">Date</dt>
        <dd class="text-ink">
          <time datetime={competition.start_date}>
            {competitionDate.format(new Date(competition.start_date))}
          </time>
          {#if competition.end_date && competition.end_date !== competition.start_date}
            <span class="text-muted">–</span>
            <time datetime={competition.end_date}>
              {competitionDate.format(new Date(competition.end_date))}
            </time>
          {/if}
        </dd>
      {/if}
      {#if location}
        <dt class="text-muted">Location</dt>
        <dd class="text-ink">{location}</dd>
      {/if}
      <dt class="text-muted">Federation</dt>
      <dd class="min-w-0 text-ink">
        <a
          href={resolve(federationHref)}
          title={competition.federation.name}
          class="break-words underline decoration-stroke-strong underline-offset-2 hover:text-secondary"
          >{federationLabel}</a
        >
      </dd>
      {#if formatLabel}
        <dt class="text-muted">Format</dt>
        <dd class="text-ink">{formatLabel}</dd>
      {/if}
      {#if published}
        <dt class="text-muted">RIS</dt>
        <dd class="text-ink">
          {#if data.ris === 'recomputed'}
            Recomputed
          {:else if data.ris === 'reported'}
            Reported by the federation<span
              class={REPORTED_MARK}
              title={REPORTED_TITLE}
              aria-label={REPORTED_TITLE}>{REPORTED_GLYPH}</span
            >
          {:else if data.ris === 'not-published'}
            Not published, and no bodyweight to compute one, so the table ranks on total.
          {:else}
            None
          {/if}
        </dd>
      {/if}
      {#if published && !classed}
        <dt class="text-muted">Weight classes</dt>
        <dd class="text-ink">No weight classes</dd>
      {/if}
    </dl>

    {#if published && classed}
      <section aria-labelledby="weight-classes-heading" class="sm:col-span-2">
        <h2 id="weight-classes-heading" class="mb-2 text-xs font-medium text-secondary">
          Weight classes
        </h2>
        <dl
          class="grid grid-cols-[6rem_minmax(0,1fr)] gap-x-3 gap-y-1.5 text-sm sm:grid-cols-[7rem_minmax(0,1fr)]"
        >
          {#each representedClasses as group (group.gender)}
            <dt class="text-muted">{group.label}</dt>
            <dd>
              <ul
                class="flex flex-wrap gap-x-2 gap-y-1 text-ink"
                aria-label="{group.label}'s weight classes"
              >
                {#each group.classes as weightClass, i (weightClass)}
                  <li class="inline-flex items-baseline gap-2 whitespace-nowrap tabular-nums">
                    {#if i > 0}<span aria-hidden="true" class="text-muted">·</span>{/if}
                    {weightClass || 'No weight classes'}
                  </li>
                {/each}
              </ul>
            </dd>
          {/each}
        </dl>
      </section>
    {/if}

    {#if editPath}
      <a
        href={`https://github.com/openstreetlifting/openstreetlifting/edit/main/backend/data/competitions/${editPath}`}
        target="_blank"
        rel="noopener noreferrer"
        class="inline-flex min-h-11 items-center gap-2 justify-self-start rounded-md border border-stroke px-3 py-2 text-sm font-medium text-secondary transition-colors hover:border-stroke-strong hover:bg-surface-hover hover:text-ink focus:ring-2 focus:ring-focus focus:outline-none sm:col-start-2 sm:row-start-1 sm:self-start sm:justify-self-end"
      >
        <GitHubIcon class="h-4 w-4" />
        Edit competition on GitHub
      </a>
    {/if}
  </header>

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
      {#if classed}
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
      {/if}

      <div class="flex w-full items-center gap-2 sm:w-auto">
        <label for="sort-by" class="text-sm text-muted">Sort by</label>
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
    <div class="border-l-2 border-stroke-strong py-1 pl-5">
      <p class="text-lg text-ink">This competition has not been lifted yet.</p>
      <p class="mt-2 max-w-2xl text-sm leading-relaxed text-secondary">
        {competition.name} is scheduled for {formatDate(competition.start_date)}. There is nothing
        to rank until the platform closes, and the results will land on this page once they are in.
      </p>
    </div>
  {:else if rankings.length === 0 && unplacedOnPage.length === 0 && !busy}
    <RankingsEmpty
      canReset={table.narrowed || pagination.page > 1}
      resetHref={resolve(`/competitions/${competition.slug}`)}
    />
  {:else}
    <Table
      rows={rankings}
      itemName="athlete"
      {busy}
      pagination={{
        ...pagination,
        total_items: fieldSize,
        total_pages: fieldPages,
        pageHref: (target) => table.pageHref(target),
        replaceState: true,
      }}
    >
      {#snippet head()}
        <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} {FROZEN_EDGE} text-secondary"
          >Rank</th
        >
        <th class="{TABLE_HEAD_CELL} {ATHLETE_COLUMN} text-secondary">Athlete</th>
        <th class="{TABLE_HEAD_CELL} text-secondary {sorted('total')}">Total</th>
        {#if risColumn}
          <th class="{TABLE_HEAD_CELL} text-secondary {sorted('ris')}">
            <RisHeader />
          </th>
        {/if}
        {#each contested as lift (lift.key)}
          <th class="{TABLE_HEAD_CELL} text-secondary {sorted(lift.key)}">
            {lift.label}
            <span class="{ATTEMPT_ROW} mt-1 text-[0.6rem] font-normal text-muted">
              <span class="text-right">1</span>
              <span class="text-right">2</span>
              <span class="text-right">3</span>
              <span class="text-right">Best</span>
            </span>
          </th>
        {/each}
        <th class="{TABLE_HEAD_CELL} text-secondary">Sex</th>
        {#if classed}
          <th class="{TABLE_HEAD_CELL} text-secondary">Class</th>
        {/if}
      {/snippet}

      {#snippet body(rows)}
        {#each rows as entry (entry.rank + entry.athlete.athlete_id)}
          {@const participant = participants.get(entry.athlete.athlete_id)}
          <tr
            class="transition-colors"
            data-focused={entry.athlete.slug === table.focusedAthlete ? '' : undefined}
          >
            <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {FROZEN_EDGE} {CELL.identity}">
              {entry.rank}
            </td>
            <td class="{TABLE_CELL} {ATHLETE_COLUMN} {CELL.identity}">
              <span class="flex items-center gap-1.5 {ATHLETE_CONTENT}">
                <span class="flex min-w-0 items-center gap-2.5">
                  <Flag
                    link
                    countryCode={entry.athlete.country}
                    class="shrink-0 [--flag-height:1.25em]"
                  />
                  <a
                    href={resolve(`/athletes/${entry.athlete.slug}`)}
                    class="truncate underline hover:text-secondary"
                  >
                    {formatAthleteName(entry.athlete)}
                  </a>
                </span>
                {#if participant && participant.status !== 'competed'}
                  <span
                    class="shrink-0 text-[0.65rem] font-medium tracking-wide uppercase {STATUS_FLAG}"
                    title={athleteStatusTitle(participant.status, participant.status_reason)}
                  >
                    {ATHLETE_STATUS_LABEL[participant.status]}
                  </span>
                {/if}
              </span>
            </td>
            <td class="{TABLE_CELL} {CELL.counted}">{formatWeight(entry.total)}</td>
            {#if risColumn}
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
            {#if classed}
              <td class="{TABLE_CELL} {CELL.data}">{entry.category}</td>
            {/if}
          </tr>
        {/each}

        {#each unplacedOnPage as { category, participant } (participant.athlete.athlete_id)}
          <tr class="transition-colors">
            <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {FROZEN_EDGE} {CELL.absent}"
              >{NO_VALUE}</td
            >
            <td class="{TABLE_CELL} {ATHLETE_COLUMN} {CELL.identity}">
              <span class="flex items-center gap-1.5 {ATHLETE_CONTENT}">
                <span class="flex min-w-0 items-center gap-2.5">
                  <Flag
                    link
                    countryCode={participant.athlete.country}
                    class="shrink-0 [--flag-height:1.25em]"
                  />
                  <a
                    href={resolve(`/athletes/${participant.athlete.slug}`)}
                    class="truncate underline hover:text-secondary"
                  >
                    {formatAthleteName(participant.athlete)}
                  </a>
                </span>
                <span
                  class="shrink-0 text-[0.65rem] font-medium tracking-wide uppercase {STATUS_FLAG}"
                  title={athleteStatusTitle(participant.status, participant.status_reason)}
                >
                  {ATHLETE_STATUS_LABEL[participant.status]}
                </span>
              </span>
            </td>
            <td class="{TABLE_CELL} {CELL.nothing}">{NO_RESULT}</td>
            {#if risColumn}
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
            {#if classed}
              <td class="{TABLE_CELL} {CELL.data}">{category.category.weight_class}</td>
            {/if}
          </tr>
        {/each}
      {/snippet}
    </Table>
  {/if}
</div>
