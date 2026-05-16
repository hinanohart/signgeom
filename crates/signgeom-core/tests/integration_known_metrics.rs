//! Integration tests against closed-form curvature on known curved and flat
//! manifolds. These guard the curvature pipeline against quiet sign / index
//! regressions: the Schwarzschild test by itself only proves the Ricci-flat
//! case, so we add positive-curvature (2-sphere) and negative-curvature
//! (2-hyperbolic) reference manifolds, plus the flat `(4,0,0)` "Orthogonal"
//! and `(2,2,0)` "Dichronauts" cases.

use approx::assert_abs_diff_eq;
use signgeom_core::{scalar_curvature, Error, Manifold, MetricTensor, MinkowskiFlat, Signature};

/// 2-sphere of radius 1 in polar coordinates `(θ, φ)`.
///
/// `g = diag(1, sin²θ)`. The closed-form scalar curvature is `R = 2/r² = 2`.
struct UnitTwoSphere;

impl Manifold for UnitTwoSphere {
    fn dim(&self) -> usize {
        2
    }
    fn signature(&self) -> Signature {
        Signature::riemannian(2)
    }
    fn metric(&self, point: &[f64]) -> Result<MetricTensor, Error> {
        if point.len() != 2 {
            return Err(Error::DimensionMismatch {
                expected: 2,
                actual: point.len(),
            });
        }
        let theta = point[0];
        let sin_t = theta.sin();
        Ok(vec![vec![1.0, 0.0], vec![0.0, sin_t * sin_t]])
    }
}

/// 2-hyperbolic plane in polar coordinates `(ρ, φ)`.
///
/// `g = diag(1, sinh²ρ)`. The closed-form scalar curvature is `R = -2`.
struct HyperbolicPlane;

impl Manifold for HyperbolicPlane {
    fn dim(&self) -> usize {
        2
    }
    fn signature(&self) -> Signature {
        Signature::riemannian(2)
    }
    fn metric(&self, point: &[f64]) -> Result<MetricTensor, Error> {
        if point.len() != 2 {
            return Err(Error::DimensionMismatch {
                expected: 2,
                actual: point.len(),
            });
        }
        let rho = point[0];
        let sh = rho.sinh();
        Ok(vec![vec![1.0, 0.0], vec![0.0, sh * sh]])
    }
}

#[test]
fn two_sphere_scalar_curvature_is_plus_two() {
    let m = UnitTwoSphere;
    // Equator avoids the polar coordinate singularity at θ = 0, π.
    let r = scalar_curvature(&m, &[std::f64::consts::FRAC_PI_2, 0.0]).unwrap();
    // Central-difference error on a fourth-derivative quantity at fd=1e-3 is
    // typically O(1e-4 .. 1e-3); we keep the bound generous.
    assert_abs_diff_eq!(r, 2.0, epsilon = 5e-3);
}

#[test]
fn two_sphere_scalar_curvature_is_plus_two_off_equator() {
    let m = UnitTwoSphere;
    // A non-equatorial point: the same closed-form value applies everywhere.
    let r = scalar_curvature(&m, &[1.0, 0.5]).unwrap();
    assert_abs_diff_eq!(r, 2.0, epsilon = 5e-3);
}

#[test]
fn hyperbolic_plane_scalar_curvature_is_minus_two() {
    let m = HyperbolicPlane;
    // ρ = 1.0 — well away from the origin where sinh(ρ) ≈ 0.
    let r = scalar_curvature(&m, &[1.0, 0.0]).unwrap();
    assert_abs_diff_eq!(r, -2.0, epsilon = 5e-3);
}

#[test]
fn flat_orthogonal_4_0_0_is_ricci_flat() {
    // The (4,0,0) "Orthogonal" flat case: trivial geometry, but the curvature
    // pipeline must not produce sign/index ghosts when q == 0.
    let m = MinkowskiFlat::new(Signature::orthogonal4());
    let r = scalar_curvature(&m, &[0.5, -0.2, 0.0, 1.1]).unwrap();
    assert_abs_diff_eq!(r, 0.0, epsilon = 1e-6);
}

#[test]
fn flat_dichronauts_2_2_0_is_ricci_flat() {
    // The (2,2,0) split-signature flat case: two times, two spaces.
    let m = MinkowskiFlat::new(Signature::dichronauts4());
    let r = scalar_curvature(&m, &[0.5, -0.2, 0.0, 1.1]).unwrap();
    assert_abs_diff_eq!(r, 0.0, epsilon = 1e-6);
}

#[test]
fn flat_riemannian_5_dim_is_ricci_flat() {
    // Higher-dimensional Riemannian flat case: exercises arbitrary dim.
    let m = MinkowskiFlat::new(Signature::riemannian(5));
    let r = scalar_curvature(&m, &[0.0; 5]).unwrap();
    assert_abs_diff_eq!(r, 0.0, epsilon = 1e-6);
}
