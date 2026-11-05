<script lang="ts">
  import { LAYOUT } from '#lib/constants/layout.js';
  import Select from '#lib/components/ui/select.svelte';
  import { useI18n } from '#lib/i18n/index.js';
  import { useWeightUnit } from '#lib/state/weight-unit.svelte.js';
  import { MAIN_RANKING_PAGE_SIZE } from '#lib/constants/pagination.js';
  import type { PageData } from './$types';
  import type { AthleteCompetitionSummary } from '#lib/types/athlete.js';
  import type { Attempt } from '#lib/types/competition.js';
  import type { Gender } from '#lib/types/enums.js';
  import { ATHLETE_STATUS_LABEL, athleteStatusTitle } from '#lib/constants/athlete-status.js';
  import {
    MetricCard,
    RankingCard,
    ResultState,
    Breadcrumb,
    Flag,
    Table,
    RisScore,
    RisHeader,
    TABLE_CELL,
    TABLE_HEAD_CELL,
    FROZEN_CELL,
    FROZEN_HEAD_CELL,
    FROZEN_RANK,
    Text,
  } from '#lib/components/ui/index.js';
  import { ChevronIcon, GlobeIcon, InstagramIcon } from '#lib/components/icons/index.js';
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { athleteFilters } from '#lib/utils/athlete-filters.js';
  import { rankingsHref } from '#lib/state/rankings-return.svelte.js';
  import { SvelteURLSearchParams } from 'svelte/reactivity';
  import { RANKING_SORTS } from '#lib/constants/ranking.js';
  import { formatDate, formatAthleteName, countryName, countryPath } from '#lib/utils/index.js';
  import Seo from '#lib/components/seo.svelte';
  import AthleteProgress from '#lib/components/athlete-progress.svelte';
  import AthleteBalance from '#lib/components/athlete-balance.svelte';
  import { absolute, athleteLd, athleteTitle, breadcrumbLd, risStanding } from '#lib/seo/index.js';
  import {
    ATTEMPT_ROW,
    CELL,
    FIGURE,
    STATUS_FLAG,
    NO_VALUE,
    NO_RESULT,
    TEXT_CELL,
  } from '#lib/constants/table.js';
  import { TEXT } from '#lib/constants/typography.js';

  const i18n = useI18n();
  const weights = useWeightUnit();

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

  const GENDER_LABEL = { M: 'Male', F: 'Female', MX: 'Mixed' } as const satisfies Record<
    Gender,
    string
  >;
  const genderLabel = $derived(i18n.t(GENDER_LABEL[athlete.gender]));
  const latestCategory = $derived(
    athlete.lift_balance?.category ??
      athlete.competitions.find((competition) => competition.status === 'competed')?.category_name
  );
  const latestWeightClass = $derived(
    latestCategory?.match(/([+-]\d+(?:\.\d+)?)\s*(?:kg)?$/i)?.[1] ?? null
  );

  const CARD_GRID = 'grid max-w-records grid-cols-2 gap-tile md:grid-cols-4';
  const RANKING_CARD_GRID = 'grid grid-cols-1 gap-tile sm:grid-cols-2';

  const filters = $derived(athleteFilters(page.url.searchParams));
  const selectedMetric = $derived(filters.ranking);

  function updateFilter(key: 'ranking' | 'performance', value: string) {
    const params = new SvelteURLSearchParams(page.url.search);
    const defaults = {
      ranking: 'ris',
      performance: 'total',
    };
    params.delete('event');
    if (value === defaults[key]) params.delete(key);
    else params.set(key, value);
    const query = params.toString();

    return goto(
      i18n.href(resolve(`athletes/${athlete.slug}${query ? `?${query}` : ''}${page.url.hash}`)),
      {
        replace: true,
        reset: false,
      }
    );
  }

  const selectedStanding = $derived(athlete.standing?.[selectedMetric] ?? null);
  const selectedCountry = $derived(selectedStanding?.country?.code ?? athlete.country);
  const selectedBoardFilters = $derived.by((): Record<string, string> =>
    selectedMetric === 'ris'
      ? {}
      : {
          movement: selectedMetric,
          gender: athlete.gender,
          category: selectedStanding?.class ?? '',
        }
  );

  function boardQuery(place: number, filters: Record<string, string> = {}): string {
    const params = new SvelteURLSearchParams(filters);
    const target = Math.ceil(place / MAIN_RANKING_PAGE_SIZE);
    if (target > 1) params.set('page', String(target));
    params.set('athlete', athlete.slug);
    return params.toString();
  }

  const athleteName = $derived(formatAthleteName(athlete));
  const seoBests = $derived(
    athlete.personal_records
      .map(
        (record) =>
          `${i18n.movement(record.movement_name).toLowerCase()} ${weights.format(record.max_weight, i18n.locale)} ${weights.unit}`
      )
      .join(', ')
  );

  const seoDescription = $derived(
    [
      athleteName,
      athlete.native_name ? ` (${athlete.native_name})` : '',
      athlete.country ? `, ${countryName(athlete.country, i18n.locale)}` : '',
      '. ',
      risStanding(athlete.standing, i18n.locale)
        ? `${risStanding(athlete.standing, i18n.locale)} `
        : '',
      i18n.t('Streetlifting results and personal records'),
      seoBests ? `: ${seoBests}` : '',
      '. ' +
        i18n.t('{count} {competitions} in the OpenStreetlifting archive.', {
          count: i18n.number(athlete.total_competitions, 0),
          competitions: i18n.count('competition', athlete.total_competitions),
        }),
    ].join('')
  );
