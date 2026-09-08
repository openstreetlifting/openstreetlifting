import { COMPETITION_STATUSES, type CompetitionStatus } from '$lib/types/enums';

interface CompetitionStatusOption {
  value: CompetitionStatus;
  label: string;
  text: string;
}

const STATUS_STYLE: Record<CompetitionStatus, Omit<CompetitionStatusOption, 'value'>> = {
  draft: { label: 'Draft', text: 'text-zinc-600' },
  upcoming: { label: 'Planned', text: 'text-zinc-300' },
  live: { label: 'Live', text: 'text-emerald-400' },
  completed: { label: 'Completed', text: 'text-zinc-400' },
  cancelled: { label: 'Cancelled', text: 'text-zinc-600 line-through' },
};

export const COMPETITION_STATUS_OPTIONS: readonly CompetitionStatusOption[] =
  COMPETITION_STATUSES.map((value) => ({ value, ...STATUS_STYLE[value] }));

export const COMPETITION_STATUS_FILTERS = COMPETITION_STATUS_OPTIONS.filter(
  (status) => status.value === 'upcoming' || status.value === 'completed'
);
