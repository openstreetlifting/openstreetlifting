<script lang="ts">
  import CompetitionBodyweight, {
    MIN_POINTS,
    type BodyweightPoint,
  } from '#lib/components/competition-bodyweight.svelte';
  import CompetitionAttempts, { MIN_ATTEMPTS } from '#lib/components/competition-attempts.svelte';
  import CompetitionResultsTable, {
    type ResultColumns,
  } from '#lib/components/competition-results-table.svelte';
  import SegmentedControl from '#lib/components/ui/segmented-control.svelte';
  import Button from '#lib/components/ui/button.svelte';
  import { LAYOUT } from '#lib/constants/layout.js';
  import { LIFTS } from '#lib/constants/lift.js';
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
    Text,
    DetailList,
    Detail,
  } from '#lib/components/ui/index.js';
  import { resolve } from '$app/paths';
  import type { ResolvedPathname } from '$app/types';
  import { page } from '$app/state';
  import { afterNavigate } from '$app/navigation';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import {
    formatLongDate,
    formatLocation,
    countryName,
    slugify,
    federationPath,
    formatAthleteName,
  } from '#lib/utils/index.js';
  import { EDGE_TO_EDGE } from '#lib/constants/table.js';
  import { TABLE_PAGE_SIZE } from '#lib/constants/pagination.js';
  import { GitHubIcon } from '#lib/components/icons/index.js';
  import { risProvenance } from '#lib/constants/ranking.js';
  import { ListingSearch } from '#lib/state/listing-search.svelte.js';
  import type { RankBy, RankingMetric } from '#lib/types/enums.js';
  import {
    attemptSuccess,
    competitionDivisions,
    contestSummary,
    matchesAthlete,
    mixedResults,
    resultTables,
    type ContestGroup,
    type DivisionSummary,
  } from '#lib/utils/competition-results.js';
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
  // we hold, so an MPDS meet with no published bodyweight keeps an empty column.
  const ris = $derived(risProvenance(competition));
  const risSortable = $derived(ris === 'recomputed' || ris === 'reported');

  const classed = $derived(
    competition.categories.some(({ category }) => category.weight_class !== '')
  );

  // As of 16/11/2026, only DCSV and MACHBAR have authorized me to show the age of
  // competitors for all the competitions they organize.
  const AGE_FEDERATIONS = new Set(['DCSV', 'MACHBAR']);

  const event = $derived(competition.event_code ?? null);
  const contested = $derived(
    event ? LIFTS.filter((lift) => event.includes(lift.code)) : [...LIFTS]
  );

  const params = $derived(page.url.searchParams);
  const focused = $derived(params.get('athlete'));

  const divisions = $derived(competitionDivisions(competition.categories));
  const activeDivision = $derived.by(() => {
    if (divisions.length < 2) return null;
    const requested =
      params.get('division') ??
      competition.categories.find(({ participants }) =>
        participants.some((participant) => participant.athlete.slug === focused)
      )?.category.division;
    return (divisions.find((division) => division.name === requested) ?? divisions[0]).name;
  });
  const board = $derived(
    activeDivision
      ? competition.categories.filter(({ category }) => category.division === activeDivision)
      : competition.categories
  );

  const classSorts = $derived([
    { value: 'total' as const, label: 'Total' as const },
    ...contested.map((lift) => ({ value: lift.key, label: lift.label })),
  ]);
  const defaultSort = $derived.by((): RankingMetric => {
    const rankBy =
      divisions.find((division) => division.name === activeDivision)?.rankBy ??
      competition.rank_by ??
      board[0]?.category.rank_by;
    return rankBy === 'ris' && risSortable ? 'ris' : 'total';
  });
  const sort = $derived.by((): RankingMetric => {
    const requested = params.get('movement');
    if (requested === 'ris') return risSortable ? 'ris' : defaultSort;
    return classSorts.find(({ value }) => value === requested)?.value ?? defaultSort;
  });

  const search = new ListingSearch(page.url);
  const query = $derived(search.value.trim());
  const matches = (participant: Parameters<typeof matchesAthlete>[0]) =>
    !query || matchesAthlete(participant, query);

  type ResultsState = { division: string | null; sort: RankingMetric; q: string; page: number };

  function resultsHref(slug: string, next: Partial<ResultsState> = {}) {
    const state: ResultsState = {
      division: activeDivision,
      sort,
      q: search.applied.trim(),
      page: 1,
      ...next,
    };
    const query = new SvelteURLSearchParams();
    if (state.division && state.division !== divisions[0]?.name) {
      query.set('division', state.division);
    }
    if (state.sort !== defaultSort) query.set('movement', state.sort);
    if (state.q) query.set('q', state.q);
    if (state.page > 1) query.set('page', String(state.page));
    const suffix = query.size ? `?${query}` : '';
    return i18n.href(resolve(`competitions/${slug}${suffix}`)) as ResolvedPathname;
  }

  const go = (next: Partial<ResultsState>) => search.navigate(resultsHref(competition.slug, next));

  afterNavigate(({ type, shallow }) => {
    if (shallow) return;
    search.sync(page.url, type);
    if (focused) {
      requestAnimationFrame(() => {
        document.querySelector('[data-focused]')?.scrollIntoView({ block: 'center' });
      });
    }
  });

  const tables = $derived(
    sort === 'ris'
      ? []
      : resultTables(board, sort)
          .map((table) => ({
            ...table,
            rows: table.rows.filter((row) => matches(row.participant)),
          }))
          .filter((table) => table.rows.length > 0)
  );

  const mixed = $derived(
    sort === 'ris' ? mixedResults(board, 'ris').filter((row) => matches(row.participant)) : []
  );
  const mixedPages = $derived(Math.max(1, Math.ceil(mixed.length / TABLE_PAGE_SIZE)));
  const mixedPage = $derived.by(() => {
    const requested = Number(params.get('page'));
    if (Number.isInteger(requested) && requested > 0) return Math.min(requested, mixedPages);
    const index = mixed.findIndex((row) => row.participant.athlete.slug === focused);
    return index < 0 ? 1 : Math.floor(index / TABLE_PAGE_SIZE) + 1;
  });

  const columns = $derived<ResultColumns>({
    event,
    ris: ris !== 'not-contested',
    risPublished: ris === 'reported',
    lifts: contested,
    withAttempts: new Set(
      competition.categories.flatMap((category) =>
        category.participants.flatMap((participant) =>
          participant.lifts.filter((lift) => lift.attempts.length).map((lift) => lift.movement_name)
        )
      )
    ),
    field: sort === 'ris',
    classed,
    age: AGE_FEDERATIONS.has(competition.federation.name),
    weight: competition.categories.some(({ participants }) =>
      participants.some(({ bodyweight }) => bodyweight != null)
    ),
  });

  const narrowed = $derived(Boolean(query || sort !== defaultSort || focused));

  const bodyweightPoints = $derived(
    board.flatMap((category) =>
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

  const bodyweightCharted = $derived(data.formula != null && bodyweightPoints.length >= MIN_POINTS);
  const attemptRates = $derived(
    attemptSuccess(
      board,
      competition.movements.map(({ movement_name }) => movement_name)
    )
  );
  const attemptsCharted = $derived(
    attemptRates.some(({ rates }) => rates[0].taken >= MIN_ATTEMPTS)
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
  const place = $derived([competition.venue, location].filter(Boolean).join(', '));
  // A class can appear in several divisions but only needs to be listed once per sex.
  const summary = $derived(contestSummary(competition.categories));
  const divisioned = $derived(summary.length > 1);
  const activeSummary = $derived(
    divisioned ? summary.find((division) => division.name === activeDivision) : undefined
  );
  const rule = (rankBy: RankBy) => (rankBy === 'ris' ? 'RIS' : i18n.t('Total'));
  const ruleInline = (rankBy: RankBy) => (rankBy === 'ris' ? 'RIS' : i18n.t('total'));
  const who = (gender: ContestGroup['gender']) =>
    i18n.t(({ M: 'Men', F: 'Women', MX: 'Men and women' } as const)[gender]);
  const risLabel = $derived(
    ris === 'reported' || !page.data.risEdition ? 'RIS' : `RIS ${page.data.risEdition}`
  );

  const seo = $derived(listingSeo(page.url));

  const seoName = $derived(competitionSeoName(competition));
  const federationHref = $derived(federationPath(competition.federation.name));
  const seoDescription = $derived(
    published
      ? `Streetlifting results from ${seoName}.`
      : `Streetlifting competition details for ${seoName}.`
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
      { name: 'Competitions', path: '/competitions' },
      { name: competition.federation.name, path: federationHref },
      { name: competition.name, path: `/competitions/${competition.slug}` },
    ]),
  ]}
