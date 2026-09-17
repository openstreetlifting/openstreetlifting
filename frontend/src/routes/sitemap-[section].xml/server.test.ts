import { beforeEach, expect, it, vi } from 'vitest';
import { athletesService, competitionsService, rankingsService } from '$lib/server/api';
import { GET as index } from '../sitemap.xml/+server';
import { GET } from './+server';

vi.mock('$lib/server/api', () => ({
  athletesService: { getAll: vi.fn() },
  competitionsService: { getAll: vi.fn() },
  rankingsService: { getRankingCountries: vi.fn() },
}));

const finalrep = { federation_id: 'f', name: 'FinalRep', abbreviation: null, country: null };
const page = <T>(data: T[]) => ({
  data,
  pagination: { page: 1, page_size: 100, total_items: data.length, total_pages: 1 },
});

const request = (section: string) => ({ params: { section } }) as Parameters<typeof GET>[0];

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(competitionsService.getAll).mockResolvedValue(
    page([
      { slug: 'worlds-2025', status: 'completed', country: 'DE', federation: finalrep },
      { slug: 'draft-meet', status: 'draft', country: 'DE', federation: finalrep },
    ]) as unknown as Awaited<ReturnType<typeof competitionsService.getAll>>
  );
  vi.mocked(athletesService.getAll).mockResolvedValue(
    page([{ slug: 'aubin-chevillard', country: 'FR' }]) as unknown as Awaited<
      ReturnType<typeof athletesService.getAll>
    >
  );
  vi.mocked(rankingsService.getRankingCountries).mockResolvedValue(['FR']);
});

it('lists one sitemap per section', async () => {
  const body = await (await index({} as Parameters<typeof index>[0])).text();

  expect(body).toContain('<sitemapindex');
  expect(body.match(/<loc>[^<]+<\/loc>/g)).toEqual([
    '<loc>https://openstreetlifting.org/sitemap-pages.xml</loc>',
    '<loc>https://openstreetlifting.org/sitemap-competitions.xml</loc>',
    '<loc>https://openstreetlifting.org/sitemap-athletes.xml</loc>',
  ]);
});

it('splits pages, published competitions and athletes into their own sitemaps', async () => {
  const locs = async (section: string) =>
    (await (await GET(request(section))).text()).match(/<loc>[^<]+<\/loc>/g);

  expect(await locs('pages')).toEqual(
    expect.arrayContaining([
      '<loc>https://openstreetlifting.org/federations/finalrep</loc>',
      '<loc>https://openstreetlifting.org/countries/de</loc>',
      '<loc>https://openstreetlifting.org/countries/fr</loc>',
    ])
  );
  expect(await locs('competitions')).toEqual([
    '<loc>https://openstreetlifting.org/competitions/worlds-2025</loc>',
  ]);
  expect(await locs('athletes')).toEqual([
    '<loc>https://openstreetlifting.org/athletes/aubin-chevillard</loc>',
  ]);
});

it('answers an unknown section with a 404', async () => {
  await expect(GET(request('photos'))).rejects.toMatchObject({ status: 404 });
});
