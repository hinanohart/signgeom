//! Schwarzschild vs. flat space, head-to-head.
//!
//! We integrate a radial-and-tangential geodesic in two metrics:
//! 1. flat (3, 1, 0) — control case, straight line in any chart
//! 2. Schwarzschild M=1 — the same initial data, now feeling curvature
//!
//! Then we print the two endpoints. The Schwarzschild path bends; the flat
//! path doesn't. We also print the Ricci scalar, which is identically zero
//! in vacuum — this is the canonical sanity check that the numerical pipeline
//! is wired up correctly.
//!
//! Run with `cargo run -p signgeom-core --example schwarzschild_compare`.

use signgeom_core::{
    integrate_geodesic, scalar_curvature, GeodesicConfig, MinkowskiFlat, Schwarzschild, Signature,
};

fn main() {
    let cfg = GeodesicConfig {
        steps: 400,
        proper_time: 4.0,
        ..GeodesicConfig::default()
    };

    // Spherical coordinates (r, θ, φ, t). Start at r=10M, equatorial plane,
    // with a small tangential angular velocity.
    let x0 = [10.0, std::f64::consts::FRAC_PI_2, 0.0, 0.0];
    let v0 = [0.0, 0.0, 0.08, 1.0];

    // ----- flat control (we use a Cartesian-style flat manifold here only
    // for the comparison; the geodesic in spherical-style coordinates on
    // flat space would require its own non-zero Christoffel symbols, so we
    // simply integrate in flat Cartesian instead).
    let flat = MinkowskiFlat::new(Signature::minkowski(4));
    let flat_path = integrate_geodesic(&flat, &[0.0; 4], &[0.5, 0.0, 0.0, 1.0], cfg)
        .expect("flat Minkowski cannot diverge");

    let schw = Schwarzschild { mass: 1.0 };
    let schw_path = match integrate_geodesic(&schw, &x0, &v0, cfg) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Schwarzschild geodesic diverged: {e}");
            std::process::exit(1);
        }
    };

    let scalar = scalar_curvature(&schw, &x0).expect("schwarzschild scalar curvature");

    println!("# Schwarzschild vs flat Minkowski, same initial energy");
    println!();
    println!("flat Minkowski (3,1,0):");
    let last = flat_path.last().unwrap();
    println!(
        "  final tau = {:.3}, pos = {:?}, vel = {:?}",
        last.tau, last.position, last.velocity
    );
    println!();
    println!("Schwarzschild (M=1):");
    let last = schw_path.last().unwrap();
    println!(
        "  final tau = {:.3}, pos = {:?}, vel = {:?}",
        last.tau, last.position, last.velocity
    );
    println!();
    println!("scalar curvature R(r=10, θ=π/2) = {scalar:.4e}  (theory: 0 in vacuum)");
}
