import { asRankedGender } from '#lib/types/enums.js';
import { normalizeEvent } from '#lib/utils/competition-format.js';
import { text, year } from '#lib/utils/filters.js';

export const LEADERBOARD_FILTERS = { gender: asRankedGender, category: text, country: text };

export const RANKING_FILTERS = {
  gender: asRankedGender,
  category: text,
  year,
  country: text,
  federation: text,
  series: text,
};

export const COMPETITION_FILTERS = {
  federation: text,
  country: text,
  year,
  event: (raw: string | null) => normalizeEvent(raw) || null,
};

export const RECORD_FILTERS = {
  country: (raw: string | null) => text(raw)?.toUpperCase() ?? null,
  federation: text,
  year,
};
