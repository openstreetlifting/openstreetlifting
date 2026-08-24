// Mirrors osl_domain::slugify. The separator class matches Rust's
// char::is_alphanumeric exactly: Alphabetic (a superset of \p{L}) plus N.
export function slugify(text: string): string {
  return text
    .normalize('NFD')
    .replace(/\p{M}/gu, '')
    .toLowerCase()
    .split(/[^\p{Alphabetic}\p{N}]+/u)
    .filter(Boolean)
    .join('-');
}
