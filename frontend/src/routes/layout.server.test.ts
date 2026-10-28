import { expect, it } from 'vitest';
import { load } from './+layout.server.js';

it.each([
  [undefined, 'kg'],
  ['kg', 'kg'],
  ['lb', 'lb'],
  ['invalid', 'kg'],
] as const)(
  'renders the saved weight preference %s independently from the URL language',
  async (saved, expected) => {
    const data = await load({
      url: new URL('https://openstreetlifting.org/fr/ris'),
      cookies: { get: () => saved },
    } as unknown as Parameters<typeof load>[0]);
    expect(data).toEqual({ locale: 'fr', weightUnit: expected });
  }
);
