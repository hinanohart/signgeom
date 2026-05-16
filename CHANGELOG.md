# Changelog

All notable changes to signgeom are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); we use semantic
versioning, but consider every 0.x release potentially breaking.

## [Unreleased]

## [0.1.2] — 2026-05-17

A security-only patch on top of 0.1.1. No API changes.

### Security

- **Bump `vite` from `^5.4.10` to `^6.4.2`** to pick up the path-traversal
  fix for optimized-deps `.map` handling (GHSA-4w7w-66w2-5vf9 /
  CVE-2026-39365). Only affects the dev server when exposed via
  `--host`, but worth closing.
- **Indirectly bump `esbuild` to `>= 0.25.0`** (via the vite upgrade),
  closing the dev-server CORS finding (GHSA-67mh-4wv8-2f99).
- Both findings only affect the local `vite dev` server, not the
  production `vite build` output that ships to GitHub Pages.

## [0.1.1] — 2026-05-17

A post-release hardening pass driven by a 3-agent audit (architecture
retention, fatal-defect, and code-refinement). No new features; every
change is either a correctness fix, an honest-disclosure fix, or a
tightening of the public surface so that v0.1.x is forward-stable.

### Fixed

- **Announcement copy was overclaiming.** The `docs/announcements/r-rust.md`
  and `docs/announcements/mathstodon.md` drafts described the CPU path as
  "bit-exact"; the CPU integrator is f64 + sequential Euler while the WGSL
  kernel is f32 + analytical, so bitwise agreement is physically
  impossible. Both drafts now state "agrees to single-precision tolerance
  (≤ 1e-5 relative)", which is what the implementation actually delivers.
- **README claimed type-level `Signature<P, Q, R>`.** The actual API is a
  value-level `Signature { p, q, r }` constructed by const functions. The
  README now describes the surface as it actually is and links to a
  decision-log entry for the const-generics variant deferred to v0.2.
- **`README.md` Quickstart `cargo run --example` did not work** at the
  workspace root. Examples live under `signgeom-core`; the commands now
  use `cargo run -p signgeom-core --example`.
- **`DEFAULT_FD_STEP` was inconsistent** between `christoffel.rs` (`1e-4`)
  and `curvature.rs` (`1e-3`), with the mdBook documenting `1e-3`. Both
  modules now share a single `pub(crate) const DEFAULT_FD_STEP: f64 = 1e-3`
  declared in `signgeom-core::lib`, matching the documentation.
- **`Schwarzschild::signature()` hardcoded `(3, 1, 0)`** rather than using
  `Signature::minkowski(4)`; replaced with the named constructor.
- **`Signature::minkowski` did not document its panic.** Calling
  `minkowski(0)` panics; the rustdoc now carries a `# Panics` section
  documenting the precondition.
- **Schwarzschild metric assigned unused `_phi` and `_t` bindings.**
  Removed; an inline comment now explains why φ and t do not appear in
  the diagonal components in these coordinates.
- **WebGPU buffers leaked across reruns.** The browser demo destroys
  `paramBuf`, `storage`, and the new `readBack` buffer inside a
  `try { … } finally { … }` so that long-running sessions do not
  accumulate GPU memory across signature changes.
- **WGSL kernel had no dimension guard.** The compute shader hardcodes
  `vec4<f32>` storage; calling with non-4D signatures would silently
  produce garbage. The kernel call now logs a fallback message and
  returns `null` (CPU path takes over).

### Changed

- **`turing_machine_to_tileset` dropped its unused `n_states` parameter.**
  This is a breaking change scoped to v0.1.x: the parameter was prefixed
  `_n_states` since v0.1.0 and never consulted internally. The CLI's
  `tiling --states N` option is also removed for the same reason.
- **`signgeom-core::lib.rs` exposes shared numerical defaults**
  (`DEFAULT_FD_STEP`, `DEFAULT_SINGULAR_TOL`) as `pub(crate) const`s.
  This is an internal change; the public API still routes through the
  `*_with(fd_step, singular_tol)` overloads.

### Added

