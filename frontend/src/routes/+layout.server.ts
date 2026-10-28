import type { LayoutServerLoad } from './$types';
import { pathLocale } from '#lib/i18n/paths.js';
import { DEFAULT_LOCALE } from '#lib/i18n/locale.js';
import { weightUnit, WEIGHT_UNIT_COOKIE } from '#lib/utils/weight-unit.js';

export const load: LayoutServerLoad = ({ url, cookies }) => ({
  locale: pathLocale(url.pathname) ?? DEFAULT_LOCALE,
  weightUnit: weightUnit(cookies.get(WEIGHT_UNIT_COOKIE)),
});
