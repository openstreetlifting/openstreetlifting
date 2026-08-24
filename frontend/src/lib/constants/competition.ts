import type { CompetitionStatus } from '$lib/types/competition';

interface CompetitionStatusOption {
  value: CompetitionStatus;
  label: string;
  text: string;
}

export const COMPETITION_STATUSES: readonly CompetitionStatusOption[] = [
  { value: 'draft', label: 'Draft', text: 'text-zinc-600' },
  { value: 'upcoming', label: 'Planned', text: 'text-zinc-300' },
  { value: 'live', label: 'Live', text: 'text-emerald-400' },
  { value: 'completed', label: 'Completed', text: 'text-zinc-400' },
  { value: 'cancelled', label: 'Cancelled', text: 'text-zinc-600 line-through' },
];

export const COMPETITION_STATUS_FILTERS = COMPETITION_STATUSES.filter(
  (status) => status.value === 'upcoming' || status.value === 'completed'
);
