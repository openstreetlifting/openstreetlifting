import { logger } from '#lib/server/logger.js';
import { error } from '@sveltejs/kit';
import { archiveIndex } from '#lib/server/archive.js';
import { absolute, SITE_DESCRIPTION, SITE_NAME } from '#lib/seo/index.js';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async () => {
  let counts;
  try {
    ({ counts } = await archiveIndex());
  } catch (err) {
    logger.error({ err }, 'LLM index generation failed');
    error(503, 'llms.txt unavailable');
  }

  const body = `# ${SITE_NAME}

> ${SITE_DESCRIPTION} The archive holds results from ${counts.competitions} competitions and ${counts.athletes} athletes, across ${counts.federations} federations and ${counts.countries} countries.

Results cover the muscle-up, pull-up, dips and squat attempts contested at each competition, with the lifter's bodyweight class and their RIS (Relative Index for Streetlifting) score where it can be computed.

## Pages

- [Rankings](${absolute('/', 'en')}): athletes ranked by RIS, total or a single lift, filterable by country, federation, year, sex and class
- [Competitions](${absolute('/competitions', 'en')}): every competition in the archive with full results, plus upcoming meets
- [Federations](${absolute('/federations', 'en')}): each federation's competitions and athletes, at /federations/<slug>
- [Countries](${absolute('/countries', 'en')}): competitions held in a country and its athletes, at /countries/<iso code>
- [RIS](${absolute('/ris', 'en')}): how the Relative Index for Streetlifting compares totals across bodyweights, with a calculator

Pages use /en, /fr, /de, /it, or /pl prefixes. Athlete pages live at /en/athletes/<slug> and competition pages at /en/competitions/<slug>.

## Data

- [API reference](https://api.openstreetlifting.org/swagger-ui/): public JSON API behind the site
- [Documentation](https://docs.openstreetlifting.org/): how the archive is built, the data format and how to contribute
- [Licensing](https://docs.openstreetlifting.org/LICENSING.html): terms for reusing the data and the code
- [Source and data files](https://github.com/openstreetlifting/openstreetlifting)
`;

  return new Response(body, {
    headers: {
      'content-type': 'text/plain; charset=utf-8',
      'cache-control': 'public, max-age=3600',
    },
  });
};
