import { afterEach, expect, it, vi } from 'vitest';
import { apiClient, apiData, ApiError } from './client';
import { athletesService } from './services/athletes';
import type { components } from '#lib/api/schema.generated.js';

vi.mock('#lib/server/config.js', () => ({ config: { apiUrl: 'http://backend.test' } }));

afterEach(() => vi.unstubAllGlobals());

it('encodes athlete slugs and normalizes omitted detail collections', async () => {
  const athlete = {
    athlete_id: 'athlete-1',
    first_name: 'Anne',
    last_name: 'Léa',
    slug: 'anne / léa',
    gender: 'F',
    country: null,
    profile_picture_url: null,
    created_at: '2026-01-01T00:00:00',
  } satisfies components['schemas']['AthleteResponse'];
  const fetch = vi.fn().mockResolvedValue(Response.json(athlete));
  vi.stubGlobal('fetch', fetch);

  expect(await athletesService.getBySlug(athlete.slug)).toEqual({
    ...athlete,
    competitions: [],
    personal_records: [],
    total_competitions: 0,
  });
  const request = fetch.mock.calls[0][0] as Request;
  const url = new URL(request.url);
  expect(url.pathname).toBe(`/api/v1/athletes/${encodeURIComponent(athlete.slug)}`);
  expect(url.searchParams.get('include')).toBe('competitions,records,standing,strength_profile');
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
