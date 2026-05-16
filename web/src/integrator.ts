// Software (CPU/TS) reference geodesic integrator. The WebGPU path computes
// the same answer in a compute shader. Both kept simple: flat metric only —
// Christoffel symbols vanish — so the geodesic is a straight line. The point
// of the demo is to show, side-by-side, that the *same* initial velocity
// produces visually different worldlines because the *axes* themselves carry
// different metric signs.

import { Signature, dim, canonicalDiagonal } from "./signature";

export interface PathSample {
  position: number[];
  velocity: number[];
  tau: number;
  /** ds² accumulated up to this sample (signed in pseudo-Riemannian signatures). */
  ds2: number;
}

export interface IntegratorOptions {
  steps: number;
  tauMax: number;
}

export function integrateFlat(
  signature: Signature,
  x0: ReadonlyArray<number>,
  v0: ReadonlyArray<number>,
  opts: IntegratorOptions,
): PathSample[] {
  if (x0.length !== dim(signature) || v0.length !== dim(signature)) {
    throw new Error(
      `dimension mismatch: signature has ${dim(signature)} dims, x0 has ${x0.length}, v0 has ${v0.length}`,
    );
  }
  if (opts.steps < 1) throw new Error("steps must be >= 1");
  const dt = opts.tauMax / opts.steps;
  const eta = canonicalDiagonal(signature);
  const path: PathSample[] = [];
  let x = [...x0];
  const v = [...v0];
  let ds2 = 0;
  path.push({ position: [...x], velocity: [...v], tau: 0, ds2: 0 });
  for (let i = 1; i <= opts.steps; i++) {
    for (let k = 0; k < x.length; k++) x[k] = x[k] + dt * v[k];
    // ds² = η_{kk} (v^k)² dt for the flat metric
    let line = 0;
    for (let k = 0; k < v.length; k++) line += eta[k]! * v[k]! * v[k]!;
    ds2 += line * dt;
    path.push({ position: [...x], velocity: [...v], tau: i * dt, ds2 });
  }
  return path;
}
