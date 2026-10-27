export const LOCALES = ['en', 'fr'] as const;
export type Locale = (typeof LOCALES)[number];
export const DEFAULT_LOCALE: Locale = 'en';
export const LOCALE_COOKIE = 'osl-language';

export function isLocale(value: unknown): value is Locale {
  return value === 'en' || value === 'fr';
}

/** A saved choice wins; otherwise negotiate regional tags and quality weights. */
export function negotiateLocale(cookie: string | undefined, acceptLanguage: string | null): Locale {
  if (isLocale(cookie)) return cookie;
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
