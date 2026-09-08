import type { AthleteStatus } from '$lib/types/enums';

/**
 * How a result that does not stand is shown. `competed` is absent from both
 * maps: the badge is only drawn for the other two.
 */
export const ATHLETE_STATUS_LABEL: Partial<Record<AthleteStatus, string>> = {
  disqualified: 'DQ',
  no_show: 'NS',
};

const ATHLETE_STATUS_TITLE: Partial<Record<AthleteStatus, string>> = {
  disqualified: 'Disqualified',
  no_show: 'Did not lift',
};

/** The badge is two letters, so the title carries the meaning. */
export function athleteStatusTitle(status: AthleteStatus, reason: string | null): string {
  const name = ATHLETE_STATUS_TITLE[status] ?? status;

  return reason ? `${name}: ${reason.toLowerCase()}` : name;
}
