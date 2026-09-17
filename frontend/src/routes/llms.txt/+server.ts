import { error } from '@sveltejs/kit';
import { archiveIndex } from '$lib/server/archive';
import { absolute, SITE_DESCRIPTION, SITE_NAME } from '$lib/seo';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async () => {
  let counts;
  try {
    ({ counts } = await archiveIndex());
  } catch (err) {
    console.error('Failed to build llms.txt', err);
    error(503, 'llms.txt unavailable');
  }

  const body = `# ${SITE_NAME}

> ${SITE_DESCRIPTION} The archive holds results from ${counts.competitions} competitions and ${counts.athletes} athletes, across ${counts.federations} federations and ${counts.countries} countries.

Results cover the muscle-up, pull-up, dips and squat attempts contested at each competition, with the lifter's bodyweight class and their RIS (Relative Index for Streetlifting) score where it can be computed.

## Pages

- [Rankings](${absolute('/')}): athletes ranked by RIS, total or a single lift, filterable by country, federation, year, sex and class
- [Competitions](${absolute('/competitions')}): every competition in the archive with full results, plus upcoming meets
- [Federations](${absolute('/federations')}): each federation's competitions and athletes, at /federations/<slug>
- [Countries](${absolute('/countries')}): competitions held in a country and its athletes, at /countries/<iso code>
- [RIS](${absolute('/ris')}): how the Relative Index for Streetlifting compares totals across bodyweights, with a calculator

Athlete pages live at /athletes/<slug> and competition pages at /competitions/<slug>.

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
