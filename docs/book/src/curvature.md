# Curvature

The Riemann curvature tensor of the second kind is

\\[
R^a{}_{bcd} = \partial_c \Gamma^a_{db} - \partial_d \Gamma^a_{cb} + \Gamma^a_{ce}\, \Gamma^e_{db} - \Gamma^a_{de}\, \Gamma^e_{cb}
\\]

Contracting the first and third indices gives the Ricci tensor
`Ric_{bd} = R^a{}_{bad}`; contracting again with the inverse metric gives
the scalar curvature `R = g^{bd} Ric_{bd}`.

```rust,no_run
use signgeom_core::{scalar_curvature, Schwarzschild};
let m = Schwarzschild { mass: 1.0 };
let r = scalar_curvature(&m, &[10.0, std::f64::consts::FRAC_PI_2, 0.0, 0.0]).unwrap();
// theory: r = 0 in vacuum
```

## Why scalar curvature vanishes in vacuum

The vacuum Einstein equation `Ric_{ab} = 0` (no matter, no cosmological
constant) implies `R = g^{ab} Ric_{ab} = 0`. The Schwarzschild solution
is the canonical asymptotically-flat vacuum solution, so `R(r, θ, φ, t)`
is identically zero outside the horizon. signgeom's integration test
samples four widely-separated points and demands `|R| < 5e-3`, which is
limited by the central-difference noise in the metric's partial
derivatives, not by the algorithm.

## The Kretschmann scalar

For full curvature diagnostics in vacuum (where `Ric = 0` makes the
scalar curvature useless), compute the Kretschmann scalar
`K = R^{abcd} R_{abcd}`. signgeom does not expose `K` directly in v0.1.0
but it is straightforward to build from `riemann` and `metric` — see
issue #1 on GitHub.
