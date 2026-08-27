<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    label,
    children,
  }: {
    /** What the button announces. The glyph on its own says nothing. */
    label: string;
    children: Snippet;
  } = $props();

  // A popover rather than a positioned div, because every table scrolls inside
  // its own container and anything positioned normally is clipped by it. The
  // top layer escapes that, and brings Escape and click-outside dismissal with
  // no script of ours.
  const panelId = $props.id();
</script>

<button
  type="button"
  popovertarget={panelId}
  aria-label={label}
  class="ml-1 inline-flex h-3.5 w-3.5 shrink-0 cursor-pointer items-center justify-center rounded-full border border-zinc-600 align-middle text-[0.6rem] leading-none font-medium text-zinc-400 transition-colors hover:border-zinc-400 hover:text-zinc-200"
>
  ?
</button>

<!-- Sized to hold its content outright. A note the reader has to scroll is
     worse than no note, so nothing here caps the height.
     whitespace-normal because a column header sets nowrap to keep its label on
     one line, and the panel would otherwise inherit it and never wrap. -->
<div
  id={panelId}
  popover="auto"
  class="m-auto w-[min(30rem,calc(100vw-2rem))] rounded-xl whitespace-normal border border-zinc-700 bg-zinc-900 p-6 text-left text-sm leading-relaxed font-normal text-zinc-300 shadow-2xl shadow-zinc-950/70 backdrop:bg-zinc-950/70"
>
  {@render children()}
</div>
