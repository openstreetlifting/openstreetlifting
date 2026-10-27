<script lang="ts">
  import { page } from '$app/state';
  import { resolve } from '$app/paths';
  import { useI18n } from '#lib/i18n/index.js';
  import { FIELD } from '#lib/constants/typography.js';

  const i18n = useI18n();
</script>

<form method="POST" action={resolve('/language')} class="flex items-center gap-2">
  <input
    type="hidden"
    name="returnTo"
    value={`${page.url.pathname}${page.url.search}${page.url.hash}`}
  />
  <label for="site-language" class="sr-only">{i18n.t('Language')}</label>
  <select
    id="site-language"
    name="locale"
    value={i18n.locale}
    onchange={(event) => event.currentTarget.form?.requestSubmit()}
    class="{FIELD} px-2 py-1 text-sm"
  >
    <option value="en" lang="en">English</option>
    <option value="fr" lang="fr">Français</option>
  </select>
</form>
