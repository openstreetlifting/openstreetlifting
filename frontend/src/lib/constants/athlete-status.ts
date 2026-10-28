import { I18n } from '#lib/i18n/index.js';
import type { Locale } from '#lib/i18n/locale.js';
import type { AthleteStatus } from '#lib/types/enums.js';

/**
 * How a result that does not stand is shown. `competed` is absent from both
 * maps: the badge is only drawn for the other two.
 */
export const ATHLETE_STATUS_LABEL: Partial<Record<AthleteStatus, string>> = {
  disqualified: 'DQ',
  no_show: 'NS',
};

/** The badge is two letters, so the title carries the meaning. */
export function athleteStatusTitle(
  status: AthleteStatus,
  reason: string | null,
  locale: Locale = 'en'
): string {
  const i18n = new I18n(() => locale);
  const name =
    status === 'disqualified'
      ? i18n.t('Disqualified')
      : status === 'no_show'
        ? i18n.t('Did not lift')
        : status;

  return reason ? `${name}: ${reason.toLowerCase()}` : name;
}
