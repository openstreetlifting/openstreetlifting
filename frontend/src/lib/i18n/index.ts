import { getContext, setContext } from 'svelte';
import { DEFAULT_LOCALE, type Locale } from './locale.js';
import french from './fr.js';
import type { Message } from './en.js';

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
    return this.locale === 'fr' ? french[message] : message;
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

  count(
    kind: 'athlete' | 'competition' | 'country' | 'federation' | 'entry',
    count: number
  ): string {
    const plural = new Intl.PluralRules(this.locale).select(count) !== 'one';
    switch (kind) {
      case 'athlete':
        return plural ? this.t('athletes') : this.t('athlete');
      case 'competition':
        return plural ? this.t('competitions') : this.t('competition');
      case 'country':
        return plural ? this.t('countries') : this.t('country');
      case 'federation':
        return plural ? this.t('federations') : this.t('federation');
      case 'entry':
        return plural ? this.t('entries') : this.t('entry');
    }
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
