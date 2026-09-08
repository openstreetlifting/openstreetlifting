import { afterEach, expect, it, vi } from 'vitest';
import { athletesService } from '$lib/server/api';
import { ApiError } from '$lib/server/api/client';
import { load } from './+page.server';

vi.mock('$lib/server/api', () => ({ athletesService: { getBySlug: vi.fn() } }));
vi.mock('$lib/server/config', () => ({ config: { apiUrl: 'http://localhost' } }));

afterEach(() => vi.restoreAllMocks());

const request = { params: { slug: 'alex-martin' } } as Parameters<typeof load>[0];

it('keeps a missing athlete as a 404', async () => {
  vi.mocked(athletesService.getBySlug).mockRejectedValue(new ApiError(404, 'Not Found', 'Missing'));
  await expect(load(request)).rejects.toMatchObject({
    status: 404,
    body: { message: 'Athlete not found' },
  });
});

it.each([new ApiError(500, 'Internal Server Error', 'Offline'), new TypeError('fetch failed')])(
  'treats backend failures as temporary unavailability: %s',
  async (failure) => {
    vi.spyOn(console, 'error').mockImplementation(() => {});
    vi.mocked(athletesService.getBySlug).mockRejectedValue(failure);
    await expect(load(request)).rejects.toMatchObject({
      status: 503,
      body: { message: 'Athlete details are unavailable right now' },
    });
  }
);
