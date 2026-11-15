import { apiClient, apiData } from '../client';
import type { paths } from '#lib/api/schema.generated.js';

type RecordsQuery = NonNullable<paths['/api/v1/records']['get']['parameters']['query']>;

export const recordsService = {
  getRecords(query: RecordsQuery) {
    return apiData(apiClient.GET('/api/v1/records', { params: { query } }));
  },

  getFacets(query: RecordsQuery) {
    return apiData(apiClient.GET('/api/v1/records/facets', { params: { query } }));
  },
};
