import { beforeEach, expect, it, vi } from 'vitest';
import { competitionsService } from '#lib/server/api/index.js';
import { currentRisFormula } from '#lib/server/ris-formula.js';
import { load } from './+page.server';
import { ApiError } from '#lib/server/api/client.js';

vi.mock('#lib/server/api/index.js', () => ({
  competitionsService: { getById: vi.fn() },
}));
vi.mock('#lib/server/ris-formula.js', () => ({ currentRisFormula: vi.fn() }));

const request = () => ({ params: { slug: 'euros' } }) as Parameters<typeof load>[0];
type Competition = Awaited<ReturnType<typeof competitionsService.getById>>;
const formula = { year: 2026 } as Awaited<ReturnType<typeof currentRisFormula>>;

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(currentRisFormula).mockResolvedValue(formula);
});

it('keeps a confirmed missing competition as a 404', async () => {
  vi.mocked(competitionsService.getById).mockRejectedValue(
    new ApiError(404, 'Not Found', 'Missing')
  );
  await expect(load(request())).rejects.toMatchObject({ status: 404 });
});

it.each([
  new Error('Network unavailable'),
  new ApiError(500, 'Internal Server Error', 'Unavailable'),
])('reports temporary lookup failures as unavailable rather than missing', async (failure) => {
  vi.mocked(competitionsService.getById).mockRejectedValue(failure);
  await expect(load(request())).rejects.toMatchObject({ status: 503 });
});

it('loads the RIS formula only for a competition with published results', async () => {
  const published = { categories: [{ participants: [] }] } as unknown as Competition;
  vi.mocked(competitionsService.getById).mockResolvedValue(published);
  expect(await load(request())).toEqual({ competition: published, formula });

  const upcoming = { categories: [] } as unknown as Competition;
  vi.mocked(competitionsService.getById).mockResolvedValue(upcoming);
  expect(await load(request())).toEqual({ competition: upcoming, formula: null });
  expect(currentRisFormula).toHaveBeenCalledTimes(1);
});
