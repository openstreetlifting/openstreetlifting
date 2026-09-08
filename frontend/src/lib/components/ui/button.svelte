<script lang="ts">
  import type { HTMLAnchorAttributes, HTMLButtonAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import type { ResolvedPathname } from '$app/types';

  type ButtonVariant = 'default' | 'ghost' | 'outline';
  type ButtonSize = 'default' | 'sm' | 'lg' | 'icon';

  interface Props extends HTMLButtonAttributes {
    variant?: ButtonVariant;
    size?: ButtonSize;
    href?: ResolvedPathname;
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
    'inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2 focus-visible:ring-offset-canvas disabled:opacity-50 disabled:pointer-events-none';

  const variants = {
    default: 'bg-action text-on-action hover:bg-action-hover',
    ghost: 'hover:bg-surface-hover hover:text-ink',
    outline: 'border border-stroke hover:bg-surface-hover',
  };

  const sizes = {
    default: 'h-10 py-2 px-4',
    sm: 'h-9 px-3 text-sm',
    lg: 'h-11 px-8',
    icon: 'h-10 w-10',
  };

  const classes = $derived(`${baseStyles} ${variants[variant]} ${sizes[size]} ${className || ''}`);
</script>

{#if href}
  <a {href} class={classes} {...rest as HTMLAnchorAttributes}>
    {@render children?.()}
  </a>
{:else}
  <button class={classes} {...rest as HTMLButtonAttributes}>
    {@render children?.()}
  </button>
{/if}
