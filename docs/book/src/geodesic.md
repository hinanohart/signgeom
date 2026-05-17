# Geodesics

A *geodesic* is a curve `x(τ)` whose covariant acceleration vanishes:

\\[
\frac{d^2 x^k}{d\tau^2} + \Gamma^k_{ij}\, \frac{dx^i}{d\tau}\, \frac{dx^j}{d\tau} = 0
\\]

In a Riemannian setting this is the shortest-path equation; in Lorentzian
signature it splits into timelike, spacelike and null classes according to
the sign of `g_{ij} (dx^i/dτ)(dx^j/dτ)`. In Egan's `(2, 2, 0)` the same
three causal classes apply, but the cone structure is richer because two
distinct directions can be timelike (see the `dichronauts_geodesic`
example in `crates/signgeom-core/examples/`).

## Numerical integration

signgeom uses classical fourth-order Runge–Kutta on the augmented state
`(x, v)`. Each step evaluates the Christoffel tensor four times. For a
4-dimensional manifold and the default 1 000 steps this is around 30 µs of
real time on a typical laptop CPU. A WebGPU compute path computes the
same step in the browser when an adapter is available.

```rust,no_run
use signgeom_core::{integrate_geodesic, MinkowskiFlat, GeodesicConfig, Signature};
let m = MinkowskiFlat::new(Signature::minkowski(4));
let cfg = GeodesicConfig { steps: 200, proper_time: 5.0, ..Default::default() };
let path = integrate_geodesic(&m, &[0.0; 4], &[0.5, 0.0, 0.0, 1.0], cfg).unwrap();
let endpoint = path.last().unwrap();
```

## What divergence looks like

The integrator returns `Error::GeodesicDiverged` when any coordinate goes
non-finite or exceeds `blowup_threshold` (default `1e9`). On a black-hole
plunge orbit this usually fires as the integrator approaches `r = 2M`,
where the Schwarzschild chart is singular and the integrator's effective
step size diverges. The right fix is a chart change (e.g. Kruskal–Szekeres),
not a smaller step.
