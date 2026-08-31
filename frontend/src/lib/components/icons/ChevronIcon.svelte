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
  <polyline points="6 9 12 15 18 9" />
</svg>
