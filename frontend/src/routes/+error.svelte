<script lang="ts">
  import { useI18n } from '#lib/i18n/index.js';
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import Button from '#lib/components/ui/button.svelte';
  import { TEXT } from '#lib/constants/typography.js';

  const i18n = useI18n();

  const copy = $derived.by(() => {
    if (page.status === 404) {
      return {
        label: i18n.t('Page not found'),
        title: i18n.t('This page has left the platform.'),
        description: '',
      };
    }
    return {
      label: i18n.t('Something went wrong'),
      title: i18n.t('We couldn’t load this page.'),
      description: i18n.t('Please try again later.'),
    };
  });
</script>

<svelte:head>
  <title>{page.status} · {copy.label} | OpenStreetlifting</title>
  <meta name="robots" content="noindex, follow" />
</svelte:head>

<div
  class="mx-auto flex min-h-[65vh] max-w-reading flex-col items-center justify-center px-6 py-12 text-center sm:py-16"
>
  <img
    src="/logo_plate_full.svg"
    alt=""
    aria-hidden="true"
    width="160"
    height="160"
    class="mb-8 h-32 w-32 sm:h-40 sm:w-40"
  />
  <p class="mb-3 text-sm font-medium text-secondary">{page.status} · {copy.label}</p>
  <h1 class="{TEXT.title} text-balance text-ink">{copy.title}</h1>
  {#if copy.description}
    <p class="mt-3 max-w-sm text-sm leading-relaxed text-pretty text-secondary">
      {copy.description}
    </p>
  {/if}
  <div class="mt-8 flex w-full max-w-sm flex-col gap-3 sm:w-auto sm:flex-row">
    <Button href={i18n.href(resolve(''))} class="min-h-11 text-sm"
      >{i18n.t('Back to rankings')}</Button
    >

    <Button href={i18n.href(resolve('contact'))} variant="outline" class="min-h-11 text-sm text-ink"
      >{i18n.t('Get in touch')}</Button
    >
  </div>
</div>
