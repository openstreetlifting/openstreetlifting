export type CountKind = 'athlete' | 'competition' | 'country' | 'federation' | 'entry';
export type CountMessages = Record<
  CountKind,
  Partial<Record<Intl.LDMLPluralRule, string>> & { other: string }
>;
