<script lang="ts">
	let { movement, class: className = '' }: { movement: string; class?: string } = $props();

	const normalizedMovement = movement.toLowerCase().trim();
</script>

{#if normalizedMovement.includes('muscle') && normalizedMovement.includes('up')}
	<!-- Muscle-up Icon -->
	<svg class={className} fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
		<path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z" />
	</svg>
{:else if normalizedMovement.includes('pull') && normalizedMovement.includes('up')}
	<!-- Pull-up Icon -->
	<svg class={className} fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
		<path stroke-linecap="round" stroke-linejoin="round" d="M5 10l7-7m0 0l7 7m-7-7v18" />
	</svg>
{:else if normalizedMovement.includes('dip')}
	<!-- Dips Icon -->
	<svg class={className} fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
		<path stroke-linecap="round" stroke-linejoin="round" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
	</svg>
{:else if normalizedMovement.includes('squat')}
	<!-- Squat Icon -->
	<svg class={className} fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
		<path
			stroke-linecap="round"
			stroke-linejoin="round"
			d="M4 7v10c0 2 1 3 3 3h10c2 0 3-1 3-3V7M8 7V5c0-1.5 1-2 2-2h4c1 0 2 .5 2 2v2M7 12h10"
		/>
	</svg>
{:else}
	<!-- Default Icon -->
	<svg class={className} fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
		<path
			stroke-linecap="round"
			stroke-linejoin="round"
			d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"
		/>
	</svg>
{/if}
