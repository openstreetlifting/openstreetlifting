<script lang="ts">
  import type { HTMLAnchorAttributes, HTMLButtonAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import type { ResolvedPathname } from '$app/types';
  import { cn } from '#lib/utils/index.js';
  import { CONTROL } from '#lib/constants/typography.js';

  type ButtonVariant = 'default' | 'ghost' | 'outline';
  type ButtonSize = 'default' | 'sm' | 'lg' | 'icon' | 'control';

  interface Props extends HTMLButtonAttributes {
    variant?: ButtonVariant;
    size?: ButtonSize;
    href?: ResolvedPathname | `https://${string}`;
    target?: HTMLAnchorAttributes['target'];
    rel?: string;
    class?: string;
    children?: Snippet;
  }

  let {
    variant = 'default',
    size = 'default',
    href,
    class: className,
    children,
    ...rest
  }: Props = $props();

  const baseStyles =
    'inline-flex items-center justify-center whitespace-nowrap rounded-control font-mono font-medium transition-colors active:opacity-80 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2 focus-visible:ring-offset-canvas disabled:opacity-50 disabled:pointer-events-none';

  const variants = {
    default: 'bg-action text-on-action hover:bg-action-hover',
    ghost: 'hover:bg-surface-hover hover:text-ink',
    outline: 'border border-stroke-strong hover:bg-surface-hover',
  };

  const sizes = {
    default: 'min-h-11 py-2 px-4',
    sm: 'h-9 px-3',
    lg: 'h-11 px-8 text-sm',
    icon: 'h-11 w-11 sm:h-8 sm:w-8',
    control: 'h-11 px-3 sm:h-8',
  };

  const classes = $derived(cn(CONTROL, baseStyles, variants[variant], sizes[size], className));
</script>

{#if href}
  <!-- eslint-disable-next-line svelte/no-navigation-without-resolve -- callers supply resolved app paths or absolute HTTPS URLs -->
  <a {href} class={classes} {...rest as HTMLAnchorAttributes}>
    {@render children?.()}
  </a>
{:else}
  <button type="button" class={classes} {...rest as HTMLButtonAttributes}>
    {@render children?.()}
  </button>
{/if}
