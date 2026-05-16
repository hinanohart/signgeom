// Mirror of crates/signgeom-core/src/signature.rs — the metric signature
// triple (p, q, r) and a few named constructors. The TypeScript side is
// deliberately a runtime value so that the UI can swap signatures by user
// input rather than at compile time.

export interface Signature {
  readonly p: number;
  readonly q: number;
  readonly r: number;
}

export const SIGNATURES = {
  riemannian: { p: 4, q: 0, r: 0 } satisfies Signature,
  minkowski: { p: 3, q: 1, r: 0 } satisfies Signature,
  orthogonal4: { p: 4, q: 0, r: 0 } satisfies Signature,
  dichronauts4: { p: 2, q: 2, r: 0 } satisfies Signature,
} as const;

export type SignatureKey = keyof typeof SIGNATURES;

export function dim(s: Signature): number {
  return s.p + s.q + s.r;
}

export function canonicalDiagonal(s: Signature): Float32Array {
  const n = dim(s);
  const out = new Float32Array(n);
  for (let i = 0; i < s.p; i++) out[i] = 1;
  for (let i = s.p; i < s.p + s.q; i++) out[i] = -1;
  return out;
}
