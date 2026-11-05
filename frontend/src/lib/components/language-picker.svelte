<script lang="ts">
  import Select from '#lib/components/ui/select.svelte';
  import { page } from '$app/state';
  import { useI18n } from '#lib/i18n/index.js';
  import { LOCALES, LANGUAGE_NAMES, isLocale } from '#lib/i18n/locale.js';
  import { localeHref } from '#lib/i18n/paths.js';

  const i18n = useI18n();
</script>

<label for="site-language" class="sr-only">{i18n.t('Language')}</label>
<Select
  id="site-language"
  value={i18n.locale}
  onchange={(event) => {
    const locale = event.currentTarget.value;
    const url = page.shallow?.url ?? page.url;
    if (isLocale(locale))
      window.location.assign(localeHref(`${url.pathname}${url.search}${url.hash}`, locale));
  }}
  class="w-auto px-2"
>
  {#each LOCALES as locale (locale)}
    <option value={locale} lang={locale}>{LANGUAGE_NAMES[locale]}</option>
  {/each}
</Select>
