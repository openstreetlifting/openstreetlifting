import { asRankedGender } from '#lib/types/enums.js';
import { normalizeEvent } from '#lib/utils/competition-format.js';
import { text, year } from '#lib/utils/filters.js';

const country = (raw: string | null) => text(raw)?.toUpperCase() ?? null;

export const RANKING_FILTERS = {
  gender: asRankedGender,
  category: text,
  year,
  country,
  federation: text,
  series: text,
};

export const COMPETITION_FILTERS = {
  federation: text,
  country,
  year,
  event: (raw: string | null) => normalizeEvent(raw) || null,
};

export const RECORD_FILTERS = {
  country,
  federation: text,
  series: text,
  year,
};
