import { apiClient, apiData } from '../client';
import type { RecordGender } from '#lib/types/records.js';

export const recordsService = {
  getRecords(query: {
    gender?: RecordGender;
    country?: string | null;
    federation?: string | null;
    year?: number | null;
    limit?: number;
  }) {
    return apiData(apiClient.GET('/api/v1/records', { params: { query } }));
  },

  getYears() {
    return apiData(apiClient.GET('/api/v1/records/years'));
  },
};
