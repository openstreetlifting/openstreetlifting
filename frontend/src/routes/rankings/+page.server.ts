import { localeHref, pathLocale } from '#lib/i18n/paths.js';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
  const target = url.search ? `/${url.search}` : '/';
  redirect(308, localeHref(target, pathLocale(url.pathname) ?? 'en'));
};
