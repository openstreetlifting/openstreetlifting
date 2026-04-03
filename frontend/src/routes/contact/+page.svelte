<script lang="ts">
	import { Card } from '$lib/components/ui';
	import { EmailIcon, InstagramIcon, GitHubIcon, DiscordIcon } from '$lib/components/icons';
</script>

<svelte:head>
	<title>Contact - OpenStreetlifting</title>
	<meta name="description" content="Get in touch with OpenStreetlifting" />
</svelte:head>

<div class="mx-auto max-w-4xl px-6 py-16">
	<!-- Header -->
	<div class="mb-16 text-center">
		<h1 class="mb-4 text-5xl font-light tracking-tight text-white">Get in touch</h1>
		<p class="text-base font-light text-zinc-500">
			Have questions or want to connect? Reach out through any of these channels
		</p>
	</div>

	<!-- Contact Cards -->
	<div class="space-y-4">
		<!-- Row 1: GitHub & Instagram -->
		<div class="grid gap-4 md:grid-cols-2">
			<!-- GitHub Card -->
			<a
				href="https://github.com/openstreetlifting"
				target="_blank"
				rel="noopener noreferrer"
				class="group block transition-all duration-200 hover:scale-[1.02]"
			>
				<Card
					class="relative overflow-hidden border-zinc-800/60 p-8 backdrop-blur-sm transition-all duration-200 hover:border-zinc-700/60 hover:shadow-lg hover:shadow-zinc-800/50"
				>
					<!-- Icon -->
					<div
						class="mb-6 inline-flex h-14 w-14 items-center justify-center rounded-xl bg-zinc-800/60 transition-all duration-200 group-hover:bg-zinc-700/60"
					>
						<GitHubIcon class="h-7 w-7 text-zinc-400 transition-colors group-hover:text-white" />
					</div>

					<!-- Content -->
					<div class="space-y-3">
						<h2 class="text-xl font-medium text-white">GitHub</h2>
						<p class="text-sm leading-relaxed font-light text-zinc-500">
							Check out our open source code and contribute to the project
						</p>
						<div
							class="flex items-center gap-2 pt-2 text-sm font-medium text-zinc-300 transition-colors group-hover:text-white"
						>
							<span>OpenStreetlifting</span>
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

			<!-- Instagram Card -->
			<a
				href="https://www.instagram.com/openstreetlifting"
				target="_blank"
				rel="noopener noreferrer"
				class="group block transition-all duration-200 hover:scale-[1.02]"
			>
				<Card
					class="relative overflow-hidden border-zinc-800/60 p-8 backdrop-blur-sm transition-all duration-200 hover:border-pink-900/40 hover:shadow-lg hover:shadow-pink-900/20"
				>
					<!-- Icon -->
					<div
						class="mb-6 inline-flex h-14 w-14 items-center justify-center rounded-xl bg-zinc-800/60 transition-all duration-200 group-hover:bg-gradient-to-br group-hover:from-pink-600/20 group-hover:to-purple-600/20"
					>
						<InstagramIcon
							class="h-7 w-7 text-zinc-400 transition-colors group-hover:text-pink-400"
						/>
					</div>

					<!-- Content -->
					<div class="space-y-3">
						<h2 class="text-xl font-medium text-white">Instagram</h2>
						<p class="text-sm leading-relaxed font-light text-zinc-500">
							Follow us for updates, highlights, and community content
						</p>
						<div
							class="flex items-center gap-2 pt-2 text-sm font-medium text-zinc-300 transition-colors group-hover:text-pink-400"
						>
							<span>@openstreetlifting</span>
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
		</div>

		<!-- Row 2: Discord & Email -->
		<div class="grid gap-4 md:grid-cols-2">
			<!-- Discord Card -->
			<a
				href="https://discord.gg/MR9z7AnT"
				target="_blank"
				rel="noopener noreferrer"
				class="group block transition-all duration-200 hover:scale-[1.02]"
			>
				<Card
					class="relative overflow-hidden border-zinc-800/60 p-8 backdrop-blur-sm transition-all duration-200 hover:border-indigo-900/40 hover:shadow-lg hover:shadow-indigo-900/20"
				>
					<!-- Icon -->
					<div
						class="mb-6 inline-flex h-14 w-14 items-center justify-center rounded-xl bg-zinc-800/60 transition-all duration-200 group-hover:bg-gradient-to-br group-hover:from-indigo-600/20 group-hover:to-blue-600/20"
					>
						<DiscordIcon
							class="h-7 w-7 text-zinc-400 transition-colors group-hover:text-indigo-400"
						/>
					</div>

					<!-- Content -->
					<div class="space-y-3">
						<h2 class="text-xl font-medium text-white">Discord</h2>
						<p class="text-sm leading-relaxed font-light text-zinc-500">
							Join our community to chat, ask questions, and connect with others
						</p>
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

			<!-- Email Card -->
			<a
				href="mailto:adrien.pelfresne@gmail.com"
				class="group block transition-all duration-200 hover:scale-[1.02]"
			>
				<Card
					class="relative overflow-hidden border-zinc-800/60 bg-gradient-to-br from-zinc-900/50 to-zinc-900/30 p-8 backdrop-blur-sm transition-all duration-200 hover:border-zinc-700/60 hover:shadow-lg hover:shadow-zinc-900/50"
				>
					<div class="relative z-10">
						<!-- Icon -->
						<div
							class="mb-6 inline-flex h-14 w-14 items-center justify-center rounded-xl bg-zinc-800/60 transition-all duration-200 group-hover:bg-zinc-800/80 group-hover:shadow-lg group-hover:shadow-zinc-900/50"
						>
							<EmailIcon class="h-7 w-7 text-zinc-400 transition-colors group-hover:text-white" />
						</div>

						<!-- Content -->
						<div class="space-y-3">
							<h2 class="text-xl font-medium text-white">Email</h2>
							<p class="text-sm leading-relaxed font-light text-zinc-500">
								Send us an email and we'll get back to you as soon as possible
							</p>
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
