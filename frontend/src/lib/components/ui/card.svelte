<script lang="ts">
	import { cn } from '$lib/utils';
	import type { Snippet } from 'svelte';

	let {
		class: className,
		children,
		...restProps
	}: {
		class?: string;
		children?: Snippet;
		[key: string]: unknown;
	} = $props();
</script>

<div
	class={cn('rounded-xl border border-zinc-800 bg-zinc-900/50 text-zinc-50 shadow', className)}
	{...restProps}
>
	{#if children}
		{@render children()}
	{/if}
</div>