- **`tests/integration_known_metrics.rs`** validates the curvature
  pipeline against four closed-form references that v0.1.0 did not
  exercise:
  - 2-sphere (radius 1): `R = 2`
  - 2-hyperbolic plane: `R = -2`
  - Flat `(4, 0, 0)` "Orthogonal": `R = 0`
  - Flat `(2, 2, 0)` "Dichronauts": `R = 0`
  - Flat `(5, 0, 0)` Riemannian: `R = 0` (general dim sanity)
- **Honest "known limitations" paragraph** in both announcement drafts,
  enumerating: tolerance-not-bitwise, value-level Signature,
  WGSL kernel 4D-flat scope, Euclidean Lenia kernel, einstein-hat
  roadmap, and the closed-form curvature reference suite.

## [0.1.0] — 2026-05-17

The first release of signgeom. The headline idea is that the *signature*
`(p, q, r)` of a metric is a value in the API, so a single kernel handles
Euclidean, Lorentzian, Egan-style `(4, 0, 0)` "Orthogonal" and `(2, 2, 0)`
"Dichronauts" geometries.

### Added

- **`signgeom-core`** crate:
  - `Signature` value type with `riemannian`, `minkowski`, `orthogonal4`,
    `dichronauts4`, `galilean` named constructors.
  - `Manifold` trait with a default central-difference implementation of
    `metric_partials`.
  - `christoffel`, `riemann`, `ricci`, `scalar_curvature` free functions.
  - RK4 geodesic integrator with configurable step count, proper time,
    blow-up threshold and singular-metric tolerance.
  - `MinkowskiFlat` (constant-metric flat manifold of any signature) and
    `Schwarzschild` (`M = 1` test bed).
- **`signgeom-aperiodic`** crate: Wang-tile data structure, east/north
  adjacency, and a Turing-machine-to-tile-set encoder.
- **`signgeom-lenia`** crate: a small Flow-Lenia-style continuous CA on a
  flat background. Signature-aware kernels are exposed via a future
  extension point.
- **`signgeom-cli`** crate: a `clap`-based CLI with `signature`, `geodesic`,
  `ricci`, `tiling`, `lenia` subcommands.
- **Web demo** under `web/`: a TypeScript + Three.js (WebGPU when available)
  reference UI that integrates the same geodesic in four signatures.
- **Documentation**: rustdoc with `#![warn(missing_docs)]` on every public
  item, plus an mdBook under `docs/book/` covering the signature concept,
  Christoffel symbols, geodesics and a "license strategy" page on Greg
  Egan's themes.
- **Tests**: 34 tests across unit, property (`proptest`) and integration
  suites, including a Schwarzschild Ricci-flat sanity check at four points.
- **CI**: GitHub Actions matrix (Linux/macOS/Windows × stable Rust) plus
  `cargo audit`, `cargo deny`, `gitleaks`, web `tsc` + `vite build`,
  rustdoc and mdBook deploy to GitHub Pages.
- **Licensing**: Apache-2.0 (`LICENSE`), with a `NOTICE` file recording the
  Egan-inspiration-but-independent-implementation policy and a
  `plugins/egan-applets/LICENSE-PENDING.md` stub reserved for v0.2+.

### Known limitations

- WebGPU geodesic compute is implemented for the flat case only; curved
  metrics still use the CPU integrator.
- `signgeom-lenia` integrates on a Euclidean `(2, 0, 0)` background only;
  signature-aware Lenia kernels are scheduled for a later release.
- Python bindings (`pyo3`) and Julia bindings (`jlrs`) are deferred to
  v0.2+, as recorded in `architecture/decision-log.md`.
- `f32`-only WebGPU pipelines may drift on long geodesic integrations
  (Earth-mass black hole, > 10⁴ steps). Use the CPU path for high-precision
  work until the planned double-single emulation lands.

[Unreleased]: https://github.com/hinanohart/signgeom/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/hinanohart/signgeom/releases/tag/v0.1.2
[0.1.1]: https://github.com/hinanohart/signgeom/releases/tag/v0.1.1
[0.1.0]: https://github.com/hinanohart/signgeom/releases/tag/v0.1.0
