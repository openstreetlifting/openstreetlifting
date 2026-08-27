<script lang="ts">
  import type { PageData } from './$types';
  import type { PersonalRecord } from '$lib/types/athlete';
  import { Card, Breadcrumb, Flag } from '$lib/components/ui';
  import { InstagramIcon } from '$lib/components/icons';
  import { resolve } from '$app/paths';
  import { formatDate, formatWeight, formatScore } from '$lib/utils';
  import { CELL, STATUS_FLAG, NO_VALUE } from '$lib/constants/table';
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
</script>

<svelte:head>
  <title>{athlete.first_name} {athlete.last_name} - OpenStreetlifting</title>
  <meta
    name="description"
    content="Competition history and personal records for {athlete.first_name} {athlete.last_name}"
  />
</svelte:head>

<div class="mx-auto max-w-[var(--content-max-width)] px-4 py-8 sm:px-6 sm:py-12">
  <Breadcrumb
    items={[
      { label: 'Rankings', href: '/' },
      { label: `${athlete.first_name} ${athlete.last_name}` },
    ]}
  />

  <div class="mb-12">
    <div class="mb-4 flex items-center gap-4">
      <Flag countryCode={athlete.country} class="text-xl" />
      <h1 class="{TEXT.title} text-white">
        {athlete.first_name}
        {athlete.last_name}
      </h1>

      {#if athlete.instagram_handle}
        <a
          href={`https://www.instagram.com/${athlete.instagram_handle}`}
          target="_blank"
          rel="noopener noreferrer"
          class="inline-flex items-center text-white transition-colors hover:text-zinc-300"
          aria-label="{athlete.first_name} {athlete.last_name} on Instagram"
          title="@{athlete.instagram_handle}"
        >
          <InstagramIcon class="h-7 w-7 sm:h-9 sm:w-9" />
        </a>
      {/if}
    </div>

    <div class="flex flex-wrap gap-x-6 gap-y-3 text-base text-zinc-400">
      <div class="flex items-center gap-2">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M3 21v-4m0 0V5a2 2 0 012-2h6.5l1 1H21l-3 6 3 6h-8.5l-1-1H5a2 2 0 00-2 2zm9-13.5V9"
          />
        </svg>
        <span>Country: {athlete.country}</span>
      </div>

      <div class="flex items-center gap-2">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
          />
        </svg>
        <span>Sex: {athlete.gender}</span>
      </div>

      <div class="flex items-center gap-2">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"
          />
        </svg>
        <span
          >{athlete.total_competitions} Competition{athlete.total_competitions !== 1
            ? 's'
            : ''}</span
        >
      </div>
    </div>
  </div>

  {#if athlete.personal_records && athlete.personal_records.length > 0}
    <div class="mb-8">
      <h2 class="mb-4 {TEXT.heading} text-white">Personal Records</h2>
      <div class="grid gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
        {#each sortPersonalRecords(athlete.personal_records) as pr (pr.movement_name)}
          <Card class="p-6 transition-colors hover:border-zinc-700/60">
            <div class="mb-2 text-sm font-medium text-zinc-400">{pr.movement_name}</div>
            <div class="mb-2 {TEXT.figure} text-white">{formatWeight(pr.max_weight)}</div>
            <div class="text-xs text-zinc-500">
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

  <div>
    <h2 class="mb-4 {TEXT.heading} text-white">Competition History</h2>
    {#if athlete.competitions && athlete.competitions.length > 0}
      <Card class="hidden p-4 md:block">
        <div class="overflow-x-auto">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-zinc-800">
                <th class="px-3 py-2 text-left font-medium text-zinc-400">Competition</th>
                <th class="px-3 py-2 text-left font-medium text-zinc-400">Date</th>
                {#if showsDivision}
                  <th class="px-3 py-2 text-left font-medium text-zinc-400">Division</th>
                {/if}
                <th class="px-3 py-2 text-left font-medium text-zinc-400">Category</th>
                <th class="px-3 py-2 text-center font-medium text-zinc-400">Rank</th>
                <th class="px-3 py-2 text-center font-medium text-zinc-400">Total</th>
                <th class="px-3 py-2 text-center font-medium text-zinc-400">RIS Score</th>
              </tr>
            </thead>
            <tbody>
              {#each athlete.competitions as competition (competition.competition_id)}
                <tr
                  class="border-b border-zinc-800/50 transition-colors even:bg-zinc-900/60 hover:bg-zinc-800/50 {competition.is_disqualified
                    ? 'opacity-50'
                    : ''}"
                >
                  <td class="px-3 py-2">
                    <a
                      href={resolve(`/competitions/${competition.competition_slug}`)}
                      class="text-white underline hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
                    >
                      {competition.competition_name}
                    </a>
                  </td>
                  <td class="px-3 py-2 {CELL.data}">{formatDate(competition.competition_date)}</td>
                  {#if showsDivision}
                    <td class="px-3 py-2 {CELL.data}">{competition.division || NO_VALUE}</td>
                  {/if}
                  <td class="px-3 py-2 {CELL.data}">{competition.category_name}</td>
                  <td class="px-3 py-2 text-center {CELL.identity}">
                    {#if competition.is_disqualified}
                      <span class={STATUS_FLAG}>DQ</span>
                    {:else}
                      {competition.rank || NO_VALUE}
                    {/if}
                  </td>
                  <td class="px-3 py-2 text-center {CELL.counted}">
                    {formatWeight(competition.total)}
                  </td>
                  <td class="px-3 py-2 text-center {CELL.counted}">
                    {formatScore(competition.ris_score)}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </Card>

      <!-- Mobile Card Layout -->
      <div class="grid gap-3 md:hidden">
        {#each athlete.competitions as competition (competition.competition_id)}
          <Card
            class="p-4 transition-all duration-200 hover:border-zinc-700/60 hover:shadow-lg hover:shadow-zinc-900/50 {competition.is_disqualified
              ? 'opacity-50'
              : ''}"
          >
            <a
              href={resolve(`/competitions/${competition.competition_slug}`)}
              class="mb-3 block text-base font-medium text-white hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
            >
              {competition.competition_name}
            </a>
            <div class="grid grid-cols-2 gap-3 text-sm">
              <div>
                <div class="text-xs text-zinc-500">Date</div>
                <div class={CELL.data}>{formatDate(competition.competition_date)}</div>
              </div>
              {#if competition.division}
                <div>
                  <div class="text-xs text-zinc-500">Division</div>
                  <div class={CELL.data}>{competition.division}</div>
                </div>
              {/if}
              <div>
                <div class="text-xs text-zinc-500">Category</div>
                <div class={CELL.data}>{competition.category_name}</div>
              </div>
              <div>
                <div class="text-xs text-zinc-500">Rank</div>
                <div class={CELL.identity}>
                  {#if competition.is_disqualified}
                    <span class={STATUS_FLAG}>DQ</span>
                  {:else}
                    {competition.rank || NO_VALUE}
                  {/if}
                </div>
              </div>
              <div>
                <div class="text-xs text-zinc-500">Total</div>
                <div class={CELL.counted}>{formatWeight(competition.total)}</div>
              </div>
              <div>
                <div class="text-xs text-zinc-500">RIS Score</div>
                <div class={CELL.counted}>{formatScore(competition.ris_score)}</div>
              </div>
            </div>
          </Card>
        {/each}
      </div>
    {:else}
      <Card class="p-8">
        <p class="text-center text-zinc-400">No competition history available</p>
      </Card>
    {/if}
  </div>
</div>
