# Christoffel symbols

The Levi-Civita connection on a (pseudo-)Riemannian manifold is determined
by the metric and torsion-freeness. Its coefficients are the *Christoffel
symbols of the second kind*:

\\[
\Gamma^k_{ij} = \tfrac{1}{2}\, g^{kl}\bigl(\partial_i g_{jl} + \partial_j g_{il} - \partial_l g_{ij}\bigr)
\\]

In matrix-by-matrix form:

```rust,no_run
use signgeom_core::{christoffel, Schwarzschild, Manifold};
let m = Schwarzschild { mass: 1.0 };
let gamma = christoffel(&m, &[10.0, std::f64::consts::FRAC_PI_2, 0.0, 0.0]).unwrap();
// gamma[k][i][j] — Γ^k_{ij}
```

## Symmetry

The Levi-Civita Γ is symmetric in its lower indices:
`Γ^k_{ij} = Γ^k_{ji}`. The numerical implementation does *not* exploit
this — it computes both halves and returns the full `O(n^3)` tensor —
because for small `n` (≤ 6) the wasted work is irrelevant and the explicit
form makes property tests easier to write.

## What vanishes in a flat metric

When `g_{ij}` is constant on the chart (e.g. `MinkowskiFlat` in any
signature) every partial derivative vanishes, so `Γ ≡ 0`. signgeom's unit
tests verify this for `(3, 1, 0)`, `(4, 0, 0)` and `(2, 2, 0)` separately
— a small but irreplaceable smoke test for the code path that mixes the
metric, the inverse metric and the partial derivatives.

## Finite differences

If a manifold implementor does not provide an analytic `metric_partials`,
signgeom falls back to second-order central differences with step `h`.
The default step is `1e-3`, chosen so that on `f64` the truncation error
(`O(h²) ≈ 1e-6`) and the round-off (`≈ 1e-16 / h`) are both small. Pass
your own through `christoffel_with` if you need a sharper bound for a
specific manifold.
