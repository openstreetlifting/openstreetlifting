<script lang="ts">
  import { SearchIcon } from '$lib/components/icons';

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
    class="pointer-events-none absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-zinc-500"
  />
  <input
    type="text"
    bind:value
    {placeholder}
    oninput={schedule}
    onkeydown={(event) => event.key === 'Enter' && flush()}
    class="w-full rounded-lg border border-zinc-800 bg-zinc-900/50 py-2 pr-8 pl-9 text-sm text-zinc-300 placeholder:text-zinc-500 transition-colors focus:border-zinc-700 focus:ring-2 focus:ring-zinc-500 focus:ring-offset-2 focus:ring-offset-zinc-950 focus:outline-none"
  />
  {#if value}
    <button
      type="button"
      onclick={clear}
      aria-label="Clear search"
      class="absolute top-1/2 right-2 -translate-y-1/2 rounded px-1 text-zinc-500 transition-colors hover:text-zinc-300"
    >
      &times;
    </button>
  {/if}
</div>
