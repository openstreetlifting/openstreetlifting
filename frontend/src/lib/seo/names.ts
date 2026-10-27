import { I18n } from '#lib/i18n/index.js';
import type { Locale } from '#lib/i18n/locale.js';
import type { AthleteStanding } from '#lib/types/athlete.js';
import type { Federation } from '#lib/types/competition.js';
import { countryName } from '#lib/utils/format.js';

const SPORT = 'Streetlifting';

function mentions(text: string, part: string): boolean {
  return text.toLowerCase().includes(part.toLowerCase());
}

export function competitionSeoName(competition: {
  name: string;
  start_date: string | null;
  federation: Pick<Federation, 'name' | 'abbreviation'>;
}): string {
  const { name, federation } = competition;
  const year = competition.start_date?.slice(0, 4) ?? '';
  const named =
    mentions(name, federation.name) ||
    (federation.abbreviation !== null && mentions(name, federation.abbreviation));
  const prefix = named ? '' : `${federation.abbreviation || federation.name} `;
  const suffix = year && !name.includes(year) ? ` ${year}` : '';

  return `${prefix}${name}${suffix}`;
}

export function competitionTitle(
  seoName: string,
  published: boolean,
  locale: Locale = 'en'
): string {
  const sport = mentions(seoName, SPORT) ? '' : ` ${SPORT}`;
  const i18n = new I18n(() => locale);
  return published
    ? i18n.t('{competition} results', { competition: `${seoName}${sport}` })
    : i18n.t('{competition} competition', { competition: `${seoName}${sport}` });
}

export function athleteTitle(name: string, country: string | null, locale: Locale = 'en'): string {
  const i18n = new I18n(() => locale);
  return i18n.t('{athlete} - Streetlifting results', {
    athlete: country ? `${name}, ${countryName(country, locale)}` : name,
  });
}

export function risStanding(
  standing: Pick<AthleteStanding, 'ris'> | null | undefined,
  locale: Locale = 'en'
): string {
  const ris = standing?.ris;
  if (!ris) return '';

  const i18n = new I18n(() => locale);
  const places = [
    ...(ris.country
      ? [
          i18n.t('#{place} of {field} in {country}', {
            place: ris.country.place,
            field: i18n.number(ris.country.field, 0),
            country: countryName(ris.country.code, locale),
          }),
        ]
      : []),
    i18n.t('#{place} of {field} worldwide', {
      place: ris.global.place,
      field: i18n.number(ris.global.field, 0),
    }),
  ];
  return i18n.t('Ranked {places} by RIS.', { places: places.join(i18n.t(' and ')) });
}
