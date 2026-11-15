<script lang="ts">
  interface Props {
    class?: string;
  }

  let { class: className = 'h-5 w-5' }: Props = $props();
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
  <circle cx="12" cy="12" r="9" />
  <line x1="12" y1="11" x2="12" y2="16" />
  <line x1="12" y1="8" x2="12" y2="8" />
</svg>
