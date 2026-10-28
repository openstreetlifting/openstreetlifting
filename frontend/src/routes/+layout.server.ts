import type { LayoutServerLoad } from './$types';
import { pathLocale } from '#lib/i18n/paths.js';
import { DEFAULT_LOCALE } from '#lib/i18n/locale.js';

export const load: LayoutServerLoad = ({ url }) => ({
  locale: pathLocale(url.pathname) ?? DEFAULT_LOCALE,
});
