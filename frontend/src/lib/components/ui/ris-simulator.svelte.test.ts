import { expect, it } from 'vitest';
import { render } from 'vitest-browser-svelte';
import RisSimulator from './ris-simulator.svelte';
import type { RisFormula } from '$lib/types/ris';

const constants = {
  a: 335.5625,
  k: 556.1103380806655,
  b: 0.10289374204365953,
  v: 76.74125992622565,
  q: 0.4973075488457353,
};
const edition: RisFormula = {
  year: 2026,
  is_current: true,
  credit: '',
  constants: { men: constants, women: constants },
};

async function enterPerformance() {
  const screen = await render(RisSimulator, { edition });
  await screen.getByRole('textbox', { name: 'Bodyweight', exact: true }).fill('80');
  await screen.getByRole('textbox', { name: 'Total lifted', exact: true }).fill('500');
  await screen.getByRole('textbox', { name: 'Target RIS' }).fill('110');
  return screen;
}

it.each(['Bodyweight', 'Total lifted'])(
  'shows target controls for valid inputs and hides them when %s is cleared',
  async (field) => {
    const screen = await render(RisSimulator, { edition });
    const bodyweight = screen.getByRole('textbox', { name: 'Bodyweight', exact: true });
    const total = screen.getByRole('textbox', { name: 'Total lifted', exact: true });
    const target = screen.getByRole('textbox', { name: 'Target RIS' });
    await expect.element(bodyweight).toHaveValue('');
    await expect.element(total).toHaveValue('');
    await expect.element(target).not.toBeInTheDocument();
    await bodyweight.fill('80');
    await expect.element(target).not.toBeInTheDocument();
    await total.fill('500');
    await expect.element(screen.getByRole('status', { name: 'RIS:' })).toHaveTextContent('100.35');
    await expect.element(screen.getByRole('heading', { name: 'Target RIS' })).toBeVisible();
    await expect.element(target).toBeVisible();
    await target.fill('110');
    await screen.getByRole('textbox', { name: field, exact: true }).fill('');
    await expect.element(target).not.toBeInTheDocument();
    await expect
      .element(screen.getByRole('heading', { name: 'Target RIS' }))
      .not.toBeInTheDocument();
    await screen
      .getByRole('textbox', { name: field, exact: true })
      .fill(field === 'Bodyweight' ? '80' : '500');
    await expect.element(target).toBeVisible();
    await expect.element(target).toHaveValue('110');
  }
);

it('accepts decimal commas and rejects nonnumeric input without showing a score', async () => {
  const screen = await enterPerformance();
  const bodyweight = screen.getByRole('textbox', { name: 'Bodyweight', exact: true });
  await bodyweight.fill('80,0');
  await expect.element(screen.getByRole('status', { name: 'RIS:' })).toHaveTextContent('100.35');
  await bodyweight.fill('abc');
  await expect.element(bodyweight).toHaveAttribute('aria-invalid', 'true');
  await expect.element(screen.getByRole('status', { name: 'RIS:' })).not.toBeInTheDocument();
});

it('explains the two alternatives for a higher target', async () => {
  const screen = await enterPerformance();
  await expect.element(screen.getByRole('status', { name: 'RIS:' })).toHaveTextContent('100.35');
  await expect.element(screen.getByText('48.1 kg more', { exact: true })).toBeVisible();
  await expect.element(screen.getByText('8.5 kg less', { exact: true })).toBeVisible();
  await expect.element(screen.getByText(/At a fixed total of 500 kg/)).toBeVisible();
});

it('replaces unnecessary changes with confirmation for a lower or equal target', async () => {
  const screen = await enterPerformance();
  const target = screen.getByRole('textbox', { name: 'Target RIS' });
  await target.fill('100');
  await expect
    .element(screen.getByText('The calculated score is above the target of 100 RIS.'))
    .toBeVisible();
  await expect.element(screen.getByText(/kg more/)).not.toBeInTheDocument();
  await expect.element(screen.getByText(/kg less/)).not.toBeInTheDocument();
  await target.fill('100.35');
  await expect
    .element(screen.getByText('The displayed score equals the target of 100.35 RIS.'))
    .toBeVisible();
  await target.fill('110');
  await expect.element(screen.getByText('48.1 kg more', { exact: true })).toBeVisible();
});

it('updates the score and explains when bodyweight alone cannot reach the target', async () => {
  const screen = await enterPerformance();
  await screen.getByRole('textbox', { name: 'Total lifted', exact: true }).fill('0');
  await expect.element(screen.getByRole('status', { name: 'RIS:' })).toHaveTextContent('0.00');
  await expect
    .element(
      screen.getByText(
        'At the entered total, no finite bodyweight in the model yields this target score.'
      )
    )
    .toBeVisible();
});

it('does not show scenarios for blank or invalid inputs', async () => {
  const screen = await enterPerformance();
  await screen.getByRole('textbox', { name: 'Target RIS' }).fill('');
  await expect
    .element(
      screen.getByText(
        'Enter a positive target score to calculate the corresponding total and bodyweight.'
      )
    )
    .toBeVisible();
  await screen.getByRole('textbox', { name: 'Bodyweight', exact: true }).fill('20');
  await expect.element(screen.getByRole('status', { name: 'RIS:' })).not.toBeInTheDocument();
});
