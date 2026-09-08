import type { AthleteStatus } from '$lib/types/enums';

/**
 * How a result that does not stand is shown. Both the competition page and the
 * athlete page render the same badge, so the wording lives here rather than
 * being written out twice and drifting.
 *
 * `competed` is absent from both maps on purpose: a result that stands needs no
 * explaining, and the badge is only drawn for the other two.
 */
export const ATHLETE_STATUS_LABEL: Partial<Record<AthleteStatus, string>> = {
  disqualified: 'DQ',
  no_show: 'NS',
};

const ATHLETE_STATUS_TITLE: Partial<Record<AthleteStatus, string>> = {
  disqualified: 'Disqualified',
  no_show: 'Did not lift',
};

/**
 * The badge is two letters, so the title carries the meaning. A reason from the
 * source is better than either, when there is one.
 */
export function athleteStatusTitle(status: AthleteStatus, reason: string | null): string {
  const name = ATHLETE_STATUS_TITLE[status] ?? status;

  return reason ? `${name}: ${reason.toLowerCase()}` : name;
}