</script>

<Seo
  title={athleteTitle(athleteName, athlete.country, i18n.locale)}
  description={seoDescription}
  canonical={absolute(`/athletes/${athlete.slug}`)}
  type="profile"
  jsonLd={[
    athleteLd(athlete, seoDescription, i18n.locale),
    breadcrumbLd(
      [
        { name: i18n.t('Rankings'), path: '/' },
        ...(athlete.country
          ? [
              {
                name: countryName(athlete.country, i18n.locale),
                path: countryPath(athlete.country),
              },
            ]
          : []),
        { name: athleteName, path: `/athletes/${athlete.slug}` },
      ],
      i18n.locale
    ),
  ]}
/>

<div class={LAYOUT.page}>
  <Breadcrumb
    items={[
      { label: i18n.t('Rankings'), href: rankingsHref() },
      ...(athlete.country
        ? [
            {
              label: countryName(athlete.country, i18n.locale),
              href: countryPath(athlete.country),
            },
          ]
        : []),
      { label: athleteName },
    ]}
  />

  <header class="mb-section border-b border-stroke pb-4">
    <div class="flex items-center gap-3">
      <h1
        class="flex min-w-0 items-start gap-3 font-sans text-title font-semibold tracking-tight text-balance text-ink sm:text-display"
      >
        {#if athlete.country}
          <Flag
            countryCode={athlete.country}
            link
            class="mt-[0.2em] shrink-0 [--flag-height:0.8em]"
          />
        {/if}
        <span class="min-w-0 break-words">{athleteName}</span>
      </h1>

      {#if athlete.instagram_handle}
        <a
          href={`https://www.instagram.com/${athlete.instagram_handle}`}
          target="_blank"
          rel="noopener noreferrer"
          class="shrink-0 text-ink transition-colors hover:text-secondary"
          aria-label={i18n.t('{athlete} on Instagram', { athlete: athleteName })}
          title="@{athlete.instagram_handle}"
        >
          <InstagramIcon class="h-5 w-5 sm:h-6 sm:w-6" />
        </a>
      {/if}
    </div>

    {#if !athlete.country}
      <p class="mt-1 text-body text-secondary">{i18n.t('Country not recorded')}</p>
    {/if}

    {#if athlete.native_name}
      <p class="mt-1 break-words text-lead text-secondary">{athlete.native_name}</p>
    {/if}

    <dl
      class="mt-4 grid grid-cols-[6rem_minmax(0,1fr)] gap-x-3 gap-y-1.5 text-body sm:grid-cols-[7rem_minmax(0,1fr)]"
    >
      <dt class="text-muted">{i18n.t('Sex')}</dt>
      <dd class="text-ink">{genderLabel}</dd>
      {#if latestWeightClass}
        <dt class="text-muted">{i18n.t('Weight class')}</dt>
        <dd class="text-ink">{weights.category(latestWeightClass + ' kg', i18n.locale)}</dd>
      {/if}
    </dl>
  </header>

  {#snippet standing(
    country: string | null,
    scope: string,
    place: number | undefined,
    field: number | undefined,
    query: string | undefined,
    missingLabel = i18n.t('Not ranked')
  )}
    <RankingCard
      heading={scope}
      {place}
      {field}
      {missingLabel}
      flagBackground={country !== null}
      href={query ? i18n.href(resolve(`?${query}`)) : undefined}
    >
      {#snippet background()}
        {#if country}<Flag countryCode={country} background />
        {:else if scope === i18n.t('Global')}<GlobeIcon class="text-secondary" />{/if}
      {/snippet}
      {#if selectedStanding?.class}
        {i18n.t('in category {category}', {
          category: weights.category(selectedStanding.class, i18n.locale),
        })}
      {/if}
    </RankingCard>
  {/snippet}
  {#if athlete.standing}
    <div class="mb-section max-w-summary">
      <div class="mb-3 flex items-center justify-between gap-3">
        <Text as="h2" variant="heading" class="text-ink">{i18n.t('Ranking')}</Text>
        <label class="flex items-center gap-2 text-meta text-muted">
          <span class="sr-only">{i18n.t('Metric')}</span>
          <Select
            name="ranking"
            value={selectedMetric}
            onchange={(event) => updateFilter('ranking', event.currentTarget.value)}
            class="px-3 py-1.5"
          >
            {#each RANKING_SORTS as metric (metric.value)}
              <option value={metric.value}>{i18n.t(metric.label)}</option>
            {/each}
          </Select>
        </label>
      </div>
      <div class={RANKING_CARD_GRID}>
        {@render standing(
          null,
          i18n.t('Global'),
          selectedStanding?.global.place,
          selectedStanding?.global.field,
          selectedStanding
            ? boardQuery(selectedStanding.global.place, selectedBoardFilters)
            : undefined
        )}
        {@render standing(
          selectedCountry,
          selectedCountry ? countryName(selectedCountry, i18n.locale) : i18n.t('National'),
          selectedStanding?.country?.place,
          selectedStanding?.country?.field,
          selectedStanding?.country && selectedCountry
            ? boardQuery(selectedStanding.country.place, {
                ...selectedBoardFilters,
                country: selectedCountry,
              })
            : undefined,
          selectedCountry ? i18n.t('Not ranked') : i18n.t('Country needed for national ranking')
        )}
      </div>
    </div>
  {/if}

  {#if athlete.personal_records.length > 0}
    <div class="mb-section">
      <Text as="h2" variant="heading" class="mb-3 text-ink">{i18n.t('Personal records')}</Text>
      <div class={CARD_GRID}>
        {#each athlete.personal_records as pr (pr.movement_name)}
          <MetricCard
            heading={i18n.movement(pr.movement_name)}
            meta={pr.date ? formatDate(pr.date, i18n.locale) : undefined}
            href={i18n.href(resolve(`competitions/${pr.competition_slug}`))}
          >
            <span class={TEXT.cardFigure}>{weights.format(pr.max_weight, i18n.locale)}</span>
            <span class="font-mono text-meta text-secondary">{weights.unit}</span>
            {#snippet footer()}
              <span
                class="mt-1 flex items-center gap-1 border-t border-stroke pt-3 text-secondary transition-colors group-hover:text-ink"
              >
                <span class="truncate" title={pr.competition_name}>{pr.competition_name}</span>
                <ChevronIcon class="size-3 shrink-0 -rotate-90" />
              </span>
              {#if pr.date}<span class="{TEXT.fieldLabel} mt-1 block lg:hidden"
                  >{formatDate(pr.date, i18n.locale)}</span
                >{/if}
            {/snippet}
          </MetricCard>
        {/each}
      </div>
    </div>
  {/if}

  <div>
    <Text as="h2" variant="heading" class="mb-3 text-ink">{i18n.t('Performance')}</Text>
    <div class="grid items-stretch gap-tile lg:auto-rows-fr lg:grid-cols-2">
      <AthleteProgress
        athleteSlug={athlete.slug}
        competitions={athlete.competitions}
        metric={filters.performance}
        onMetricChange={(value) => updateFilter('performance', value)}
      />

      <AthleteBalance balance={athlete.lift_balance} />
    </div>
  </div>

  <div class={LAYOUT.section}>
    <Text as="h2" variant="heading" class="mb-3 text-ink">{i18n.t('Competition history')}</Text>
    {#if athlete.competitions.length > 0}
      <Table
        rows={athlete.competitions}
        itemName="competition"
        pageParam="history_page"
        frozenColumns="rank"
      >
        {#snippet head()}
          <th class="{TABLE_HEAD_CELL} {FROZEN_HEAD_CELL} {FROZEN_RANK} text-secondary"
            >{i18n.t('Rank')}</th
          >
          <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Competition')}</th>
          <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Total')} ({weights.unit})</th>
          <th class="{TABLE_HEAD_CELL} text-secondary"><RisHeader /></th>
          {#each contested as lift (lift.key)}
            <th class="{TABLE_HEAD_CELL} text-secondary">
              {i18n.t(lift.label)}
              <span class="{ATTEMPT_ROW} mt-0.5 text-micro font-normal text-muted">
                <span class="text-right">1</span>
                <span class="text-right">2</span>
                <span class="text-right">3</span>
                <span class="text-right">{i18n.t('Best')}</span>
              </span>
            </th>
          {/each}
          <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Date')}</th>
          <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Class')}</th>
          {#if showsDivision}
            <th class="{TABLE_HEAD_CELL} text-secondary">{i18n.t('Division')}</th>
          {/if}
        {/snippet}

        {#snippet body(rows)}
          {#each rows as competition (`${competition.competition_id}:${competition.category_name}:${competition.division ?? ''}`)}
            <tr class="transition-colors {competition.status !== 'competed' ? 'opacity-50' : ''}">
              <td class="{TABLE_CELL} {FROZEN_CELL} {FROZEN_RANK} {CELL.identity}">
                {#if competition.status !== 'competed'}
                  <span
                    class={STATUS_FLAG}
                    title={athleteStatusTitle(competition.status, null, i18n.locale)}
                    >{ATHLETE_STATUS_LABEL[competition.status]}</span
                  >
                {:else}
                  {competition.rank || NO_VALUE}
                {/if}
              </td>
              <td class={TABLE_CELL}>
                <a
                  href={i18n.href(resolve(`competitions/${competition.competition_slug}`))}
                  class="{TEXT_CELL.competition} text-ink underline hover:text-secondary focus:ring-2 focus:ring-focus focus:outline-none"
                >
                  {competition.competition_name}
                </a>
              </td>

              <td class="{TABLE_CELL} {CELL.counted} {FIGURE}"
                >{weights.format(competition.total, i18n.locale)}</td
              >

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
              <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap">
                {formatDate(competition.competition_date, i18n.locale)}
              </td>
              <td class="{TABLE_CELL} {CELL.data} whitespace-nowrap">
                {weights.category(competition.category_name, i18n.locale)}
              </td>
              {#if showsDivision}
                <td class="{TABLE_CELL} {CELL.data}">{competition.division || NO_VALUE}</td>
              {/if}
            </tr>
          {/each}
        {/snippet}
      </Table>
    {:else}
      <ResultState title={i18n.t('No competition history available')} />
    {/if}
  </div>
</div>
