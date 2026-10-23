import { afterEach, expect, it, vi } from 'vitest';
import { apiClient, apiData, ApiError } from './client';
import { athletesService } from './services/athletes';
import type { components } from '#lib/api/schema.generated.js';

vi.mock('#lib/server/config.js', () => ({ config: { apiUrl: 'http://backend.test' } }));

afterEach(() => vi.unstubAllGlobals());

it('encodes athlete slugs and preserves null values and empty requested sections', async () => {
  const athlete = {
    athlete_id: 'athlete-1',
    first_name: 'Anne',
    last_name: 'Léa',
    native_name: null,
    instagram_handle: null,
    slug: 'anne / léa',
    gender: 'F',
    country: null,
    profile_picture_url: null,
    created_at: '2026-01-01T00:00:00',
    competitions: [],
    personal_records: [],
    total_competitions: 0,
    standing: null,
    strength_profile: null,
  } satisfies components['schemas']['AthleteResponse'];
  const fetch = vi.fn().mockResolvedValue(Response.json(athlete));
  vi.stubGlobal('fetch', fetch);

  expect(await athletesService.getBySlug(athlete.slug)).toEqual(athlete);
  const request = fetch.mock.calls[0][0] as Request;
  const url = new URL(request.url);
  expect(url.pathname).toBe(`/api/v1/athletes/${encodeURIComponent(athlete.slug)}`);
  expect(url.searchParams.get('include')).toBe('competitions,records,standing,strength_profile');
});

it('keeps unrequested sections absent from athlete listings', async () => {
  const athlete = {
    athlete_id: 'athlete-1',
    first_name: 'Anne',
    last_name: 'Léa',
    native_name: null,
    instagram_handle: null,
    slug: 'anne-lea',
    gender: 'F',
    country: null,
    profile_picture_url: null,
    created_at: '2026-01-01T00:00:00',
  } satisfies components['schemas']['AthleteResponse'];
  const response = {
    data: [athlete],
    pagination: { page: 1, page_size: 50, total_items: 1, total_pages: 1 },
  };
  vi.stubGlobal(
    'fetch',
    vi
      .fn()
      .mockResolvedValueOnce(Response.json(response))
      .mockResolvedValueOnce(Response.json(athlete))
  );

  expect(await athletesService.getAll({ page: 1 })).toEqual(response);
  await expect(athletesService.getBySlug(athlete.slug)).rejects.toThrow(
    'Requested athlete sections are missing'
  );
});

it('preserves backend error messages and HTTP status', async () => {
  vi.stubGlobal(
    'fetch',
    vi.fn().mockResolvedValue(Response.json({ error: 'Athlete not found' }, { status: 404 }))
  );

  await expect(athletesService.getBySlug('unknown')).rejects.toMatchObject({
    name: 'ApiError',
    status: 404,
    message: 'Athlete not found',
  });
});

it('handles non-JSON errors from an upstream proxy', async () => {
  vi.stubGlobal(
    'fetch',
    vi
      .fn()
      .mockResolvedValue(
        new Response('Unavailable', { status: 503, statusText: 'Service Unavailable' })
      )
  );

  await expect(apiData(apiClient.GET('/api/v1/rankings'))).rejects.toEqual(
    new ApiError(503, 'Service Unavailable', 'HTTP 503: Service Unavailable')
  );
});
