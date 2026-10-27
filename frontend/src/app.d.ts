// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      locale: import('#lib/i18n/locale.js').Locale;
    }
    interface PageData {
      locale: import('#lib/i18n/locale.js').Locale;
    }
    // interface PageState {}
    // interface Platform {}
  }
}

export {};
