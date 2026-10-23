import type { components, paths } from '#lib/api/schema.generated.js';

export type RankingEntry = components['schemas']['GlobalRankingEntry'];
export type RankingFilters = NonNullable<paths['/api/v1/rankings']['get']['parameters']['query']>;
