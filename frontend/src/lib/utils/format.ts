import type { Locale } from '#lib/i18n/locale.js';
import { NO_VALUE } from '#lib/constants/table.js';
import { displayWeight, type WeightUnit } from './weight-unit.js';

export function formatDate(dateString: string | null, locale: Locale = 'en'): string {
  if (!dateString) return '-';
  const date = new Date(dateString);
  if (Number.isNaN(date.getTime())) return '-';
  return new Intl.DateTimeFormat(locale === 'en' ? 'en-GB' : locale, {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    timeZone: 'UTC',
  }).format(date);
}

export function formatLongDate(dateString: string | null, locale: Locale = 'en'): string {
  if (!dateString) return '';
  const date = new Date(dateString);
  return Number.isNaN(date.getTime())
    ? ''
    : new Intl.DateTimeFormat(locale, {
        day: 'numeric',
        month: 'long',
        year: 'numeric',
        timeZone: 'UTC',
      }).format(date);
}

export function formatLocation(...parts: (string | null | undefined)[]): string {
  return parts.filter(Boolean).join(', ');
}

// An athlete with only one name has no first name, so the halves are joined
// rather than interpolated with a space that would sit there on its own.
export function formatAthleteName(athlete: { first_name: string; last_name: string }): string {
  return [athlete.first_name, athlete.last_name].filter(Boolean).join(' ');
}

// A date on its own does not say how soon a competition is, which is the only thing
// worth knowing about one nobody has lifted yet.
export function formatCountdown(dateString: string | null, locale: Locale = 'en'): string {
  if (!dateString) return '';

  const target = new Date(dateString);
  if (Number.isNaN(target.getTime())) return '';

  const relativeTime = new Intl.RelativeTimeFormat(locale, { numeric: 'auto' });
  const days = Math.round((target.getTime() - Date.now()) / 86_400_000);
  if (Math.abs(days) < 14) return relativeTime.format(days, 'day');
  if (Math.abs(days) < 60) return relativeTime.format(Math.round(days / 7), 'week');
  return relativeTime.format(Math.round(days / 30), 'month');
}

export function countryName(countryCode: string, locale: Locale = 'en'): string {
  try {
    return (
      new Intl.DisplayNames([locale], { type: 'region' }).of(countryCode.toUpperCase()) ??
      countryCode
    );
  } catch {
    return countryCode;
  }
}

// A lift of 0 is a real result, a muscle-up at bodyweight, so only the absence
// of a value is a dash. Guarding on truthiness silently turns it into no data.
export function formatWeight(
  weight: number | string | null | undefined,
  locale: Locale = 'en',
  unit: WeightUnit = 'kg',
  digits?: number
): string {
  if (weight === null || weight === undefined || weight === '') return NO_VALUE;
  return locale === 'en' && unit === 'kg' && digits === undefined
    ? `${weight}`
    : new Intl.NumberFormat(locale, {
        useGrouping: false,
        minimumFractionDigits: digits,
        maximumFractionDigits: digits ?? (unit === 'lb' ? 2 : 3),
      }).format(displayWeight(Number(weight), unit));
}

export function formatScore(
  score: number | string | null | undefined,
  locale: Locale = 'en'
): string {
  return score === null || score === undefined || score === ''
    ? NO_VALUE
    : new Intl.NumberFormat(locale, {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
        useGrouping: false,
      }).format(Number(score));
}
