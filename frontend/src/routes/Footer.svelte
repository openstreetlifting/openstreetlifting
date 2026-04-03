<script lang="ts">
	import { resolve } from '$app/paths';
	import { GitHubIcon, InstagramIcon, DiscordIcon, EmailIcon } from '$lib/components/icons';
</script>

<footer class="border-t border-zinc-800/50 bg-zinc-950">
	<div class="mx-auto max-w-7xl px-6 py-16">
		<div class="grid gap-12 md:grid-cols-2 lg:grid-cols-4">
			<div>
				<h3 class="mb-4 text-sm font-medium text-white">Product</h3>
				<ul class="space-y-3">
					<li>
						<a
							href={resolve('/')}
							class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
						>
							Home
						</a>
					</li>
					<li>
						<a
							href={resolve('/rankings')}
							class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
						>
							Rankings
						</a>
					</li>
					<li>
						<a
							href={resolve('/competitions')}
							class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
						>
							Competitions
						</a>
					</li>
					<li>
						<a
							href={resolve('/athletes')}
							class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
						>
							Athletes
						</a>
					</li>
				</ul>
			</div>

			<div>
				<h3 class="mb-4 text-sm font-medium text-white">Resources</h3>
				<ul class="space-y-3">
					<li>
						<a
							href="https://docs.openstreetlifting.org/"
							target="_blank"
							rel="noopener noreferrer"
							class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
						>
							Documentation
						</a>
					</li>
					<li>
						<a
							href={resolve('/contact')}
							class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
						>
							Contact
						</a>
					</li>
				</ul>
			</div>

			<div>
				<h3 class="mb-4 text-sm font-medium text-white">Legal</h3>
				<ul class="space-y-3">
					<li>
						<a
							href={resolve('/privacy')}
							class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
						>
							Privacy Policy
						</a>
					</li>
					<li>
						<a
							href={resolve('/terms')}
							class="text-sm font-light text-zinc-400 transition-colors hover:text-white"
						>
							Terms of Service
						</a>
					</li>
				</ul>
			</div>

			<div>
				<h3 class="mb-4 text-sm font-medium text-white">Connect</h3>
				<div class="flex gap-3">
					<a
						href="https://github.com/openstreetlifting"
						target="_blank"
						rel="noopener noreferrer"
						class="text-zinc-400 transition-colors hover:text-white"
						aria-label="GitHub"
					>
						<GitHubIcon />
					</a>
					<a
						href="https://www.instagram.com/openstreetlifting"
						target="_blank"
						rel="noopener noreferrer"
						class="text-zinc-400 transition-colors hover:text-pink-500"
						aria-label="Instagram"
					>
						<InstagramIcon />
					</a>
					<a
						href="https://discord.gg/MR9z7AnT"
						target="_blank"
						rel="noopener noreferrer"
						class="text-zinc-400 transition-colors hover:text-indigo-400"
						aria-label="Discord"
					>
						<DiscordIcon />
					</a>
					<a
						href="mailto:adrien.pelfresne@gmail.com"
						class="text-zinc-400 transition-colors hover:text-white"
						aria-label="Email"
					>
						<EmailIcon />
					</a>
				</div>
			</div>
		</div>

		<div class="mt-12 border-t border-zinc-800/50 pt-8">
			<p class="text-sm font-light text-zinc-500">
				© {new Date().getFullYear()} OpenStreetlifting. Open source under the
				<a
					href="https://www.gnu.org/licenses/gpl-3.0.html"
					target="_blank"
					rel="noopener noreferrer"
					class="text-zinc-400 transition-colors hover:text-white"
				>
					GNU General Public License v3.0
				</a>.
			</p>
		</div>
	</div>
</footer>
