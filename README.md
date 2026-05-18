# signgeom

[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)](rust-toolchain.toml)
[![Status](https://img.shields.io/badge/status-early%20alpha-yellow)]()

**Signature-parametric Riemannian / pseudo-Riemannian geometry in Rust, with a WebGPU front-end.**

signgeom is a small computational geometry library that treats the *signature*
of a metric — `(p, q, r)`, the number of positive, negative, and degenerate
eigenvalues — as a first-class type parameter. The same kernel computes:

| Signature | Familiar name | Where it appears |
|---|---|---|
| `(n, 0)` | Riemannian | classical geometry, machine learning |
| `(n−1, 1)` | Lorentzian / Minkowski | relativity, the universe we live in |
| `(n, 0)` with `n = 4` | "Orthogonal" | Greg Egan's *Orthogonal* trilogy |
| `(2, 2)` | split / neutral | Egan's *Dichronauts* |
| `(p, q, r)` with `r > 0` | degenerate | Galilean / Newton–Cartan |

The intent is to make non-Euclidean geometry as easy to compute, visualise and
embed in the browser as Euclidean geometry already is.

## Features (v0.1.x)

- **`signgeom-core`** — value-type `Signature { p, q, r }` (const-constructible
  with `Signature::riemannian(n)` / `minkowski(n)` / `orthogonal4()` /
  `dichronauts4()` / `galilean(n)`), `Manifold` trait, Christoffel symbols,
  Riemann / Ricci / scalar curvature, RK4 geodesic integrator.
- **`signgeom-aperiodic`** — Wang-tile matching rules, east/north adjacency,
  and a small Turing-machine-to-tile-set compiler. The 2023 einstein-hat
  monotile is on the v0.1.x roadmap.
- **`signgeom-lenia`** — a small Flow-Lenia-style continuous CA on a flat
  Euclidean background. A signature-aware kernel is on the v0.2 roadmap.
- **`signgeom-cli`** — `clap`-based command-line front-end.
- **`web/`** — TypeScript browser demo using the standard WebGPU API
  (no `wgpu` Rust crate, no Three.js) with Canvas2D rendering. The WGSL
  compute kernel covers 4D flat metrics only in v0.1.x; CPU/GPU
  agreement is validated to single-precision tolerance (≤ 1e-5
  relative), not bitwise.

## Quickstart

```bash
cargo build --release
cargo test --workspace
cargo run -p signgeom-cli -- --help

# Try the bundled examples (example crate is signgeom-core)
cargo run -p signgeom-core --example light_cone_orthogonal
cargo run -p signgeom-core --example dichronauts_geodesic
cargo run -p signgeom-core --example schwarzschild_compare
```

## Greg Egan note (please read)

The mathematical themes of signgeom were inspired by Greg Egan's *Orthogonal*
trilogy, *Dichronauts*, *Schild's Ladder*, *Permutation City*, *Wang's
Carpets* and *Diaspora*. Every formula in this repository was independently
re-derived from public-domain mathematics (textbooks, arXiv preprints,
peer-reviewed papers). **No code, applet source or asset from
[gregegan.net](https://www.gregegan.net) has been inspected or copied.**

signgeom is not endorsed by or affiliated with Greg Egan.

See [`NOTICE`](NOTICE) and [`docs/book/src/license-strategy.md`](docs/book/src/license-strategy.md).

## Status

This is an early alpha. The public API may break before v1.0. Numerical
results should be regarded as "directionally correct" until a v1.0 release —
property tests cover sign-invariants, Schwarzschild Ricci-flatness is
checked to ≤ 5e-3 absolute (the dominant cost is fourth-derivative
finite-difference noise), and long geodesic integrations on WebGPU `f32`
may drift.

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md). All conversations are governed by
the [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## License

MIT. See [`LICENSE`](LICENSE) and [`NOTICE`](NOTICE).
