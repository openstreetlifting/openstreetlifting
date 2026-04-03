<script lang="ts">
	import type { PageData } from './$types';
	import { Card, Breadcrumb } from '$lib/components/ui';
	import { SortIcon } from '$lib/components/icons';
	import { rankingsService } from '$lib/api';
	import type { RankingEntry } from '$lib/types/ranking';
	import { resolve } from '$app/paths';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { SvelteURLSearchParams } from 'svelte/reactivity';

	let { data }: { data: PageData } = $props();

	let rankings = $state<RankingEntry[]>(data.initialRankings);
	let currentPage = $state(data.pagination.page);
	let totalPages = $state(data.pagination.total_pages);
	let totalItems = $state(data.pagination.total_items);
	let isLoading = $state(false);

	let genderFilter = $state<string | null>($page.url.searchParams.get('gender') || null);
	let countryFilter = $state<string>($page.url.searchParams.get('country') || '');
	let movementFilter = $state<string>($page.url.searchParams.get('movement') || 'total');

	const movements = [
		{ value: 'total', label: 'Total' },
		{ value: 'muscleup', label: 'Muscle Up' },
		{ value: 'pullup', label: 'Pull Up' },
		{ value: 'dips', label: 'Dips' },
		{ value: 'squat', label: 'Squat' }
	];

	const genders = [
		{ value: null, label: 'All' },
		{ value: 'M', label: 'Men' },
		{ value: 'F', label: 'Women' }
	];

	function updateURL() {
		const params = new SvelteURLSearchParams();

		if (movementFilter) {
			params.set('movement', movementFilter);
		}

		if (genderFilter) {
			params.set('gender', genderFilter);
		}

		if (countryFilter) {
			params.set('country', countryFilter);
		}

		const queryString = params.toString();
		const path = queryString ? `/rankings?${queryString}` : '/rankings';
		goto(resolve(path), { replaceState: true, keepFocus: true });
	}

	async function loadRankings(resetData = true) {
		if (isLoading) return;

		isLoading = true;
		try {
			const response = await rankingsService.getGlobalRankings({
				pagination: resetData ? 1 : currentPage + 1,
				gender: genderFilter,
				country: countryFilter || null,
				movement: movementFilter
			});

			if (resetData) {
				rankings = response.data;
				currentPage = 1;
			} else {
				rankings = [...rankings, ...response.data];
				currentPage += 1;
			}

			totalPages = response.pagination.total_pages;
			totalItems = response.pagination.total_items;
		} catch (error) {
			console.error('Error loading rankings:', error);
		} finally {
			isLoading = false;
		}
	}

	async function handleFilterChange() {
		updateURL();
		await loadRankings(true);
	}

	async function loadMore() {
		await loadRankings(false);
	}

	function formatWeight(weight: number): string {
		return weight && weight > 0 ? `${weight} kg` : '-';
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function getCountryFlag(countryCode: string): string {
		if (!countryCode || countryCode.length !== 2) return '';
		const codePoints = [...countryCode.toUpperCase()].map((char) => 127397 + char.charCodeAt(0));
		return String.fromCodePoint(...codePoints);
	}

	function formatRIS(ris: number): string {
		return ris && ris > 0 ? ris.toFixed(2) : '-';
	}
</script>

<svelte:head>
	<title>Global Rankings - OpenStreetlifting</title>
	<meta name="description" content="Global streetlifting rankings by movement and category" />
</svelte:head>

<div class="mx-auto max-w-7xl px-6 py-12">
	<Breadcrumb items={[{ label: 'Home', href: '/' }, { label: 'Rankings' }]} />

	<div class="mb-8">
		<h1 class="mb-4 text-5xl font-light text-white">Global Rankings</h1>
		<p class="text-lg font-extralight text-zinc-400">
			World's best streetlifting performances across all movements
		</p>
	</div>

	<div class="mb-6 flex flex-wrap gap-2">
		{#each genders as gender (gender.label)}
			<button
				onclick={() => {
					genderFilter = gender.value;
					handleFilterChange();
				}}
				class="rounded-lg px-4 py-2 text-sm font-medium transition-colors focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none
					{genderFilter === gender.value
					? 'bg-blue-500 text-white'
					: 'bg-zinc-800/50 text-zinc-400 hover:bg-zinc-800 hover:text-zinc-300'}"
			>
				{gender.label}
			</button>
		{/each}
	</div>

	<div class="mb-6">
		<div class="relative">
			<svg
				class="absolute top-1/2 left-3 h-5 w-5 -translate-y-1/2 text-zinc-500"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
				stroke-width="2"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
				/>
			</svg>
			<input
				type="text"
				bind:value={countryFilter}
				onkeydown={(e) => {
					if (e.key === 'Enter') {
						handleFilterChange();
					}
				}}
				placeholder="Filter by country code (e.g., FR, US, GB)..."
				class="w-full rounded-lg border border-zinc-800 bg-zinc-900/50 py-2.5 pr-4 pl-10 text-sm text-white placeholder-zinc-500 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
			/>
		</div>
	</div>

	{#if data.error}
		<Card class="p-8">
			<div class="text-center">
				<p class="text-red-400">{data.error}</p>
			</div>
		</Card>
	{:else if rankings.length === 0 && !isLoading}
		<Card class="p-8">
			<div class="text-center">
				<p class="text-zinc-400">No rankings found for the selected filters</p>
				<button
					onclick={() => {
						genderFilter = null;
						countryFilter = '';
						movementFilter = 'total';
						handleFilterChange();
					}}
					class="mt-4 text-sm text-zinc-500 underline hover:text-zinc-300 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
				>
					Clear filters
				</button>
			</div>
		</Card>
	{:else}
		<Card class="p-6">
			<div class="overflow-x-auto">
				<table class="w-full text-sm">
					<thead class="sticky top-0 z-10">
						<tr class="border-b border-zinc-800 bg-zinc-900 shadow-lg shadow-zinc-950/50">
							<th class="px-4 py-3 text-left font-medium text-zinc-400">Rank</th>
							<th class="px-4 py-3 text-left font-medium text-zinc-400">Athlete</th>
							<th class="px-4 py-3 text-left font-medium text-zinc-400">Country</th>
							<th class="px-4 py-3 text-left font-medium text-zinc-400">RIS</th>
							{#each movements as movement (movement.value)}
								<th
									class="cursor-pointer px-4 py-3 text-left font-medium transition-colors select-none hover:text-white {movementFilter ===
									movement.value
										? 'text-white'
										: 'text-zinc-400'}"
									onclick={() => {
										movementFilter = movement.value;
										handleFilterChange();
									}}
								>
									{movement.label}
									<SortIcon
										direction={movementFilter === movement.value ? 'desc' : 'none'}
										class="ml-1"
									/>
								</th>
							{/each}
							<th class="px-4 py-3 text-left font-medium text-zinc-400">Competition</th>
							<th class="px-4 py-3 text-left font-medium text-zinc-400">Date</th>
						</tr>
					</thead>
					<tbody>
						{#each rankings as entry (entry.rank + entry.athlete.athlete_id)}
							<tr
								class="border-b border-zinc-800/50 transition-colors hover:bg-zinc-900/30
								{entry.rank === 1
									? 'bg-gradient-to-r from-yellow-500/5 via-transparent to-transparent shadow-lg shadow-yellow-900/20'
									: entry.rank === 2
										? 'bg-gradient-to-r from-zinc-400/5 via-transparent to-transparent'
										: entry.rank === 3
											? 'bg-gradient-to-r from-orange-700/5 via-transparent to-transparent'
											: ''}"
							>
								<td class="px-4 py-3 text-white">
									{#if entry.rank === 1}
										<span class="font-semibold text-yellow-400">🥇 {entry.rank}</span>
									{:else if entry.rank === 2}
										<span class="font-semibold text-zinc-300">🥈 {entry.rank}</span>
									{:else if entry.rank === 3}
										<span class="font-semibold text-orange-400">🥉 {entry.rank}</span>
									{:else}
										{entry.rank}
									{/if}
								</td>
								<td class="px-4 py-3 text-white">
									<a
										href={resolve(`/athletes/${entry.athlete.slug}`)}
										class="hover:text-zinc-300 hover:underline"
									>
										{entry.athlete.first_name}
										{entry.athlete.last_name}
									</a>
								</td>
								<td class="px-4 py-3">
									<div class="flex items-center gap-2 text-zinc-400">
										<span class="text-lg" title={entry.athlete.country}>
											{getCountryFlag(entry.athlete.country)}
										</span>
										<span>{entry.athlete.country}</span>
									</div>
								</td>
								<td class="px-4 py-3 font-medium text-zinc-400">{formatRIS(entry.ris)}</td>
								<td class="px-4 py-3 font-medium text-zinc-400">{formatWeight(entry.total)}</td>
								<td class="px-4 py-3 text-zinc-400">{formatWeight(entry.muscleup)}</td>
								<td class="px-4 py-3 text-zinc-400">{formatWeight(entry.pullup)}</td>
								<td class="px-4 py-3 text-zinc-400">{formatWeight(entry.dips)}</td>
								<td class="px-4 py-3 text-zinc-400">{formatWeight(entry.squat)}</td>
								<td class="px-4 py-3 text-zinc-400">
									<a
										href={resolve(`/competitions/${entry.competition.name}`)}
										class="hover:text-zinc-300 hover:underline"
									>
										{entry.competition.name}
									</a>
								</td>
								<td class="px-4 py-3 text-zinc-400">{formatDate(entry.competition.date)}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</Card>

		{#if currentPage < totalPages}
			<div class="mt-4">
				<button
					onclick={loadMore}
					disabled={isLoading}
					class="w-full rounded-lg border border-zinc-800 bg-zinc-900/50 px-4 py-3 text-sm font-medium text-zinc-300 transition-colors hover:border-zinc-700 hover:bg-zinc-900 hover:text-white focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
				>
					{isLoading ? 'Loading...' : `Load More (${rankings.length} of ${totalItems})`}
				</button>
			</div>
		{/if}

		{#if rankings.length > 0}
			<div class="mt-4 text-center text-sm text-zinc-500">
				Showing {rankings.length} of {totalItems} rankings
			</div>
		{/if}
	{/if}
</div>
