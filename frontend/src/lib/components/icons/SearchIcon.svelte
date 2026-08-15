<script lang="ts">
  interface Props {
    class?: string;
  }

  let { class: className = 'h-4 w-4' }: Props = $props();
</script>

<svg
  class={className}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  <circle cx="11" cy="11" r="7" />
  <line x1="16.5" y1="16.5" x2="21" y2="21" />
</svg>
