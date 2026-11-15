import { beforeEach, expect, it, vi } from 'vitest';
import { rankingsService } from '#lib/server/api/index.js';
import { load } from './+layout.server';

vi.mock('#lib/server/api/index.js', () => ({
  rankingsService: { getFacets: vi.fn() },
}));

const options = { classes: ['-80kg'], years: [], countries: [], federations: [], series: [] };

beforeEach(() => {
  vi.resetAllMocks();
  vi.mocked(rankingsService.getFacets).mockResolvedValue(options);
});

function facetsFor(search: string) {
  return load({ url: new URL(`http://localhost/${search}`) } as Parameters<typeof load>[0]);
}

it('asks for the options of the metric being ranked', async () => {
  await facetsFor('?movement=pullup&country=FR&q=martin&page=3');
  expect(rankingsService.getFacets).toHaveBeenCalledWith(
    expect.objectContaining({ movement: 'pullup', country: 'FR' })
  );
  expect(rankingsService.getFacets).not.toHaveBeenCalledWith(
    expect.objectContaining({ q: expect.anything() })
  );
});

it('defaults to the RIS options', async () => {
  await facetsFor('');
  expect(rankingsService.getFacets).toHaveBeenCalledWith(
    expect.objectContaining({ movement: 'ris' })
  );
});
