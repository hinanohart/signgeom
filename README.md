# signgeom

[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)
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

## Features (v0.1.0)

- **`signgeom-core`** — `Signature<P, Q, R>` type-level signature, `Manifold`
  trait, Christoffel symbols, Riemann curvature, geodesic integration.
- **`signgeom-aperiodic`** — Wang-tile / einstein-hat tilings on
  signature-aware manifolds; a small Turing-tile compiler.
- **`signgeom-lenia`** — *Flow-Lenia*-style continuous cellular automata on
  arbitrary-signature backgrounds.
- **`signgeom-cli`** — command-line front-end.
- **`web/`** — TypeScript + Three.js r171 WebGPU demo (WebGL2 fallback).

## Quickstart

```bash
cargo build --release
cargo test
cargo run -p signgeom-cli -- --help

# Try the bundled examples
cargo run --example light_cone_orthogonal
cargo run --example dichronauts_geodesic
cargo run --example schwarzschild_compare
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
property tests cover sign-invariants, Schwarzschild known-values are checked
to 1e-6 in tests, but long geodesic integrations on WebGPU `f32` may drift.

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md). All conversations are governed by
the [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## License

Apache-2.0. See [`LICENSE`](LICENSE) and [`NOTICE`](NOTICE).
