import type { AthleteDetail } from '$lib/types/athlete';
import type { CompetitionDetail } from '$lib/types/competition';
import { countryName, formatAthleteName } from '$lib/utils';
import { absolute, SITE_DESCRIPTION, SITE_NAME, SITE_URL } from './site';

export type JsonLd = Record<string, unknown>;

// Every "<" is escaped, so a value carrying a closing tag cannot end the script
// element early and turn the payload into markup.
export function jsonLdScript(entry: JsonLd): string {
  const payload = JSON.stringify(entry).replace(/</g, '\\u003c');
  return `<script type="application/ld+json">${payload}</script>`;
}

const SOCIAL_PROFILES = [
  'https://github.com/openstreetlifting',
  'https://www.instagram.com/openstreetlifting',
];

function organization(): JsonLd {
  return {
    '@type': 'Organization',
    '@id': `${SITE_URL}/#organization`,
    name: SITE_NAME,
    url: SITE_URL,
    logo: absolute('/logowidth.png'),
    description: SITE_DESCRIPTION,
    sameAs: SOCIAL_PROFILES,
  };
}

export function websiteLd(): JsonLd {
  return {
    '@context': 'https://schema.org',
    '@type': 'WebSite',
    '@id': `${SITE_URL}/#website`,
    name: SITE_NAME,
    url: SITE_URL,
    description: SITE_DESCRIPTION,
    inLanguage: 'en',
    publisher: organization(),
    potentialAction: {
      '@type': 'SearchAction',
      target: {
        '@type': 'EntryPoint',
        urlTemplate: `${SITE_URL}/?q={search_term_string}`,
      },
      'query-input': 'required name=search_term_string',
    },
  };
}

export function breadcrumbLd(trail: { name: string; path?: string }[]): JsonLd {
  return {
    '@context': 'https://schema.org',
    '@type': 'BreadcrumbList',
    itemListElement: trail.map((crumb, index) => ({
      '@type': 'ListItem',
      position: index + 1,
      name: crumb.name,
      ...(crumb.path ? { item: absolute(crumb.path) } : {}),
    })),
  };
}

const EVENT_STATUS: Record<string, string> = {
  cancelled: 'https://schema.org/EventCancelled',
  upcoming: 'https://schema.org/EventScheduled',
  live: 'https://schema.org/EventScheduled',
  completed: 'https://schema.org/EventScheduled',
};

export function competitionLd(competition: CompetitionDetail, description: string): JsonLd {
  const place = [competition.city, competition.region].filter(Boolean).join(', ');

  return {
    '@context': 'https://schema.org',
    '@type': 'SportsEvent',
    name: competition.name,
    url: absolute(`/competitions/${competition.slug}`),
    description,
    ...(competition.start_date ? { startDate: competition.start_date } : {}),
    ...(competition.end_date ? { endDate: competition.end_date } : {}),
    eventStatus: EVENT_STATUS[competition.status] ?? 'https://schema.org/EventScheduled',
    eventAttendanceMode: 'https://schema.org/OfflineEventAttendanceMode',
    ...(competition.country || place
      ? {
          location: {
            '@type': 'Place',
            name: place || countryName(competition.country ?? ''),
            address: {
              '@type': 'PostalAddress',
              ...(competition.city ? { addressLocality: competition.city } : {}),
              ...(competition.region ? { addressRegion: competition.region } : {}),
              ...(competition.country ? { addressCountry: competition.country } : {}),
            },
          },
        }
      : {}),
    organizer: {
      '@type': 'SportsOrganization',
      name: competition.federation.name,
      ...(competition.federation.abbreviation
        ? { alternateName: competition.federation.abbreviation }
        : {}),
    },
    isAccessibleForFree: true,
    publisher: organization(),
  };
}

const SCHEMA_GENDER: Record<string, string> = {
  M: 'https://schema.org/Male',
  F: 'https://schema.org/Female',
};

export function athleteLd(athlete: AthleteDetail, description: string): JsonLd {
  const url = absolute(`/athletes/${athlete.slug}`);

  return {
    '@context': 'https://schema.org',
    '@type': 'ProfilePage',
    url,
    description,
    mainEntity: {
      '@type': 'Person',
      '@id': `${url}#person`,
      name: formatAthleteName(athlete),
      ...(athlete.native_name ? { alternateName: athlete.native_name } : {}),
      url,
      ...(SCHEMA_GENDER[athlete.gender] ? { gender: SCHEMA_GENDER[athlete.gender] } : {}),
      ...(athlete.country
        ? { nationality: { '@type': 'Country', name: countryName(athlete.country) } }
        : {}),
      ...(athlete.instagram_handle
        ? { sameAs: [`https://www.instagram.com/${athlete.instagram_handle}`] }
        : {}),
      ...(athlete.profile_picture_url ? { image: athlete.profile_picture_url } : {}),
    },
  };
}
