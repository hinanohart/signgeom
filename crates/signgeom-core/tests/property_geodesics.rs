//! Property tests for geodesic integration: in flat space of any signature,
//! the integrated path must remain on a straight line through the origin,
//! and `ds²` must be exactly `(v·v) * τ` (no integration drift).

#![allow(clippy::needless_range_loop)]

use approx::abs_diff_eq;
use proptest::prelude::*;
use signgeom_core::{integrate_geodesic, GeodesicConfig, MinkowskiFlat, Signature};

fn signature_strategy() -> impl Strategy<Value = Signature> {
    prop_oneof![
        Just(Signature::minkowski(4)),
        Just(Signature::orthogonal4()),
        Just(Signature::dichronauts4()),
        Just(Signature::riemannian(3)),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn flat_geodesic_is_a_straight_line_in_every_signature(
        sig in signature_strategy(),
        steps in 50_usize..200,
        proper_time in 0.5_f64..4.0,
    ) {
        let dim = sig.dim();
        let manifold = MinkowskiFlat::new(sig);
        let v0: Vec<f64> = (0..dim).map(|i| 0.5 + 0.13 * i as f64).collect();
        let x0 = vec![0.0; dim];
        let cfg = GeodesicConfig { steps, proper_time, ..GeodesicConfig::default() };
        let path = integrate_geodesic(&manifold, &x0, &v0, cfg).unwrap();
        // Final position should equal v0 * proper_time, to high precision.
        let last = path.last().unwrap();
        for i in 0..dim {
            let expected = v0[i] * proper_time;
            prop_assert!(
                abs_diff_eq!(last.position[i], expected, epsilon = 1e-6),
                "position[{i}] = {} != expected {}",
                last.position[i], expected,
            );
        }
    }

    #[test]
    fn flat_geodesic_constant_velocity(
        sig in signature_strategy(),
        steps in 50_usize..200,
        proper_time in 0.5_f64..4.0,
    ) {
        let dim = sig.dim();
        let m = MinkowskiFlat::new(sig);
        let x0 = vec![0.0; dim];
        let v0: Vec<f64> = (0..dim).map(|i| 0.3 * (i as f64 + 1.0)).collect();
        let cfg = GeodesicConfig { steps, proper_time, ..GeodesicConfig::default() };
        let path = integrate_geodesic(&m, &x0, &v0, cfg).unwrap();
        // Velocity must remain ~constant.
        let last = path.last().unwrap();
        for i in 0..dim {
            prop_assert!(abs_diff_eq!(last.velocity[i], v0[i], epsilon = 1e-9));
        }
    }
}

#[test]
fn integrator_rejects_dimension_mismatch() {
    let m = MinkowskiFlat::new(Signature::minkowski(4));
    let cfg = GeodesicConfig::default();
    let err = integrate_geodesic(&m, &[0.0, 0.0, 0.0], &[1.0, 0.0, 0.0, 0.0], cfg);
    assert!(err.is_err());
}

#[test]
fn integrator_rejects_zero_steps() {
    let m = MinkowskiFlat::new(Signature::riemannian(2));
    let cfg = GeodesicConfig {
        steps: 0,
        ..GeodesicConfig::default()
    };
    let err = integrate_geodesic(&m, &[0.0, 0.0], &[1.0, 0.0], cfg);
    assert!(err.is_err());
}
