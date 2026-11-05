import { expect, it } from 'vitest';
import { render } from 'svelte/server';
import RisCurve from './ris-curve.svelte';

it('renders finite scatter points and the RIS benchmark before hydration', () => {
  const { body } = render(RisCurve, {
    props: {
      constants: {
        a: 335.5625,
        k: 556.1103380806655,
        b: 0.10289374204365953,
        v: 76.74125992622565,
        q: 0.4973075488457353,
      },
      label: 'RIS distribution',
      points: [
        {
          participant_id: 'performance',
          athlete_slug: 'alex-martin',
          athlete_name: 'Alex Martin',
          competition_slug: 'euros',
          competition_name: 'Euros',
          competition_date: '2026-01-01',
          bodyweight: 80,
          total: 500,
        },
      ],
    },
  });
  expect(body).toMatch(/<circle[^>]*cx="[\d.-]+"/);
  expect(body).toMatch(/<path[^>]*d="M[\d.-]+/);
  expect(body).not.toMatch(/(?:cx|cy|d)="[^"]*(?:NaN|undefined)/);
});
