import { apiClient } from '../client';
import type { RisFormula } from '$lib/types/ris';

export const risService = {
  /** Every published edition, newest last, as the backend holds them. */
  async getFormulas(): Promise<RisFormula[]> {
    return apiClient.get<RisFormula[]>('/api/v1/ris/formulas');
  },
};
