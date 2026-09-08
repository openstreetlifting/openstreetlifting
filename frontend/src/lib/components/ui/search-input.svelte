<script lang="ts">
  import { SearchIcon } from '$lib/components/icons';
  import { FIELD } from '$lib/constants/typography';

  interface Props {
    value: string;
    placeholder?: string;
    delay?: number;
    onSearch: () => void;
  }

  let { value = $bindable(), placeholder = 'Search', delay = 300, onSearch }: Props = $props();

  let timer: ReturnType<typeof setTimeout> | undefined;

  function schedule() {
    clearTimeout(timer);
    timer = setTimeout(onSearch, delay);
  }

  function flush() {
    clearTimeout(timer);
    onSearch();
  }

  function clear() {
    value = '';
    flush();
  }
</script>

<div class="relative">
  <SearchIcon
    class="pointer-events-none absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-muted"
  />
  <input
    type="text"
    bind:value
    {placeholder}
    oninput={schedule}
    onkeydown={(event) => event.key === 'Enter' && flush()}
    class="w-full {FIELD} py-2 pr-8 pl-9 placeholder:text-muted"
  />
  {#if value}
    <button
      type="button"
      onclick={clear}
      aria-label="Clear search"
      class="absolute top-1/2 right-2 -translate-y-1/2 rounded px-1 text-muted transition-colors hover:text-ink"
    >
      &times;
    </button>
  {/if}
</div>
