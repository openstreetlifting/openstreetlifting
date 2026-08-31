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
  stroke-width="2.25"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  <circle cx="12" cy="12" r="9" />
  <path d="M3 12h18" />
  <path d="M12 3a14 14 0 0 1 0 18a14 14 0 0 1 0 -18" />
</svg>
