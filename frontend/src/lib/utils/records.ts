import type { Movement } from '#lib/types/enums.js';
import type { LiftRecord, RecordMark } from '#lib/types/records.js';

export function recordFor(records: readonly LiftRecord[], movement: Movement): RecordMark | null {
  return records.find((record) => record.movement === movement)?.mark ?? null;
}
