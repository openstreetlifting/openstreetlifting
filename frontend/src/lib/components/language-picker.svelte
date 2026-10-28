<script lang="ts">
  import { page } from '$app/state';
  import { useI18n } from '#lib/i18n/index.js';
  import { LOCALES, LANGUAGE_NAMES, isLocale } from '#lib/i18n/locale.js';
  import { localeHref } from '#lib/i18n/paths.js';
  import { FIELD } from '#lib/constants/typography.js';

  const i18n = useI18n();
</script>

<label for="site-language" class="sr-only">{i18n.t('Language')}</label>
<select
  id="site-language"
  value={i18n.locale}
  onchange={(event) => {
    const locale = event.currentTarget.value;
    if (isLocale(locale))
      window.location.assign(
        localeHref(`${page.url.pathname}${page.url.search}${page.url.hash}`, locale)
      );
  }}
  class="{FIELD} px-2 py-1 text-sm"
>
  {#each LOCALES as locale (locale)}
    <option value={locale} lang={locale}>{LANGUAGE_NAMES[locale]}</option>
  {/each}
</select>
