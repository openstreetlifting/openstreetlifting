import { error, redirect } from '@sveltejs/kit';
import { isLocale, LOCALE_COOKIE } from '#lib/i18n/locale.js';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, cookies, url }) => {
  const form = await request.formData();
  const locale = form.get('locale');
  if (!isLocale(locale)) error(400, 'Unsupported language');

  cookies.set(LOCALE_COOKIE, locale, {
    path: '/',
    maxAge: 60 * 60 * 24 * 365,
    httpOnly: true,
    sameSite: 'lax',
    secure: url.protocol === 'https:',
  });

  const returnTo = form.get('returnTo');
  let target: URL;
  try {
    target = new URL(typeof returnTo === 'string' ? returnTo : '/', url);
  } catch {
    target = new URL('/', url);
  }
  // Never allow the return path to become an external redirect.
  redirect(
    303,
    target.origin === url.origin ? `${target.pathname}${target.search}${target.hash}` : '/'
  );
};
