//! Two-time Dichronauts geometry: signature (2, 2, 0).
//!
//! In (2, 2, 0) — Greg Egan's *Dichronauts* setting — there are two
//! "spacelike" and two "timelike" directions. The line element along a
//! geodesic can be positive, negative, or zero depending on the velocity
//! split, which is why the inhabitants of that universe see the world
//! in such an unusual way.
//!
//! Run with `cargo run -p signgeom-core --example dichronauts_geodesic`.

use signgeom_core::{integrate_geodesic, GeodesicConfig, MinkowskiFlat, Signature};

fn main() {
    let m = MinkowskiFlat::new(Signature::dichronauts4());
    let eta = Signature::dichronauts4().canonical_diagonal();
    println!("# Dichronauts (2, 2, 0) geometry — eta diag = {eta:?}");
    println!();

    // Three illustrative velocities: spacelike-dominant, null, timelike-dominant.
    let cases = [
        ("spacelike-dominant", [2.0, 0.0, 0.5, 0.0]),
        ("balanced (≈ null)", [1.0, 0.5, 1.0, 0.5]),
        ("timelike-dominant", [0.0, 0.25, 1.5, 0.0]),
    ];

    let cfg = GeodesicConfig {
        steps: 100,
        proper_time: 3.0,
        ..GeodesicConfig::default()
    };

    for (label, v0) in cases {
        let line: f64 = (0..4).map(|i| eta[i] * v0[i] * v0[i]).sum();
        let path =
            integrate_geodesic(&m, &[0.0; 4], &v0, cfg).expect("flat (2,2,0) cannot diverge");
        let last = path.last().unwrap();
        let kind = if line > 1e-9 {
            "spacelike"
        } else if line < -1e-9 {
            "timelike"
        } else {
            "null"
        };
        println!(
            "{label:>22}: v0={v0:?}  -> ds²/dτ = {line:+.4}  ({kind:>9})  pos@τ={:.2} = {:?}",
            last.tau, last.position
        );
    }

    println!();
    println!("# in (2,2,0) the same kinematics partitions into three causal classes.");
}
