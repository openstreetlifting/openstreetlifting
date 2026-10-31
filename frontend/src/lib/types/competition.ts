import type { components, paths } from '#lib/api/schema.generated.js';

export type { AthleteStatus, CompetitionStatus, Gender, Movement, RisSource } from './enums';

type Schemas = components['schemas'];
type CompetitionResponse = Schemas['CompetitionResponse'];

export type Federation = Schemas['CompetitionFederation'];
export type CompetitionMovement = Schemas['MovementInfo'];
export type Attempt = Schemas['AttemptInfo'];
export type Participant = Schemas['ParticipantDetail'];
export type CategoryDetail = Schemas['CategoryDetail'];
export type Competition = Omit<CompetitionResponse, 'federation' | 'movements'> & {
  federation: Federation;
  movements: CompetitionMovement[];
};
export type CompetitionDetail = Competition & {
  categories: CategoryDetail[];
  editions: Schemas['CompetitionEdition'][];
};
export type CompetitionFilters = Omit<
  NonNullable<paths['/api/v1/competitions']['get']['parameters']['query']>,
  'include'
>;
