import { error } from '@sveltejs/kit';
import { risService } from '$lib/server/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
  try {
    const [formulas, distribution] = await Promise.all([
      risService.getFormulas(),
      risService.getDistribution(),
    ]);

    const editions = formulas.sort((a, b) => a.year - b.year);

    return {
      editions,
      distribution,
    };
  } catch {
    error(503, 'The RIS editions are unavailable right now');
  }
};
