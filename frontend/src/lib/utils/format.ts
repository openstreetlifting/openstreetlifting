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

const regionNames = new Intl.DisplayNames(['en'], { type: 'region' });

export function countryName(countryCode: string): string {
  try {
    return regionNames.of(countryCode.toUpperCase()) ?? countryCode;
  } catch {
    return countryCode;
  }
}
