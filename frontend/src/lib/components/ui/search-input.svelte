<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { SearchIcon } from '#lib/components/icons/index.js';
  import { FIELD } from '#lib/constants/typography.js';
  import { onDestroy } from 'svelte';
  import { beforeNavigate } from '$app/navigation';

  const i18n = useI18n();

  interface Props {
    id?: string;
    value: string;
    placeholder?: string;
    delay?: number;
    onSearch: () => void;
  }

  const inputId = $props.id();
  let {
    id = inputId,
    value = $bindable(),
    placeholder = 'Search',
    delay = 300,
    onSearch,
  }: Props = $props();

  let timer: ReturnType<typeof setTimeout> | undefined;
  let composing = false;

  function cancel() {
    clearTimeout(timer);
  }

  beforeNavigate(cancel);
  onDestroy(cancel);

  function schedule() {
    cancel();
    if (!composing) timer = setTimeout(onSearch, delay);
  }

  function flush() {
    cancel();
    onSearch();
  }

  function clear() {
    value = '';
    flush();
  }
</script>

<div class="relative">
  <label for={id} class="sr-only">{placeholder}</label>
  <SearchIcon
    class="pointer-events-none absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-muted"
  />
  <input
    {id}
    name="q"
    autocomplete="off"
    spellcheck={false}
    type="text"
    bind:value
    placeholder={`${placeholder.replace(/(?:\.\.\.|…)$/, '')}…`}
    oninput={schedule}
    onkeydown={(event) => event.key === 'Enter' && !event.isComposing && flush()}
    oncompositionstart={() => {
      composing = true;
      cancel();
    }}
    oncompositionend={() => {
      composing = false;
      schedule();
    }}
    class="w-full {FIELD} py-1.5 pr-8 pl-9 placeholder:text-muted"
  />
  {#if value}
    <button
      type="button"
      onclick={clear}
      aria-label={i18n.t('Clear search')}
      class="absolute top-1/2 right-0 flex h-11 w-8 -translate-y-1/2 items-center justify-center text-muted transition-colors hover:text-ink focus-visible:outline-2 focus-visible:outline-focus sm:h-8"
    >
      &times;
    </button>
  {/if}
</div>
