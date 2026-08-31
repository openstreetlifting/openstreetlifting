<script lang="ts">
  import { INDEXABLE, OG_IMAGE, SITE_NAME, jsonLdScript, pageTitle, type JsonLd } from '$lib/seo';

  interface Props {
    title: string;
    description: string;
    canonical: string;
    noindex?: boolean;
    type?: 'website' | 'article' | 'profile';
    jsonLd?: JsonLd[];
  }

  let {
    title,
    description,
    canonical,
    noindex = false,
    type = 'website',
    jsonLd = [],
  }: Props = $props();

  const fullTitle = $derived(pageTitle(title));
  const indexable = $derived(INDEXABLE && !noindex);
  const blocks = $derived(jsonLd.map(jsonLdScript));
</script>

<svelte:head>
  <title>{fullTitle}</title>
  <meta name="description" content={description} />
  <link rel="canonical" href={canonical} />
  {#if !indexable}
    <meta name="robots" content="noindex, follow" />
  {/if}

  <meta property="og:type" content={type} />
  <meta property="og:site_name" content={SITE_NAME} />
  <meta property="og:locale" content="en" />
  <meta property="og:title" content={fullTitle} />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={canonical} />
  <meta property="og:image" content={OG_IMAGE.url} />
  <meta property="og:image:width" content={String(OG_IMAGE.width)} />
  <meta property="og:image:height" content={String(OG_IMAGE.height)} />

  <meta name="twitter:card" content="summary" />
  <meta name="twitter:title" content={fullTitle} />
  <meta name="twitter:description" content={description} />
  <meta name="twitter:image" content={OG_IMAGE.url} />

  {#each blocks as block, index (index)}
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html block}
  {/each}
</svelte:head>
