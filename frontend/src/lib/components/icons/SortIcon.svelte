<script lang="ts">
	type SortDirection = 'none' | 'asc' | 'desc';

	let { direction = 'none', class: className = '' }: { direction?: SortDirection; class?: string } =
		$props();
</script>

{#if direction === 'none'}
	<svg
		class="inline-block h-4 w-4 transition-transform {className}"
		fill="none"
		stroke="currentColor"
		viewBox="0 0 24 24"
		stroke-width="2"
	>
		<path
			stroke-linecap="round"
			stroke-linejoin="round"
			d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"
		/>
	</svg>
{:else if direction === 'asc'}
	<svg
		class="inline-block h-4 w-4 transition-transform {className}"
		fill="none"
		stroke="currentColor"
		viewBox="0 0 24 24"
		stroke-width="2"
	>
		<path stroke-linecap="round" stroke-linejoin="round" d="M5 15l7-7 7 7" />
	</svg>
{:else}
	<svg
		class="inline-block h-4 w-4 transition-transform {className}"
		fill="none"
		stroke="currentColor"
		viewBox="0 0 24 24"
		stroke-width="2"
	>
		<path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
	</svg>
{/if}
