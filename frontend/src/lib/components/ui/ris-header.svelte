<script lang="ts">
  import { page } from '$app/state';

  let { published = false }: { published?: boolean } = $props();

  const edition = $derived(published ? null : page.data.risEdition);
</script>

<span class="whitespace-nowrap">{edition ? `RIS ${edition}` : 'RIS'}</span>
