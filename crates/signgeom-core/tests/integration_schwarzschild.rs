//! Integration test: the Schwarzschild geometry is Ricci-flat in vacuum, at
//! all points outside the horizon. We sample a handful of points to make
//! sure the numerical pipeline is not biased toward one location.

use approx::assert_abs_diff_eq;
use signgeom_core::{scalar_curvature, Schwarzschild};

#[test]
fn schwarzschild_is_ricci_flat_at_multiple_points() {
    let m = Schwarzschild { mass: 1.0 };
    let pi_over_2 = std::f64::consts::FRAC_PI_2;
    // (r, θ, φ, t) — every r is comfortably outside 2M.
    let points = [
        [5.0, pi_over_2, 0.0, 0.0],
        [7.5, pi_over_2 - 0.2, 1.1, 0.0],
        [15.0, 0.7, -0.5, 4.2],
        [25.0, 1.4, 2.0, -1.0],
    ];
    for p in &points {
        let scalar = scalar_curvature(&m, p).unwrap();
        assert_abs_diff_eq!(scalar, 0.0, epsilon = 5e-3); // numerical tolerance for the dimensional FD
    }
}

#[test]
fn schwarzschild_metric_rejects_inside_horizon_singular_points() {
    use signgeom_core::Manifold;
    let m = Schwarzschild { mass: 1.0 };
    // r = 2M exactly -> singular (1 / (1 - 2M/r) blows up).
    let err = m.metric(&[2.0, std::f64::consts::FRAC_PI_2, 0.0, 0.0]);
    assert!(err.is_err());
}
