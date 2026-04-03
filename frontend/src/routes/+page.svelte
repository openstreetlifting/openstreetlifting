<script lang="ts">
	import type { PageData } from './$types';
	import { Card } from '$lib/components/ui';
	import { resolve } from '$app/paths';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>{data.title}</title>
	<meta name="description" content={data.description} />
</svelte:head>

<div
	class="relative mx-auto flex min-h-[calc(100vh-4rem)] max-w-7xl flex-col justify-center px-4 py-8 sm:px-6 sm:py-12"
>
	<!-- Subtle ambient background glow - very minimal -->
	<div class="pointer-events-none absolute inset-0 -z-10 overflow-hidden" aria-hidden="true">
		<div></div>
		<div
			class="ambient-glow-2 absolute top-0 right-1/3 h-64 w-64 rounded-full bg-gradient-to-br from-purple-500/6 to-transparent blur-3xl"
		></div>
	</div>

	<div class="mb-12 text-left sm:mb-16">
		<h1
			class="hero-title max-w-5xl text-7xl font-light tracking-tight text-white sm:text-8xl lg:text-9xl"
		>
			OpenStreetlifting
		</h1>
		<p
			class="mt-6 max-w-2xl text-xl leading-8 font-light text-zinc-500 sm:mt-8 sm:text-2xl sm:leading-9"
		>
			OpenStreetlifting is an <span class="keyword-glow">open</span>, collaborative project building
			a <span class="keyword-glow">permanent</span> and <span class="keyword-glow">traceable</span> archive
			of all streetlifting data, accessible to everyone.
		</p>
	</div>

	<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:gap-8">
		<a
			href={resolve('/rankings')}
			class="group block transition-all duration-200 hover:scale-[1.01]"
		>
			<Card
				class="relative h-full overflow-hidden border-zinc-800/60 p-8 backdrop-blur-sm transition-all duration-200 hover:border-zinc-700/60 hover:shadow-lg hover:shadow-zinc-800/50"
			>
				<div class="flex flex-col items-start gap-6">
					<div
						class="rounded-xl bg-zinc-800/60 p-3 transition-all duration-200 group-hover:bg-zinc-700/60"
					>
						<svg
							class="h-8 w-8 text-zinc-400 transition-colors group-hover:text-white"
							fill="currentColor"
							viewBox="0 0 24 24"
						>
							<path d="M5 3h4v18H5V3zm7 4h4v14h-4V7zm7 7h4v7h-4v-7z" />
						</svg>
					</div>
					<div class="space-y-2">
						<h2 class="text-2xl font-medium text-white">Global Ranking</h2>
						<p class="leading-relaxed font-light text-zinc-500">
							View worldwide rankings and compare performances across all athletes
						</p>
						<div
							class="flex items-center gap-2 pt-2 text-sm font-medium text-zinc-400 transition-colors group-hover:text-white"
						>
							<span>Explore rankings</span>
							<svg
								class="h-4 w-4 transition-transform group-hover:translate-x-1"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M17 8l4 4m0 0l-4 4m4-4H3"
								/>
							</svg>
						</div>
					</div>
				</div>
			</Card>
		</a>

		<!-- Competitions Card -->
		<a
			href={resolve('/competitions')}
			class="group block transition-all duration-200 hover:scale-[1.01]"
		>
			<Card
				class="relative h-full overflow-hidden border-zinc-800/60 p-8 backdrop-blur-sm transition-all duration-200 hover:border-zinc-700/60 hover:shadow-lg hover:shadow-zinc-800/50"
			>
				<div class="flex flex-col items-start gap-6">
					<div
						class="rounded-xl bg-zinc-800/60 p-3 transition-all duration-200 group-hover:bg-zinc-700/60"
					>
						<svg
							class="h-8 w-8 text-zinc-400 transition-colors group-hover:text-white"
							fill="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								d="M12 2L9.19 8.63 2 9.24l5.46 4.73L5.82 21 12 17.27 18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2z"
							/>
						</svg>
					</div>
					<div class="space-y-2">
						<h2 class="text-2xl font-medium text-white">Competitions</h2>
						<p class="leading-relaxed font-light text-zinc-500">
							Explore upcoming and past competitions from around the world
						</p>
						<div
							class="flex items-center gap-2 pt-2 text-sm font-medium text-zinc-400 transition-colors group-hover:text-white"
						>
							<span>Browse competitions</span>
							<svg
								class="h-4 w-4 transition-transform group-hover:translate-x-1"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M17 8l4 4m0 0l-4 4m4-4H3"
								/>
							</svg>
						</div>
					</div>
				</div>
			</Card>
		</a>
	</div>
</div>

<style lang="postcss">
	/* Apple-style lettering glow - subtle and thin around letters */
	.hero-title {
		text-shadow:
			0 0 15px rgba(59, 130, 246, 0.5),
			0 0 30px rgba(147, 51, 234, 0.4),
			0 0 45px rgba(236, 72, 153, 0.25);
	}

	.keyword-glow {
		color: rgb(228 228 231);
		text-shadow:
			0 0 10px rgba(59, 130, 246, 0.4),
			0 0 20px rgba(147, 51, 234, 0.3),
			0 0 30px rgba(236, 72, 153, 0.2);
	}

	/* Subtle ambient background - static */
	:global(.ambient-glow-1) {
		opacity: 0.4;
	}

	:global(.ambient-glow-2) {
		opacity: 0.3;
	}
</style>
