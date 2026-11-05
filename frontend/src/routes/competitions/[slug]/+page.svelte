<script lang="ts">
  import CompetitionBodyweight, {
    MIN_POINTS,
    type BodyweightPoint,
  } from '#lib/components/competition-bodyweight.svelte';
  import { CONTROL } from '#lib/constants/typography.js';
  import Button from '#lib/components/ui/button.svelte';
  import { LAYOUT } from '#lib/constants/layout.js';
  import Notice from '#lib/components/ui/notice.svelte';
  import SelectField from '#lib/components/ui/select-field.svelte';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
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
    FROZEN_RANK,
    FROZEN_ATHLETE,
    ATHLETE_COLUMN_FILL,
    ATHLETE_CONTENT,
    Text,
  } from '#lib/components/ui/index.js';
  import { resolve } from '$app/paths';
  import { rankingsHref } from '#lib/state/rankings-return.svelte.js';
  import { slowNavigation } from '#lib/state/slow-navigation.svelte.js';
  import { page, navigating } from '$app/state';
  import { afterNavigate } from '$app/navigation';
  import {
    formatLongDate,
    formatLocation,
    countryName,
    slugify,
    federationPath,
    formatAthleteName,
  } from '#lib/utils/index.js';
  import {
    ATTEMPT_ROW,
    BEST_ROW,
    CELL,
    STATUS_FLAG,
    SORTED_COLUMN,
    NO_VALUE,
    NO_RESULT,
    EDGE_TO_EDGE,
  } from '#lib/constants/table.js';
  import { GitHubIcon } from '#lib/components/icons/index.js';
  import {
    RANKING_SORTS,
    RANKING_SORTS_NO_RIS,
    RANKING_GENDERS,
    defaultRankingSort,
  } from '#lib/constants/ranking.js';
  import { RankingsTable } from '#lib/state/rankings-table.svelte.js';
  import type { RankingEntry } from '#lib/types/ranking.js';
  import type { Attempt, Participant, CategoryDetail } from '#lib/types/competition.js';
  import { GENDERS, asRankingMetric, type AthleteStatus } from '#lib/types/enums.js';
  import { hasRankingResult, weightClasses } from '#lib/utils/competition-results.js';
  import { ATHLETE_STATUS_LABEL, athleteStatusTitle } from '#lib/constants/athlete-status.js';
  import Seo from '#lib/components/seo.svelte';
  import {
    breadcrumbLd,
    competitionLd,
    competitionSeoName,
    competitionTitle,
    listingSeo,
  } from '#lib/seo/index.js';

  const i18n = useI18n();
  const weights = useWeightUnit();

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
    defaultSort: () => data.competition.scoring ?? defaultRankingSort(data.ris),
  });

  afterNavigate(({ type, shallow }) => {
    if (shallow) return;

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
  const genders = $derived(
    competition.categories.some(({ category }) => category.gender === 'MX')
      ? [...RANKING_GENDERS, { value: 'MX', label: 'Mixed' } as const]
      : RANKING_GENDERS
  );

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
  // arrive with the competition itself, keyed by entry.
  const participants = $derived(
    new Map(
      competition.categories
        .flatMap((category) => category.participants)
        .map((participant) => [participant.participant_id, participant])
    )
  );

  type LiftCell =
    /** The movement was not contested or its result is unknown. */
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
    if (!lift && best === null) return { kind: 'absent' };

    if (lift?.attempts.length) {
      return { kind: 'attempts', attempts: lift.attempts, best };
    }

    return best === null ? { kind: 'bombed' } : { kind: 'best', best };
  }

  const bodyweightPoints = $derived(
    competition.categories.flatMap((category) =>
      category.category.gender === 'MX'
        ? []
        : category.participants.flatMap((participant): BodyweightPoint[] => {
            const bodyweight = Number(participant.bodyweight);
            const total = Number(participant.total);
            return participant.status === 'competed' && bodyweight > 0 && total > 0
              ? [
                  {
                    id: participant.participant_id,
                    name: formatAthleteName(participant.athlete),
                    sex: category.category.gender === 'F' ? 'women' : 'men',
                    bodyweight,
                    total,
                  },
                ]
              : [];
          })
    )
  );

  const withAttempts = $derived(
    new Set(
      competition.categories.flatMap((category) =>
        category.participants.flatMap((participant) =>
          participant.lifts.filter((lift) => lift.attempts.length).map((lift) => lift.movement_name)
        )
      )
    )
  );

  function rankedCell(entry: RankingEntry, key: string, code: string, movement: string): LiftCell {
    const best = entry[key as 'muscleup' | 'pullup' | 'dips' | 'squat'];
    return liftCell(participants.get(entry.participant_id), best, code, movement);
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
    competition.movements.map(({ movement_name }) => i18n.movement(movement_name)).join(' · ')
  );

  const competitionDate = $derived(
    new Intl.DateTimeFormat(i18n.locale === 'en' ? 'en-GB' : i18n.locale, {
      day: 'numeric',
      month: 'short',
      year: 'numeric',
      timeZone: 'UTC',
    })
  );

  const editions = $derived.by(() => {
    const shortDate = new Intl.DateTimeFormat(i18n.locale === 'en' ? 'en-GB' : i18n.locale, {
      day: 'numeric',
      month: 'short',
      timeZone: 'UTC',
    });
    // The API orders editions chronologically, so matching years are adjacent.
    return competition.editions.map((edition, index, editions) => {
      const year = edition.start_date.slice(0, 4);
      const sharesYear =
        editions[index - 1]?.start_date.startsWith(year) ||
        editions[index + 1]?.start_date.startsWith(year);
      const label = sharesYear
        ? `${year} · ${shortDate.format(new Date(edition.start_date))}`
        : year;
      return { ...edition, label };
    });
  });

  const location = $derived(formatLocation(competition.city, competition.region));
  const risNote = $derived(
    data.ris === 'recomputed'
      ? i18n.t('In this competition, RIS is recomputed with the current formula.')
      : data.ris === 'reported'
        ? i18n.t('In this competition, RIS scores are shown as published.')
        : data.ris === 'not-published'
          ? competition.movements.length < 4
            ? i18n.t('Published RIS scores are missing from the source.')
            : i18n.t(
                'RIS scores are unavailable because the required bodyweight or total is missing.'
              )
          : undefined
  );
  const place = $derived([competition.venue, location].filter(Boolean).join(', '));
  const DETAILS =
    'grid w-max max-w-full grid-cols-[6rem_minmax(0,1fr)] content-start gap-x-3 gap-y-1.5 text-body sm:grid-cols-[7rem_minmax(0,1fr)]';
  const EDITION = `${CONTROL} inline-flex h-full min-w-control shrink-0 items-center justify-center px-3 tabular-nums transition-colors focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2 focus-visible:ring-offset-canvas focus-visible:outline-none`;

  // Use the complete results, not the filtered ranking facets. A class can
  // appear in several divisions but only needs to be listed once per sex.
  const representedClasses = $derived(
    GENDERS.map((gender) => ({
      gender,
      label: i18n.t(({ M: 'Men', F: 'Women', MX: 'Mixed' } as const)[gender]),
      classes: weightClasses(competition.categories, gender),
    })).filter(({ classes }) => classes.length > 0)
  );

  // Missing scores have no rank, but their entries still belong in the results.
  const NOT_PLACED_ORDER: AthleteStatus[] = ['competed', 'disqualified', 'no_show'];

  const selectedMetric = $derived(
    asRankingMetric(table.movementFilter) ?? defaultRankingSort(data.ris)
  );

  const notPlaced = $derived(
    competition.categories
      .flatMap((category: CategoryDetail) =>
        category.participants
          .filter((participant) => !hasRankingResult(participant, selectedMetric))
          .map((participant) => ({ category, participant }))
      ) // The ranked half is filtered by the server, so these have to answer the
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
    formatLocation(
      competition.city,
      competition.country && countryName(competition.country, i18n.locale)
    )
  );
  const seoMovements = $derived(
    competition.movements
      .map((movement) => i18n.movement(movement.movement_name).toLowerCase())
      .join(', ')
  );

  const seoDescription = $derived(
    [
      published
        ? competition.status === 'live'
          ? i18n.t('Partial results and standings from {competition}', {
              competition: competition.name,
            })
          : i18n.t('Full results and standings from {competition}', {
              competition: competition.name,
            })
        : i18n.t('{competition}, a {federation} Streetlifting competition', {
            competition: competition.name,
            federation: federationLabel,
          }),
      competition.start_date ? `, ${formatLongDate(competition.start_date, i18n.locale)}` : '',
      seoWhere ? `, ${seoWhere}` : '',
      published
        ? '. ' +
          i18n.t('{count} lifters ranked on {movements}.', {
            count: i18n.number(lifterCount, 0),
            movements: seoMovements,
          })
        : '. ' + i18n.t('Entered in the OpenStreetlifting calendar.'),
    ].join('')
  );
</script>

<Seo
  title={competitionTitle(seoName, published, i18n.locale)}
  description={seoDescription}
  canonical={seo.canonical}
  noindex={seo.noindex}
  jsonLd={[
    competitionLd(competition, seoDescription, i18n.locale),
    breadcrumbLd(
      [
        { name: i18n.t('Rankings'), path: '/' },
        { name: i18n.t('Competitions'), path: '/competitions' },
        { name: competition.federation.name, path: federationHref },
        { name: competition.name, path: `/competitions/${competition.slug}` },
      ],
      i18n.locale
    ),
  ]}
/>

{#snippet editLink()}
  {#if editPath}
    <Button
      variant="outline"
      size="control"
      href={`https://github.com/openstreetlifting/openstreetlifting/edit/main/backend/data/competitions/${editPath}`}
      target="_blank"
      rel="noopener noreferrer"
      class="shrink-0 gap-2 text-secondary hover:text-ink"
    >
      <GitHubIcon class="h-4 w-4" />
      {i18n.t('Edit competition on GitHub')}
    </Button>
  {/if}
{/snippet}

<div class={LAYOUT.page}>
  <Breadcrumb
    items={[
      { label: i18n.t('Rankings'), href: rankingsHref() },
      { label: i18n.t('Competitions'), href: '/competitions' },
      { label: competition.federation.name, href: federationHref },
      { label: competition.name },
    ]}
  />

  <header class="mb-section border-b border-stroke pb-4">
    <div class="flex flex-wrap items-start justify-between gap-3">
      <Text as="h1" variant="title" class="min-w-0 break-words text-ink">{competition.name}</Text>
      {#if !published}{@render editLink()}{/if}
    </div>

    {#if editions.length > 1}
      <dl class="{DETAILS} mt-4">
        <dt id="edition-label" class="self-center text-muted">{i18n.t('Edition')}</dt>
        <dd class="min-w-0">
          <nav aria-labelledby="edition-label">
            <ul
              class="inline-flex h-control max-w-full overflow-x-auto rounded-control border border-stroke"
            >
              {#each editions as edition (edition.slug)}
                <li class="h-full">
                  {#if edition.slug === competition.slug}
                    <span aria-current="page" class="{EDITION} bg-surface-selected text-ink">
                      <time datetime={edition.start_date}>{edition.label}</time>
                    </span>
                  {:else}
                    <a
                      href={i18n.href(resolve('/competitions/[slug]', { slug: edition.slug }))}
                      aria-label={`${edition.label} · ${edition.name}`}
                      title={`${edition.name} · ${formatLongDate(edition.start_date, i18n.locale)}`}
                      class="{EDITION} text-secondary hover:bg-surface hover:text-ink"
                    >
                      <time datetime={edition.start_date}>{edition.label}</time>
                    </a>
                  {/if}
                </li>
              {/each}
            </ul>
          </nav>
        </dd>
      </dl>
    {/if}

    <div class="mt-section flex flex-wrap gap-x-chapter gap-y-section">
      <dl class={DETAILS}>
        {#if competition.start_date}
          <dt class="text-muted">{i18n.t('Date')}</dt>
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
        {#if place || competition.country}
          <dt class="text-muted">{i18n.t('Location')}</dt>
          <dd class="flex min-w-0 items-start gap-2 text-ink">
            {#if competition.country}
              <Flag
                countryCode={competition.country}
                link
                class="mt-[0.3em] shrink-0 [--flag-height:0.9em]"
              />
            {/if}
            <span class="min-w-0"
              >{place ||
                (competition.country ? countryName(competition.country, i18n.locale) : '')}</span
            >
          </dd>
        {/if}
        <dt class="text-muted">{i18n.t('Federation')}</dt>
        <dd class="flex min-w-0 items-center gap-2 text-ink">
          {#if competition.federation.country}
            <Flag
              countryCode={competition.federation.country}
              class="shrink-0 [--flag-height:0.9em]"
            />
          {/if}
          <a
            href={i18n.href(
              resolve('/federations/[slug]', { slug: slugify(competition.federation.name) })
            )}
            title={competition.federation.name}
            class="min-w-0 break-words underline decoration-stroke-strong underline-offset-2 hover:text-secondary"
            >{federationLabel}</a
          >
        </dd>
      </dl>
      <dl class={DETAILS}>
        <dt class="text-muted">{i18n.t('Format')}</dt>
        <dd class="text-ink">
          {formatLabel ||
            (competition.status === 'upcoming' ? i18n.t('Format not announced') : '—')}
        </dd>
        {#if competition.scoring}
          <dt class="text-muted">{i18n.t('Scoring')}</dt>
          <dd class="text-ink">{competition.scoring === 'ris' ? 'RIS' : i18n.t('Total')}</dd>
        {/if}
        {#if published}
          <dt class="text-muted">{i18n.t('Weight classes')}</dt>
          <dd class="grid min-w-0 gap-y-1.5 text-ink">
            {#if classed}
              {#each representedClasses as group (group.gender)}
                <p class="flex flex-col gap-x-2 sm:flex-row">
                  <span class="shrink-0 text-muted sm:w-14">{group.label}</span>
                  <span
                    class="flex min-w-0 flex-wrap gap-x-3 gap-y-1"
                    role="list"
                    aria-label={i18n.t("{category}'s weight classes", { category: group.label })}
                  >
                    {#each group.classes as weightClass (weightClass)}
                      <span role="listitem" class="whitespace-nowrap tabular-nums">
                        {weights.category(weightClass, i18n.locale) || i18n.t('No weight classes')}
                      </span>
                    {/each}
                  </span>
                </p>
              {/each}
            {:else}
              {i18n.t('No weight classes')}
            {/if}
          </dd>
        {/if}
      </dl>
    </div>
  </header>

  {#if competition.status === 'live'}
    <Notice title={i18n.t('In Progress')} live />
  {/if}

  {#if !published}
    <div class="border-l-2 border-stroke-strong py-1 pl-4">
      {#if competition.status === 'live'}
        <p class="text-subtitle text-ink">{i18n.t('Results are not available yet.')}</p>
        <p class="mt-2 max-w-2xl text-body leading-relaxed text-secondary">
          {i18n.t('Results will appear here once they are published.')}
        </p>
      {:else}
        <p class="text-subtitle text-ink">{i18n.t('This competition has not been lifted yet.')}</p>
        <p class="mt-2 max-w-2xl text-body leading-relaxed text-secondary">
          {i18n.t(
            '{competition} is scheduled for {date}. Results will appear here once published.',
            {
              competition: competition.name,
              date: formatLongDate(competition.start_date, i18n.locale),
            }
          )}
        </p>
      {/if}
    </div>
  {:else}
    <div class="mb-shell-dense flex flex-wrap items-center justify-between gap-3">
      <Text as="h2" variant="heading" id="results-heading" class="text-ink"
        >{i18n.t('Results')}</Text
      >
      {@render editLink()}
    </div>
    <section
      class="{EDGE_TO_EDGE} min-w-0 rounded-panel border border-stroke-strong"
      aria-labelledby="results-heading"
    >
      <FilterBar
        embedded
        bind:search={table.searchFilter}
        placeholder={i18n.t('Search an athlete')}
        onSearch={() => table.handleFilterChange()}
        activeCount={activeFilters}
        onClear={() => table.clearFilters()}
        clearable={table.narrowed}
      >
        <SelectField
          label={i18n.t('Country')}
          bind:value={table.countryFilter}
          onchange={() => table.handleFilterChange()}
        >
          <option value={null}>{i18n.t('All Countries')}</option>
          {#each data.countries as countryOption (countryOption)}
            <option value={countryOption}>{countryName(countryOption, i18n.locale)}</option>
          {/each}
        </SelectField>
        <SelectField
          label={i18n.t('Sex')}
          bind:value={table.genderFilter}
          onchange={() => {
            table.categoryFilter = null;
            table.handleFilterChange();
          }}
        >
          {#each genders as gender (gender.label)}
            <option value={gender.value}>{i18n.t(gender.label)}</option>
          {/each}
        </SelectField>
        {#if classed}
          <SelectField
            label={i18n.t('Class')}
            bind:value={table.categoryFilter}
            onchange={() => table.handleFilterChange()}
          >
            <option value={null}>{i18n.t('All Classes')}</option>
            {#each data.classes as classOption (classOption)}
              <option value={classOption}>{weights.category(classOption, i18n.locale)}</option>
            {/each}
          </SelectField>
        {/if}

        <SelectField
          id="sort-by"
          label={i18n.t('Sort by')}
          bind:value={table.movementFilter}
          onchange={() => table.handleFilterChange()}
        >
          {#each sorts as sort (sort.value)}
            <option value={sort.value}
              >{i18n.t('Sort by {sort}', { sort: i18n.t(sort.label) })}</option
            >
          {/each}
        </SelectField>
      </FilterBar>
      {#if rankings.length === 0 && unplacedOnPage.length === 0 && !busy}
        <RankingsEmpty
          embedded
          canReset={table.narrowed || pagination.page > 1}
          resetHref={i18n.href(resolve(`competitions/${competition.slug}`))}
        />
      {:else}
        <Table
          embedded
          rows={rankings}
          frozenColumns="athlete"
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
            <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} text-secondary"
              >{i18n.t('Rank')}</th
            >
            <th
              class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_ATHLETE} {ATHLETE_COLUMN_FILL} text-secondary"
              >{i18n.t('Athlete')}</th
            >
            <th class="{TABLE_HEAD_CELL} text-secondary {sorted('total')}"
              >{i18n.t('Total')} ({weights.unit})</th
            >
            {#if risColumn}
              <th class="{TABLE_HEAD_CELL} text-secondary {sorted('ris')}">
                <RisHeader note={risNote} />
              </th>
            {/if}
            {#each contested as lift (lift.key)}
              {@const attempts = withAttempts.has(lift.movement)}
              <th class="{TABLE_HEAD_CELL} text-secondary {sorted(lift.key)}">
                {i18n.t(lift.label)}
                {#if attempts}
                  <span class="{ATTEMPT_ROW} mt-0.5 text-micro font-normal text-muted">
                    <span class="text-right">1</span>
                    <span class="text-right">2</span>
                    <span class="text-right">3</span>
                    <span class="text-right">{i18n.t('Best')}</span>
                  </span>
                {/if}
              </th>
            {/each}
            <th class="{TABLE_HEAD_CELL} text-secondary">
              {i18n.t('Sex')}
            </th>
            {#if classed}
              <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Class')}</th>
            {/if}
          {/snippet}

          {#snippet body(rows)}
            {#each rows as entry (entry.participant_id)}
              {@const participant = participants.get(entry.participant_id)}
              <tr
                class="transition-colors"
                data-focused={entry.athlete.slug === table.focusedAthlete ? '' : undefined}
              >
                <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {CELL.identity}">
                  {entry.rank}
                </td>
                <td
                  class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_ATHLETE} {ATHLETE_COLUMN_FILL} {CELL.identity}"
                >
                  <span class="flex items-center gap-1.5 {ATHLETE_CONTENT}">
                    <span class="flex min-w-0 items-center gap-2">
                      <Flag
                        link
                        countryCode={entry.athlete.country}
                        class="shrink-0 [--flag-height:1.25em]"
                      />
                      <a
                        href={i18n.href(resolve(`athletes/${entry.athlete.slug}`))}
                        class="truncate underline hover:text-secondary"
                      >
                        {formatAthleteName(entry.athlete)}
                      </a>
                    </span>
                    {#if participant && participant.status !== 'competed'}
                      <span
                        class="shrink-0 text-micro font-medium tracking-wide uppercase {STATUS_FLAG}"
                        title={athleteStatusTitle(
                          participant.status,
                          participant.status_reason,
                          i18n.locale
                        )}
                      >
                        {ATHLETE_STATUS_LABEL[participant.status]}
                      </span>
                    {/if}
                  </span>
                </td>
                <td class="{TABLE_CELL} {CELL.counted}"
                  >{weights.format(entry.total, i18n.locale)}</td
                >
                {#if risColumn}
                  <td class="{TABLE_CELL} {CELL.counted}">
                    <RisScore value={entry.ris} source={entry.ris_source} />
                  </td>
                {/if}
                {#each contested as lift (lift.key)}
                  {@const cell = rankedCell(entry, lift.key, lift.code, lift.movement)}
                  <td class="{TABLE_CELL} whitespace-nowrap">
                    <span class={withAttempts.has(lift.movement) ? ATTEMPT_ROW : BEST_ROW}>
                      {#if cell.kind === 'attempts'}
                        {#each [1, 2, 3] as slot (slot)}
                          {@const attempt = cell.attempts.find((a) => a.attempt_number === slot)}
                          <span
                            class="text-right {attempt && !attempt.is_successful
                              ? CELL.discounted
                              : CELL.data}"
                            title={attempt
                              ? i18n.t('Attempt {slot}: {weight} {unit}, {result}', {
                                  unit: weights.unit,
                                  slot,
                                  weight: weights.format(attempt.weight, i18n.locale),
                                  result: attempt.is_successful
                                    ? i18n.t('good lift')
                                    : i18n.t('no lift'),
                                })
                              : i18n.t('Attempt {slot} not recorded', { slot })}
                          >
                            {attempt ? weights.format(attempt.weight, i18n.locale) : ''}
                          </span>
                        {/each}
                      {:else if withAttempts.has(lift.movement)}
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
                            title={i18n.t('No successful {movement}', {
                              movement: i18n.t(lift.label).toLowerCase(),
                            })}>{NO_RESULT}</span
                          >
                        {:else}
                          {weights.format(cell.best, i18n.locale)}
                        {/if}
                      </span>
                    </span>
                  </td>
                {/each}
                <td class="{TABLE_CELL} {CELL.data}">{entry.athlete.gender}</td>
                {#if classed}
                  <td class="{TABLE_CELL} {CELL.data}"
                    >{weights.category(entry.category, i18n.locale)}</td
                  >
                {/if}
              </tr>
            {/each}

            {#each unplacedOnPage as { category, participant } (participant.athlete.athlete_id)}
              <tr class="transition-colors">
                <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {CELL.absent}">{NO_VALUE}</td>

                <td
                  class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_ATHLETE} {ATHLETE_COLUMN_FILL} {CELL.identity}"
                >
                  <span class="flex items-center gap-1.5 {ATHLETE_CONTENT}">
                    <span class="flex min-w-0 items-center gap-2">
                      <Flag
                        link
                        countryCode={participant.athlete.country}
                        class="shrink-0 [--flag-height:1.25em]"
                      />
                      <a
                        href={i18n.href(resolve(`athletes/${participant.athlete.slug}`))}
                        class="truncate underline hover:text-secondary"
                      >
                        {formatAthleteName(participant.athlete)}
                      </a>
                    </span>
                    {#if participant.status !== 'competed'}
                      <span
                        class="shrink-0 text-micro font-medium tracking-wide uppercase {STATUS_FLAG}"
                        title={athleteStatusTitle(
                          participant.status,
                          participant.status_reason,
                          i18n.locale
                        )}
                      >
                        {ATHLETE_STATUS_LABEL[participant.status]}
                      </span>
                    {/if}
                  </span>
                </td>
                <td class="{TABLE_CELL} {CELL.counted}">
                  {participant.status === 'competed'
                    ? weights.format(participant.total, i18n.locale)
                    : NO_RESULT}
                </td>
                {#if risColumn}
                  <td class="{TABLE_CELL} {CELL.counted}">
                    {#if participant.status === 'competed'}
                      <RisScore value={participant.ris_score} source={participant.ris_source} />
                    {:else}
                      {NO_RESULT}
                    {/if}
                  </td>
                {/if}
                {#each contested as lift (lift.key)}
                  {@const cell = participantCell(participant, lift.code, lift.movement)}
                  <td class="{TABLE_CELL} whitespace-nowrap">
                    <span class={withAttempts.has(lift.movement) ? ATTEMPT_ROW : BEST_ROW}>
                      {#if cell.kind === 'attempts'}
                        {#each [1, 2, 3] as slot (slot)}
                          {@const attempt = cell.attempts.find((a) => a.attempt_number === slot)}
                          <span
                            class="text-right {attempt && !attempt.is_successful
                              ? CELL.discounted
                              : CELL.data}"
                          >
                            {attempt ? weights.format(attempt.weight, i18n.locale) : ''}
                          </span>
                        {/each}
                      {:else if withAttempts.has(lift.movement)}
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
                          {weights.format(cell.best, i18n.locale)}
                        {/if}
                      </span>
                    </span>
                  </td>
                {/each}
                <td class="{TABLE_CELL} {CELL.data}">{category.category.gender}</td>
                {#if classed}
                  <td class="{TABLE_CELL} {CELL.data}"
                    >{weights.category(category.category.weight_class, i18n.locale)}</td
                  >
                {/if}
              </tr>
            {/each}
          {/snippet}
        </Table>
      {/if}
    </section>
    {#if data.formula && bodyweightPoints.length >= MIN_POINTS}
      <div class="mt-section">
        <CompetitionBodyweight points={bodyweightPoints} formula={data.formula} />
      </div>
    {/if}
  {/if}
</div>
