<script lang="ts">
  import Select from '#lib/components/ui/select.svelte';
  import '@fontsource-variable/geist-mono/wght.css';
  import '@fontsource-variable/fira-code';
  import '@fontsource/ibm-plex-mono/400.css';
  import '@fontsource/ibm-plex-mono/500.css';
  import '@fontsource/ibm-plex-mono/600.css';
  import '@fontsource/ibm-plex-mono/700.css';

  const fonts = [
    { label: 'JetBrains Mono', family: 'JetBrains Mono Variable' },
    { label: 'Geist Mono', family: 'Geist Mono Variable' },
    { label: 'IBM Plex Mono', family: 'IBM Plex Mono' },
    { label: 'Fira Code', family: 'Fira Code Variable' },
  ];
  let selected = $state(fonts[0].family);

  $effect(() => {
    const root = document.documentElement;
    root.style.setProperty('--font-mono', `'${selected}', ui-monospace, monospace`);
    return () => root.style.removeProperty('--font-mono');
  });
</script>

<!-- Development tooling only. This component and its extra fonts are excluded from production. -->
<details
  class="fixed right-3 bottom-3 z-30 max-w-[calc(100vw-1.5rem)] border border-stroke-strong bg-surface text-ink shadow-lg shadow-overlay/30"
>
  <summary
    class="min-h-11 cursor-pointer px-3 py-3 font-mono text-[0.625rem] focus-visible:outline-2 focus-visible:outline-focus"
    >Typography preview</summary
  >
  <div class="border-t border-stroke p-3">
    <label class="grid gap-2 font-sans text-xs text-secondary">
      Interface and numbers
      <Select
        bind:value={selected}
        class="min-h-11 border border-stroke-strong bg-canvas px-2 font-sans text-xs text-ink focus-visible:outline-2 focus-visible:outline-focus"
      >
        {#each fonts as font (font.family)}
          <option value={font.family}>{font.label}</option>
        {/each}
      </Select>
    </label>
    <p class="mt-2 font-sans text-xs text-muted">Athlete names keep the same sans serif.</p>
  </div>
</details>
