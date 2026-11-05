import { expect, it } from 'vitest';
import { readRisScenario, writeRisScenario } from './ris-scenario.js';

it('round trips mixed input units, category and target while preserving unrelated URL state', () => {
  const url = new URL('https://openstreetlifting.org/fr/ris?source=shared#explore-heading');
  const scenario = {
    sex: 'women',
    fields: {
      bodyweight: { text: '176.37', unit: 'lb' },
      total: { text: '500,25', unit: 'kg' },
    },
    target: '110',
  } as const;
  const shared = writeRisScenario(url, scenario);
  expect(readRisScenario(shared.searchParams)).toEqual(scenario);
  expect(shared.pathname).toBe('/fr/ris');
  expect(shared.hash).toBe('#explore-heading');
  expect(shared.searchParams.get('source')).toBe('shared');
  expect(shared.searchParams.has('total_unit')).toBe(false);
});

it('removes cleared inputs and their units from a shared scenario', () => {
  const url = new URL(
    'https://openstreetlifting.org/en/ris?sex=women&bodyweight=180&bodyweight_unit=lb&target=110'
  );
  const empty = readRisScenario(new URLSearchParams());
  expect(writeRisScenario(url, empty).search).toBe('');
});

it('falls back to supported units and categories without rejecting editable input', () => {
  const scenario = readRisScenario(
    new URLSearchParams('sex=unknown&bodyweight=abc&bodyweight_unit=oz')
  );
  expect(scenario.sex).toBe('men');
  expect(scenario.fields.bodyweight).toEqual({ text: 'abc', unit: 'kg' });
});
