<script lang="ts">
	import Header from './Header.svelte';
	import Footer from './Footer.svelte';
	import '../app.css';

	let { children } = $props();
</script>

<div class="flex min-h-screen flex-col bg-zinc-950">
	<Header />
	<main class="flex-1">
		{@render children()}
	</main>
	<Footer />
</div>
