<script lang="ts">
  import type { PageData } from './$types';
  import { Card } from '$lib/components/ui';
  import { resolve } from '$app/paths';
  import Logo from '$lib/components/ui/logo.svelte';

  let { data }: { data: PageData } = $props();
</script>

<svelte:head>
  <title>{data.title}</title>
  <meta name="description" content={data.description} />
</svelte:head>

<div
  class="relative mx-auto flex min-h-[calc(100vh-4rem)] max-w-7xl flex-col justify-center px-4 py-8 sm:px-6 sm:py-12"
>
  <div class="pointer-events-none absolute inset-0 -z-10 overflow-hidden" aria-hidden="true">
    <div></div>
    <div
      class="ambient-glow-2 absolute top-0 right-1/3 h-64 w-64 rounded-full bg-gradient-to-br from-purple-500/6 to-transparent blur-3xl"
    ></div>
  </div>

  <div class="mb-12 text-left sm:mb-16">
    <Logo></Logo>
    <p
      class="mt-6 max-w-2xl text-xl leading-8 font-light text-zinc-500 sm:mt-8 sm:text-2xl sm:leading-9"
    >
      OpenStreetlifting is an <span class="keyword-glow">open</span>, project building a
      <span class="keyword-glow">permanent</span>
      and <span class="keyword-glow">traceable</span> archive of all streetlifting data, accessible to
      everyone.
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
              View and Filter of worldwide rankings
            </p>
          </div>
        </div>
      </Card>
    </a>
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
              Explore upcoming and past competitions
            </p>
          </div>
        </div>
      </Card>
    </a>
  </div>
</div>

<style lang="postcss">
  .keyword-glow {
    color: rgb(228 228 231);
    text-shadow:
      0 0 10px rgba(239, 68, 68, 0.5),
      0 0 20px rgba(220, 38, 38, 0.4),
      0 0 30px rgba(185, 28, 28, 0.3);
  }

  /* Subtle ambient background - static */
  :global(.ambient-glow-1) {
    opacity: 0.4;
  }

  :global(.ambient-glow-2) {
    opacity: 0.3;
  }
</style>
