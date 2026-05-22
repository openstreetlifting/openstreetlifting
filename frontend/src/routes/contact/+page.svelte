<script lang="ts">
  import { Card } from '$lib/components/ui';
  import { EmailIcon, InstagramIcon, GitHubIcon, DiscordIcon } from '$lib/components/icons';
</script>

<svelte:head>
  <title>Contact - OpenStreetlifting</title>
  <meta name="description" content="Get in touch with OpenStreetlifting" />
</svelte:head>

<div class="mx-auto max-w-4xl px-6 py-16">
  <div class="mb-16 text-center">
    <h1 class="mb-4 text-5xl font-light tracking-tight text-white">Get in touch</h1>
    <p class="text-base font-light text-zinc-500">
      If you have a question, want to help me, or simply want to connect, Reach out through any of
      these channels. I'm a solo developer on this project, i may take a little bit of time to
      respond.
    </p>
  </div>

  <div class="space-y-4">
    <div class="grid gap-4 md:grid-cols-2">
      <a
        href="https://github.com/openstreetlifting/openstreetlifting"
        target="_blank"
        rel="noopener noreferrer"
        class="group block transition-all duration-200 hover:scale-[1.02]"
      >
        <Card
          class="relative overflow-hidden border-zinc-800/60 p-8 backdrop-blur-sm transition-all duration-200 hover:border-zinc-700/60 hover:shadow-lg hover:shadow-zinc-800/50"
        >
          <div
            class="mb-6 inline-flex h-14 w-14 items-center justify-center rounded-xl bg-zinc-800/60 transition-all duration-200 group-hover:bg-zinc-700/60"
          >
            <GitHubIcon class="h-7 w-7 text-zinc-400 transition-colors group-hover:text-white" />
          </div>

          <div class="space-y-3">
            <h2 class="text-xl font-medium text-white">GitHub</h2>
            <div
              class="flex items-center gap-2 pt-2 text-sm font-medium text-zinc-300 transition-colors group-hover:text-white"
            >
              <span>OpenStreetlifting</span>
            </div>
          </div>
        </Card>
      </a>
      <a
        href="https://www.instagram.com/openstreetlifting"
        target="_blank"
        rel="noopener noreferrer"
        class="group block transition-all duration-200 hover:scale-[1.02]"
      >
        <Card
          class="relative overflow-hidden border-zinc-800/60 p-8 backdrop-blur-sm transition-all duration-200 hover:border-pink-900/40 hover:shadow-lg hover:shadow-pink-900/20"
        >
          <div
            class="mb-6 inline-flex h-14 w-14 items-center justify-center rounded-xl bg-zinc-800/60 transition-all duration-200 group-hover:bg-gradient-to-br group-hover:from-pink-600/20 group-hover:to-purple-600/20"
          >
            <InstagramIcon
              class="h-7 w-7 text-zinc-400 transition-colors group-hover:text-pink-400"
            />
          </div>

          <div class="space-y-3">
            <h2 class="text-xl font-medium text-white">Instagram</h2>
            <div
              class="flex items-center gap-2 pt-2 text-sm font-medium text-zinc-300 transition-colors group-hover:text-pink-400"
            >
              <span>@openstreetlifting</span>
            </div>
          </div>
        </Card>
      </a>
    </div>

    <div class="grid gap-4 md:grid-cols-2">
      <a
        href="https://discord.gg/MR9z7AnT"
        target="_blank"
        rel="noopener noreferrer"
        class="group block transition-all duration-200 hover:scale-[1.02]"
      >
        <Card
          class="relative overflow-hidden border-zinc-800/60 p-8 backdrop-blur-sm transition-all duration-200 hover:border-indigo-900/40 hover:shadow-lg hover:shadow-indigo-900/20"
        >
          <div
            class="mb-6 inline-flex h-14 w-14 items-center justify-center rounded-xl bg-zinc-800/60 transition-all duration-200 group-hover:bg-gradient-to-br group-hover:from-indigo-600/20 group-hover:to-blue-600/20"
          >
            <DiscordIcon
              class="h-7 w-7 text-zinc-400 transition-colors group-hover:text-indigo-400"
            />
          </div>

          <div class="space-y-3">
            <h2 class="text-xl font-medium text-white">Discord</h2>
            <div
              class="flex items-center gap-2 pt-2 text-sm font-medium text-zinc-300 transition-colors group-hover:text-indigo-400"
            >
              <span>Join Server</span>
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
        </Card>
      </a>

      <a
        href="mailto:adrien.pelfresne@gmail.com"
        class="group block transition-all duration-200 hover:scale-[1.02]"
      >
        <Card
          class="relative overflow-hidden border-zinc-800/60 bg-gradient-to-br from-zinc-900/50 to-zinc-900/30 p-8 backdrop-blur-sm transition-all duration-200 hover:border-zinc-700/60 hover:shadow-lg hover:shadow-zinc-900/50"
        >
          <div class="relative z-10">
            <div
              class="mb-6 inline-flex h-14 w-14 items-center justify-center rounded-xl bg-zinc-800/60 transition-all duration-200 group-hover:bg-zinc-800/80 group-hover:shadow-lg group-hover:shadow-zinc-900/50"
            >
              <EmailIcon class="h-7 w-7 text-zinc-400 transition-colors group-hover:text-white" />
            </div>

            <div class="space-y-3">
              <h2 class="text-xl font-medium text-white">Email</h2>
              <div
                class="flex items-center gap-2 pt-2 text-sm font-medium text-zinc-300 transition-colors group-hover:text-white"
              >
                <span>adrien.pelfresne@gmail.com</span>
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

          <!-- Gradient overlay on hover -->
          <div
            class="absolute inset-0 -z-10 bg-gradient-to-br from-zinc-800/0 via-zinc-800/0 to-zinc-800/5 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
          ></div>
        </Card>
      </a>
    </div>
  </div>
</div>
