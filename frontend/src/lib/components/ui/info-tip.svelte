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

<!-- Sized to hold its content outright, so on any ordinary phone the note is
     read without scrolling. The cap is only a floor under that: on the
     shortest screens this is taller than the viewport, and text scrolled to is
     better than text hanging off the top where it cannot be reached at all.
     whitespace-normal because a column header sets nowrap to keep its label on
     one line, and the panel would otherwise inherit it and never wrap. -->
<div
  id={panelId}
  popover="auto"
  class="m-auto max-h-[calc(100dvh-2rem)] w-[min(30rem,calc(100vw-2rem))] overflow-y-auto overscroll-contain rounded-xl whitespace-normal border border-zinc-700 bg-zinc-900 p-5 text-left sm:p-6 text-sm leading-relaxed font-normal text-zinc-300 shadow-2xl shadow-zinc-950/70 backdrop:bg-zinc-950/70"
>
  {@render children()}
</div>
