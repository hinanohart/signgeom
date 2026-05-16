//! Integrate a "geodesic" in Greg Egan's (4, 0, 0) Orthogonal geometry —
//! a setting where the metric has *four* positive directions and no
//! Lorentzian split, so light cones in the usual sense do not exist.
//!
//! Two diagnostics are printed:
//! 1. Line element `ds² = δ_{ij} v^i v^j` along the geodesic — this is
//!    strictly positive in (4, 0, 0), unlike Minkowski.
//! 2. The first and last few sample positions, so it is visually obvious
//!    that the worldline is straight.
//!
//! Run with `cargo run -p signgeom-core --example light_cone_orthogonal`.

use signgeom_core::{integrate_geodesic, GeodesicConfig, MinkowskiFlat, Signature};

fn main() {
    let m = MinkowskiFlat::new(Signature::orthogonal4());
    let x0 = [0.0_f64; 4];
    let v0 = [1.0, 0.5, 0.25, 0.125];
    let cfg = GeodesicConfig {
        steps: 100,
        proper_time: 4.0,
        ..GeodesicConfig::default()
    };

    let path = integrate_geodesic(&m, &x0, &v0, cfg).expect("flat (4,0,0) cannot diverge");

    println!("# light cones in Egan's (4, 0, 0) Orthogonal geometry");
    println!(
        "# eta diag = {:?}",
        Signature::orthogonal4().canonical_diagonal()
    );
    println!("# |v0|² = {}", v0.iter().map(|x| x * x).sum::<f64>());
    println!();

    let total_ds2: f64 = v0.iter().map(|v| v * v).sum::<f64>() * cfg.proper_time;
    println!("predicted ds² at end (Riemannian = positive): {total_ds2:.6}");
    println!();

    println!(
        "first sample:  tau={:6.3}  pos={:?}",
        path[0].tau, path[0].position
    );
    let mid = path.len() / 2;
    println!(
        "middle sample: tau={:6.3}  pos={:?}",
        path[mid].tau, path[mid].position
    );
    let last = path.last().unwrap();
    println!(
        "last sample:   tau={:6.3}  pos={:?}",
        last.tau, last.position
    );

    println!();
    println!("# in (4,0,0) every direction is 'spacelike' — there is no light cone.");
}
