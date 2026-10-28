import { getContext, setContext } from 'svelte';
import { DEFAULT_LOCALE, type Locale } from './locale.js';
import { counts as englishCounts } from './en.js';
import french, { counts as frenchCounts } from './fr.js';
import german, { counts as germanCounts } from './de.js';
import italian, { counts as italianCounts } from './it.js';
import polish, { counts as polishCounts } from './pl.js';
import { localeHref } from './paths.js';

import type { Message } from './en.js';
import type { CountKind, CountMessages } from './types.js';
import type { ResolvedPathname } from '$app/types';

const translations = { fr: french, de: german, it: italian, pl: polish };

const countMessages: Record<Locale, CountMessages> = {
  en: englishCounts,
  fr: frenchCounts,
  de: germanCounts,
  it: italianCounts,
  pl: polishCounts,
};

// Placeholder names are part of the English message's type, including at call sites.
export type Placeholders<T extends string> = T extends `${string}{${infer Name}}${infer Rest}`
  ? Name | Placeholders<Rest>
  : never;
type Arguments<T extends Message> = [Placeholders<T>] extends [never]
  ? []
  : [values: Record<Placeholders<T>, string | number>];

export class I18n {
  constructor(private readonly getLocale: () => Locale = () => DEFAULT_LOCALE) {}

  get locale(): Locale {
    return this.getLocale();
  }

  t<T extends Message>(message: T, ...args: Arguments<T>): string {
    const translated = this.template(message);
    const values = args[0] as Record<string, string | number> | undefined;
    return translated.replace(/\{(\w+)\}/g, (_, name: string) =>
      String(values?.[name] ?? `{${name}}`)
    );
  }

  template(message: Message): string {
    return this.locale === 'en' ? message : translations[this.locale][message];
  }

  href(path: string): ResolvedPathname {
    // Localized public URLs are handled by the universal reroute hook.
    return localeHref(path, this.locale) as ResolvedPathname;
  }

  number(value: number, digits?: number): string {
    return new Intl.NumberFormat(this.locale, {
      minimumFractionDigits: digits,
      maximumFractionDigits: digits ?? 2,
    }).format(value);
  }

  movement(value: string): string {
    switch (value.toLowerCase().replace(/[ -]/g, '')) {
      case 'muscleup':
        return this.t('Muscle-up');
      case 'pullup':
        return this.t('Pull-up');
      case 'dips':
        return this.t('Dips');
      case 'squat':
        return this.t('Squat');
      default:
        return value;
    }
  }

  count(kind: CountKind, count: number): string {
    const category = new Intl.PluralRules(this.locale).select(count);
    const forms = countMessages[this.locale][kind];
    return forms[category] ?? forms.other;
  }
}

export const I18N_CONTEXT = Symbol('i18n');

/** Create one instance per layout, never a shared mutable server-side locale. */
export function provideI18n(locale: () => Locale): I18n {
  return setContext(I18N_CONTEXT, new I18n(locale));
}

export function useI18n(): I18n {
  return getContext<I18n | undefined>(I18N_CONTEXT) ?? new I18n();
}
