import { COMPETITION_STATUSES, type CompetitionStatus } from '$lib/types/enums';

interface CompetitionStatusOption {
  value: CompetitionStatus;
  label: string;
  text: string;
}

const STATUS_STYLE: Record<CompetitionStatus, Omit<CompetitionStatusOption, 'value'>> = {
  draft: { label: 'Draft', text: 'text-muted' },
  upcoming: { label: 'Planned', text: 'text-secondary' },
  live: { label: 'Live', text: 'text-success' },
  completed: { label: 'Completed', text: 'text-secondary' },
  cancelled: { label: 'Cancelled', text: 'text-muted line-through' },
};

export const COMPETITION_STATUS_OPTIONS: readonly CompetitionStatusOption[] =
  COMPETITION_STATUSES.map((value) => ({ value, ...STATUS_STYLE[value] }));

export const COMPETITION_STATUS_FILTERS = COMPETITION_STATUS_OPTIONS.filter(
  (status) => status.value === 'upcoming' || status.value === 'completed'
);