/>

{#snippet rules(division: DivisionSummary)}
  {#each division.groups as group (group.gender)}
    {@const rankedBy = group.rankBy ? ruleInline(group.rankBy) : '—'}
    <Detail label={who(group.gender)} class="flex flex-wrap gap-x-3 gap-y-1">
      {#if group.classes.length > 0}
        <span>{i18n.t('Ranked by {rule}, by weight class:', { rule: rankedBy })}</span>
        <span
          class="contents"
          role="list"
          aria-label={i18n.t("{category}'s weight classes", { category: who(group.gender) })}
        >
          {#each group.classes as weightClass (weightClass)}
            <span role="listitem" class="whitespace-nowrap tabular-nums">
              {weights.category(weightClass, i18n.locale)}
            </span>
          {/each}
          {#if group.allWeights}
            <span role="listitem" class="whitespace-nowrap">{i18n.t('open weight')}</span>
          {/if}
        </span>
      {:else if group.rankBy === 'total'}
        {i18n.t('Ranked by {rule}, open weight', { rule: rankedBy })}
      {:else if group.rankBy}
        {i18n.t('Ranked by {rule}', { rule: rankedBy })}
      {:else}
        {rankedBy}
      {/if}
    </Detail>
  {/each}
{/snippet}

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
      <DetailList class="mt-4">
        <Detail label={i18n.t('Edition')} labelId="edition-label" labelClass="self-center">
          <nav aria-labelledby="edition-label">
            <SegmentedControl
              items={editions}
              key={(edition) => edition.slug}
              selected={(edition) => edition.slug === competition.slug}
              href={(edition) => resultsHref(edition.slug)}
              attributes={(edition) => ({
                'aria-label': `${edition.label} · ${edition.name}`,
                title: `${edition.name} · ${formatLongDate(edition.start_date, i18n.locale)}`,
              })}
            >
              {#snippet children(edition)}
                <time datetime={edition.start_date}>{edition.label}</time>
              {/snippet}
            </SegmentedControl>
          </nav>
        </Detail>
      </DetailList>
    {/if}

    <div
      class="mt-section grid gap-y-section sm:grid-cols-[repeat(2,minmax(0,36rem))] sm:gap-x-chapter"
    >
      <DetailList>
        {#if competition.start_date}
          <Detail label={i18n.t('Date')}>
            <time datetime={competition.start_date}>
              {competitionDate.format(new Date(competition.start_date))}
            </time>
            {#if competition.end_date && competition.end_date !== competition.start_date}
              <span class="text-muted">–</span>
              <time datetime={competition.end_date}>
                {competitionDate.format(new Date(competition.end_date))}
              </time>
            {/if}
          </Detail>
        {/if}
        {#if place || competition.country}
          <Detail label={i18n.t('Location')} class="flex items-start gap-2">
            {#if competition.country}
              <Flag
                countryCode={competition.country}
                link
                class="mt-[0.3em] shrink-0 [--flag-height:0.9em]"
              />
            {/if}
            <span class="min-w-0">
              {place || (competition.country ? countryName(competition.country, i18n.locale) : '')}
            </span>
          </Detail>
        {/if}
        <Detail label={i18n.t('Federation')} class="flex items-center gap-2">
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
          >
            {federationLabel}
          </a>
        </Detail>
      </DetailList>
      <DetailList>
        <Detail label={i18n.t('Format')}>
          {formatLabel ||
            (competition.status === 'upcoming' ? i18n.t('Format not announced') : '—')}
        </Detail>
        {#if !published}
          {#if competition.rank_by}
            <Detail label={i18n.t('Ranked by')}>{rule(competition.rank_by)}</Detail>
          {/if}
        {:else if divisioned}
          <Detail label={i18n.t('Divisions')}>
            {divisions.map((division) => division.name).join(', ')}
          </Detail>
        {:else}
          {@render rules(summary[0])}
        {/if}
      </DetailList>
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
      <Text as="h2" variant="heading" id="results-heading" class="text-ink">
        {i18n.t('Results')}
      </Text>
      {@render editLink()}
    </div>
    {#if divisions.length > 1}
      <nav class="mb-shell-dense" aria-label={i18n.t('Divisions')}>
        <SegmentedControl
          items={divisions}
          key={(division) => division.name}
          selected={(division) => division.name === activeDivision}
          href={(division) => resultsHref(competition.slug, { division: division.name })}
          attributes={(division) => ({
            title: i18n.t('{division}, ranked by {metric}', {
              division: division.name,
              metric: rule(division.rankBy),
            }),
          })}
        >
          {#snippet children(division)}
            {division.name}
          {/snippet}
        </SegmentedControl>
      </nav>
      {#if activeSummary}
        <DetailList class="mb-shell-dense">{@render rules(activeSummary)}</DetailList>
      {/if}
    {/if}
    <section
      class="{EDGE_TO_EDGE} min-w-0 rounded-panel border border-stroke-strong"
      aria-labelledby="results-heading"
    >
      <FilterBar
        embedded
        bind:search={search.value}
        placeholder={i18n.t('Search an athlete')}
        onSearch={() => go({ q: query })}
        activeCount={sort === defaultSort ? 0 : 1}
        onClear={() => {
          search.value = '';
          go({ q: '', sort: defaultSort });
        }}
        clearable={narrowed}
      >
        <SelectField
          id="sort-by"
          label={i18n.t('Sort by')}
          value={sort}
          onchange={(event) => go({ sort: event.currentTarget.value as RankingMetric })}
        >
          <optgroup label={i18n.t('By class')}>
            {#each classSorts as option (option.value)}
              <option value={option.value}>
                {i18n.t('By {sort}', { sort: i18n.t(option.label) })}
              </option>
            {/each}
          </optgroup>
          {#if risSortable}
            <optgroup label={i18n.t('By index')}>
              <option value="ris">{i18n.t('By {sort}', { sort: risLabel })}</option>
            </optgroup>
          {/if}
        </SelectField>
      </FilterBar>
      {#if sort === 'ris' ? mixed.length === 0 : tables.length === 0}
        <RankingsEmpty
          embedded
          canReset={narrowed}
          resetHref={resultsHref(competition.slug, { q: '', sort: defaultSort })}
        />
      {:else if sort === 'ris'}
        <CompetitionResultsTable
          rows={mixed.slice((mixedPage - 1) * TABLE_PAGE_SIZE, mixedPage * TABLE_PAGE_SIZE)}
          {columns}
          sorted="ris"
          {focused}
          label={i18n.t('Results')}
          pagination={{
            page: mixedPage,
            total_items: mixed.length,
            total_pages: mixedPages,
            pageHref: (target) => resultsHref(competition.slug, { page: target }),
            replaceState: true,
          }}
        />
      {:else}
        {#each tables as table (table.category.name)}
          {@const heading = [
            who(table.category.gender),
            weights.category(table.category.weight_class, i18n.locale),
          ]
            .filter(Boolean)
            .join(' ')}
          <div class="border-b border-stroke last:border-b-0">
            <Text as="h3" variant="panelTitle" class="bg-surface-subtle px-3 py-1.5">
              {heading}
            </Text>
            <CompetitionResultsTable
              rows={table.rows}
              {columns}
              sorted={sort}
              {focused}
              label={heading}
            />
          </div>
        {/each}
      {/if}
    </section>
    {#if bodyweightCharted || attemptsCharted}
      <div
        class="mt-section grid gap-tile {bodyweightCharted && attemptsCharted
          ? 'lg:grid-cols-2'
          : ''}"
      >
        {#if bodyweightCharted && data.formula}
          <CompetitionBodyweight points={bodyweightPoints} formula={data.formula} />
        {/if}
        {#if attemptsCharted}
          <CompetitionAttempts rates={attemptRates} />
        {/if}
      </div>
    {/if}
  {/if}
</div>
