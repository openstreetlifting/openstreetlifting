export const LOCALES = ['en', 'fr', 'de', 'it', 'pl'] as const;
export type Locale = (typeof LOCALES)[number];
export const DEFAULT_LOCALE: Locale = 'en';
export const LANGUAGE_NAMES: Record<Locale, string> = {
  en: 'English',
  fr: 'Français',
  de: 'Deutsch',
  it: 'Italiano',
  pl: 'Polski',
};
export const OG_LOCALES: Record<Locale, string> = {
  en: 'en_GB',
  fr: 'fr_FR',
  de: 'de_DE',
  it: 'it_IT',
  pl: 'pl_PL',
};

export function isLocale(value: unknown): value is Locale {
  return typeof value === 'string' && (LOCALES as readonly string[]).includes(value);
}

/** Negotiate regional tags and quality weights for the unprefixed home page. */
export function negotiateLocale(acceptLanguage: string | null): Locale {
  const preferences = (acceptLanguage ?? '')
    .split(',')
    .map((entry) => {
      const [tag, ...parameters] = entry.trim().toLowerCase().split(';');
      const weight = parameters.find((parameter) => parameter.trim().startsWith('q='));
      const quality = weight === undefined ? 1 : Number(weight.trim().slice(2));
      return { locale: tag.split('-')[0], quality };
    })
    .filter(({ quality }) => Number.isFinite(quality) && quality > 0 && quality <= 1)
    .sort((a, b) => b.quality - a.quality);
  const preferred = preferences.find(({ locale }) => isLocale(locale))?.locale;
  return isLocale(preferred) ? preferred : DEFAULT_LOCALE;
}
