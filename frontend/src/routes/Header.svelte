<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { GitHubIcon, InstagramIcon } from '$lib/components/icons';
</script>

<header class="bg-zinc-950">
	<nav class="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
		<a
			href={resolve('/')}
			class="text-lg font-light text-white transition-colors hover:text-zinc-200"
		>
			osl
		</a>

		<div class="flex items-center gap-4">
			<ul class="flex gap-6">
				<li>
					<a
						href={resolve('/')}
						class="text-sm font-medium transition-colors hover:text-white"
						class:text-white={page.url.pathname === '/'}
						class:text-zinc-400={page.url.pathname !== '/'}
						aria-current={page.url.pathname === '/' ? 'page' : undefined}
					>
						Home
					</a>
				</li>
				<li>
					<a
						href="https://docs.openstreetlifting.org/"
						target="_blank"
						rel="noopener noreferrer"
						class="text-sm font-medium text-zinc-400 transition-colors hover:text-white"
					>
						Docs
					</a>
				</li>
				<li>
					<a
						href={resolve('/contact')}
						class="text-sm font-medium transition-colors hover:text-white"
						class:text-white={page.url.pathname === '/contact'}
						class:text-zinc-400={page.url.pathname !== '/contact'}
						aria-current={page.url.pathname === '/contact' ? 'page' : undefined}
					>
						Contact
					</a>
				</li>
			</ul>

			<div class="flex items-center gap-3">
				<!-- GitHub Link -->
				<a
					href="https://github.com/openstreetlifting/openstreetlifting_backend"
					target="_blank"
					rel="noopener noreferrer"
					class="text-zinc-400 transition-colors hover:text-white"
					aria-label="GitHub"
				>
					<GitHubIcon />
				</a>

				<!-- Instagram Link -->
				<a
					href="https://www.instagram.com/openstreetlifting"
					target="_blank"
					rel="noopener noreferrer"
					class="text-zinc-400 transition-colors hover:text-pink-500"
					aria-label="Instagram"
				>
					<InstagramIcon />
				</a>
			</div>
		</div>
	</nav>
	<div class="h-px bg-zinc-800/50"></div>
</header>
