# `signgeom-core` examples

Three runnable examples designed to make signature-parametric geometry
tactile, not abstract.

| Example | What it shows | Run |
|---|---|---|
| `light_cone_orthogonal` | A geodesic in Greg Egan's (4, 0, 0) "Orthogonal" geometry, where no Lorentzian split exists and the line element is strictly positive. | `cargo run -p signgeom-core --example light_cone_orthogonal` |
| `dichronauts_geodesic` | Three velocities in (2, 2, 0) — *Dichronauts* — that partition into spacelike, null, and timelike classes. | `cargo run -p signgeom-core --example dichronauts_geodesic` |
| `schwarzschild_compare` | Side-by-side comparison of flat (3, 1, 0) and Schwarzschild paths, with a Ricci-scalar sanity check (the theoretical value is exactly zero in vacuum). | `cargo run -p signgeom-core --example schwarzschild_compare` |

Each example prints to stdout — none of them require WebGPU or a browser.
For the WebGPU visualisation see [`../../web/`](../../web/).
