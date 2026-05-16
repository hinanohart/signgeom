# r/rust post — draft

**Suggested title** (≤ 80 chars):
`signgeom 0.1 — signature-parametric Riemannian geometry in Rust + WGSL`

**Flair:** `🦀 project`

**Body:**

```markdown
Hi r/rust,

I've just published the first 0.1 release of [signgeom](https://github.com/hinanohart/signgeom),
a small differential-geometry library where the *signature* `(p, q, r)`
of the metric is a value in the public API rather than a hard-coded
choice. The same kernel computes Christoffel symbols, geodesics and
curvature in:

* `(n, 0, 0)` — Euclidean / Riemannian,
* `(n−1, 1, 0)` — Minkowski / Lorentzian (the geometry of relativity),
* `(4, 0, 0)` — "Orthogonal" (the geometry of Greg Egan's *Orthogonal*
  trilogy, where light cones do not exist),
* `(2, 2, 0)` — *Dichronauts* split signature (two times),
* `(n, 0, 1)` — Galilean / Newton–Cartan (one degenerate direction).

The crate set is `signgeom-core` (Manifold trait + Christoffel/Ricci/RK4
geodesic), `signgeom-aperiodic` (Wang-tile + Turing-machine encoder),
`signgeom-lenia` (a small Flow-Lenia CA), and `signgeom-cli`. There is a
WebGPU demo under `web/` that integrates the same geodesic in four
signatures side-by-side; the CPU reference path agrees with the WGSL
compute kernel to single-precision tolerance (≤ 1e-5 relative), not
bitwise. The WGSL kernel currently covers 4D flat metrics only;
higher-dimensional and curved metrics run on the CPU integrator.

### v0.1.0 known limitations (honest scope)

- **GPU/CPU agreement is tolerance-based, not bit-exact.** f64 CPU
  integrator versus f32 WGSL kernel cannot match bitwise; we test
  agreement to ≤ 1e-5 relative.
- **`Signature` is value-level, not type-level.** It is a
  `const`-constructible struct `Signature { p, q, r }`. A
  type-level const-generics variant is deferred (decision-log D8).
- **WGSL kernel: dim = 4 only.** Higher-dim returns null (CPU only).
- **Egan-Lenia kernel is Euclidean** in v0.1.0; signature-aware
  convolution is on the v0.2 roadmap (decision-log D9).
- **einstein hat (2023 monotile)** is on the v0.1.x roadmap; v0.1.0
  ships Wang tiles + Turing encoder only.
- Curvature primitives (Christoffel, Riemann, Ricci) are validated
  against Schwarzschild (Ricci ≈ 0), the 2-sphere (R = 2/r²),
  the 2-hyperbolic plane (R = -2/r²), and flat cases in
  `(4,0,0)` / `(3,1,0)` / `(2,2,0)`.

Things I'd love feedback on:

1. **API ergonomics for tensor indices.** Right now everything is
   `Vec<Vec<f64>>` for legibility; would a `ndarray` or `nalgebra`
   second-tier surface be welcome, or is the current shape acceptable?
2. **WebGPU + `wgpu`.** I'm using `wgpu`/WGSL plus Three.js for the
   browser demo; the compute path runs the same kernel as the CPU.
   Is there a better idiom for "compute → readback → draw" loops?
3. **MSRV.** I've pinned 1.85 to keep `const fn` and 2021-edition gates
   simple. Should I push it down further?

Licence: Apache-2.0. Greg-Egan-themed names like
`Signature::orthogonal4` are nominative — every formula was
re-derived from textbooks and arXiv preprints; no source from
`gregegan.net` was inspected. See `NOTICE`.

Repo: https://github.com/hinanohart/signgeom
Docs: https://hinanohart.github.io/signgeom (mdBook + rustdoc + demo)
Crate: TBD (will publish to crates.io once the GitHub release URL is
stable)
```

**Posting checklist (do NOT post until):**

- [ ] `cargo publish --dry-run` is green for every member crate
- [ ] GitHub Pages site at `hinanohart.github.io/signgeom` returns 200
- [ ] At least one star on the repo (signal that the repo is reachable)
- [ ] Author has skim-read the comments policy of r/rust this week

**Notes:**

- Time the post for **Tuesday or Wednesday morning ET** for maximum
  visibility. Avoid weekends and Fridays.
- If a top-level question comes back about "why not geomstats / Manifolds.jl",
  reply with the niche table from `docs/book/src/benchmark.md`.
- If anyone mentions "Egan would not like this", point them at `NOTICE`
  and the licence-strategy chapter; do not get drawn into a debate.
