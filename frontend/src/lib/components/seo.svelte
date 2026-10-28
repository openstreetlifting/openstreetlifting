<script lang="ts">
  import { LOCALES, OG_LOCALES } from '#lib/i18n/locale.js';
  import { useI18n } from '#lib/i18n/index.js';
  import {
    INDEXABLE,
    OG_IMAGE,
    SITE_NAME,
    jsonLdScript,
    pageTitle,
    localizedUrl,
    type JsonLd,
  } from '#lib/seo/index.js';

  const i18n = useI18n();

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

  const localizedCanonical = $derived(localizedUrl(canonical, i18n.locale));
  const fullTitle = $derived(pageTitle(title));
  const indexable = $derived(INDEXABLE && !noindex);
  const blocks = $derived(jsonLd.map(jsonLdScript));
</script>

<svelte:head>
  <title>{fullTitle}</title>
  <meta name="description" content={description} />
  <link rel="canonical" href={localizedCanonical} />
  {#each LOCALES as locale (locale)}
    <link rel="alternate" hreflang={locale} href={localizedUrl(canonical, locale)} />
  {/each}
  <link rel="alternate" hreflang="x-default" href={localizedUrl(canonical, 'en')} />
  {#if !indexable}
    <meta name="robots" content="noindex, follow" />
  {/if}

  <meta property="og:type" content={type} />
  <meta property="og:site_name" content={SITE_NAME} />
  <meta property="og:locale" content={OG_LOCALES[i18n.locale]} />
  <meta property="og:title" content={fullTitle} />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={localizedCanonical} />
  <meta property="og:image" content={OG_IMAGE.url} />
  <meta property="og:image:width" content={String(OG_IMAGE.width)} />
  <meta property="og:image:height" content={String(OG_IMAGE.height)} />

  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:title" content={fullTitle} />
  <meta name="twitter:description" content={description} />
  <meta name="twitter:image" content={OG_IMAGE.url} />

  {#each blocks as block, index (index)}
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html block}
  {/each}
</svelte:head>
