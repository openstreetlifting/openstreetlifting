export function formatDate(dateString: string | null): string {
  if (!dateString) return '-';
  const date = new Date(dateString);
  const day = String(date.getDate()).padStart(2, '0');
  const month = String(date.getMonth() + 1).padStart(2, '0');
  return `${day}-${month}-${date.getFullYear()}`;
}

export function formatLocation(...parts: (string | null | undefined)[]): string {
  return parts.filter(Boolean).join(', ');
}

const relativeTime = new Intl.RelativeTimeFormat('en', { numeric: 'auto' });

// A date on its own does not say how soon a meet is, which is the only thing
// worth knowing about one nobody has lifted yet.
export function formatCountdown(dateString: string | null): string {
  if (!dateString) return '';

  const target = new Date(dateString);
  if (Number.isNaN(target.getTime())) return '';

  const days = Math.round((target.getTime() - Date.now()) / 86_400_000);
  if (Math.abs(days) < 14) return relativeTime.format(days, 'day');
  if (Math.abs(days) < 60) return relativeTime.format(Math.round(days / 7), 'week');
  return relativeTime.format(Math.round(days / 30), 'month');
}

const regionNames = new Intl.DisplayNames(['en'], { type: 'region' });

export function countryName(countryCode: string): string {
  try {
    return regionNames.of(countryCode.toUpperCase()) ?? countryCode;
  } catch {
    return countryCode;
  }
}
