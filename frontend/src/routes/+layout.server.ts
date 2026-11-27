import type { LayoutServerLoad } from './$types';
import { pathLocale } from '#lib/i18n/paths.js';
import { DEFAULT_LOCALE } from '#lib/i18n/locale.js';
import { currentRisFormula } from '#lib/server/ris-formula.js';

export const load: LayoutServerLoad = async ({ url }) => ({
  locale: pathLocale(url.pathname) ?? DEFAULT_LOCALE,
  risEdition: (await currentRisFormula())?.year ?? null,
});
