<script lang="ts">
	let { class: className = 'h-5 w-5' }: { class?: string } = $props();
</script>

<svg
	class={className}
	fill="none"
	stroke="currentColor"
	viewBox="0 0 24 24"
	stroke-width="2"
	aria-hidden="true"
>
	<path
		stroke-linecap="round"
		stroke-linejoin="round"
		d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
	/>
</svg>
