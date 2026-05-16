# Benchmark notes

The `geodesic` criterion benchmark (`crates/signgeom-core/benches/`) is
the single comparable number we publish in v0.1.0. It measures
end-to-end geodesic integration of a 4-dimensional manifold for 200
RK4 steps on a single thread.

## v0.1.0 baseline numbers

Measured on a desktop x86_64 (single-threaded, release profile, criterion
quick mode for reproducibility):

| Manifold | Steps | Time | Per-step |
|---|---:|---:|---:|
| `MinkowskiFlat` (3, 1, 0) | 200 | ~2.21 ms | ~11 µs |
| `MinkowskiFlat` (4, 0, 0) — *Orthogonal* | 200 | ~2.50 ms | ~12 µs |
| `MinkowskiFlat` (2, 2, 0) — *Dichronauts* | 200 | ~3.52 ms | ~18 µs |
| `Schwarzschild` (M=1) | 50 | ~1.00 ms | ~20 µs |
| `Schwarzschild` (M=1) | 200 | ~3.92 ms | ~20 µs |

Two observations:

1. **Switching signature is essentially free** in flat space — the
   per-step cost is dominated by the central-difference probing of the
   metric, which has the same arithmetic shape regardless of `(p, q, r)`.
2. **Curvature roughly doubles the per-step cost**, as expected: the
   Schwarzschild metric varies with position, so the finite-difference
   probes hit a genuinely different metric on each call.

## How signgeom compares to existing libraries (qualitative)

| Library | Signature support | Backend | License | Web demo |
|---|---|---|---|---|
| [geomstats](https://geomstats.github.io) | Riemannian only | NumPy / PyTorch | MIT | no |
| [einsteinpy](https://docs.einsteinpy.org) | (3, 1, 0) only | NumPy | MIT | no |
| [Manifolds.jl](https://juliamanifolds.github.io/Manifolds.jl/) | Riemannian only | Julia | MIT | no |
| **signgeom (this)** | **any (p, q, r)** | **Rust + WGSL** | **Apache-2.0** | **yes** |

A direct head-to-head wall-clock comparison against einsteinpy would
require running Python in CI on the same hardware; doing so honestly
needs more setup than fits in v0.1.0. The qualitative axes above are the
ones we believe matter for users choosing a library, and they make the
signgeom niche clear: *non-Riemannian × Web × Rust* has not been
occupied by an existing OSS project.

## Reproducing locally

```bash
cargo bench --bench geodesic --       # full run
cargo bench --bench geodesic -- --quick  # 1 sample / metric, ~30 s total
```

Criterion writes a static HTML report to `target/criterion/report/index.html`.
