export interface RisConstants {
  a: number;
  k: number;
  b: number;
  v: number;
  q: number;
}

export interface RisFormula {
  year: number;
  is_current: boolean;
  credit: string;
  constants: {
    men: RisConstants;
    women: RisConstants;
  };
}
