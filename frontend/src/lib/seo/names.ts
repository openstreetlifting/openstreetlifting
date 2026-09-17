import type { AthleteStanding } from '$lib/types/athlete';
import type { Federation } from '$lib/types/competition';
import { countryName } from '$lib/utils/format';

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

export function competitionTitle(seoName: string, published: boolean): string {
  const sport = mentions(seoName, SPORT) ? '' : ` ${SPORT}`;
  return `${seoName}${sport} ${published ? 'results' : 'competition'}`;
}

export function athleteTitle(name: string, country: string | null): string {
  return country
    ? `${name}, ${countryName(country)} - ${SPORT} results`
    : `${name} - ${SPORT} results`;
}

export function risStanding(standing: AthleteStanding | null | undefined): string {
  const ris = standing?.ris;
  if (!ris) return '';

  const places = [
    `#${ris.country.place} of ${ris.country.field.toLocaleString('en')} in ${countryName(ris.country.code)}`,
    `#${ris.global.place} of ${ris.global.field.toLocaleString('en')} worldwide`,
  ];
  return `Ranked ${places.join(' and ')} by RIS.`;
}
