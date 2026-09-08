import type { CompetitionStatus } from '$lib/types/competition';

interface CompetitionStatusOption {
  value: CompetitionStatus;
  label: string;
  text: string;
}

export const COMPETITION_STATUSES: readonly CompetitionStatusOption[] = [
  { value: 'draft', label: 'Draft', text: 'text-muted' },
  { value: 'upcoming', label: 'Planned', text: 'text-secondary' },
  { value: 'live', label: 'Live', text: 'text-success' },
  { value: 'completed', label: 'Completed', text: 'text-secondary' },
  { value: 'cancelled', label: 'Cancelled', text: 'text-muted line-through' },
];

export const COMPETITION_STATUS_FILTERS = COMPETITION_STATUSES.filter(
  (status) => status.value === 'upcoming' || status.value === 'completed'
);
