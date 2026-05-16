//! Schwarzschild metric — the canonical non-trivial test case.
//!
//! Coordinates are `(r, θ, φ, t)` (index `0..4`) in mostly-plus signature
//! `(3, 1, 0)` with the timelike coordinate placed *last* so that the
//! canonical diagonal `[+1, +1, +1, −1]` reflects the metric's sign pattern.
//! The line element is
//!
//! ```text
//! ds² = −(1 − 2M/r) dt² + dr²/(1 − 2M/r) + r² dθ² + r² sin²θ dφ²
//! ```
//!
//! with `c = G = 1`. `g_{rr} = 1/(1−2M/r)`, `g_{θθ} = r²`,
//! `g_{φφ} = r² sin²θ`, `g_{tt} = −(1−2M/r)`. Outside the horizon
//! (`r > 2M`) the scalar curvature is identically zero, which is the
//! textbook check used in the integration tests.

use crate::error::Error;
use crate::manifold::{Manifold, MetricTensor};
use crate::signature::Signature;

/// Schwarzschild manifold of mass `M` (geometrised units).
#[derive(Debug, Clone, Copy)]
pub struct Schwarzschild {
    /// Mass parameter.
    pub mass: f64,
}

impl Schwarzschild {
    /// Schwarzschild radius `2M` in geometrised units.
    #[must_use]
    pub fn horizon_radius(&self) -> f64 {
        2.0 * self.mass
    }
}

impl Manifold for Schwarzschild {
    fn dim(&self) -> usize {
        4
    }

    fn signature(&self) -> Signature {
        // Last coordinate is t; we keep "mostly plus" by placing the
        // negative eigenvalue at index 3.
        Signature::new(3, 1, 0)
    }

    fn metric(&self, point: &[f64]) -> Result<MetricTensor, Error> {
        if point.len() != 4 {
            return Err(Error::DimensionMismatch {
                expected: 4,
                actual: point.len(),
            });
        }
        // (r, θ, φ, t)
        let r = point[0];
        let theta = point[1];
        let _phi = point[2];
        let _t = point[3];

        if r.abs() < 1e-9 {
            return Err(Error::SingularMetric {
                det: 0.0,
                tol: 1e-12,
            });
        }
        let f = 1.0 - 2.0 * self.mass / r;
        if f.abs() < 1e-12 {
            return Err(Error::SingularMetric {
                det: 0.0,
                tol: 1e-12,
            });
        }

        let mut g = vec![vec![0.0; 4]; 4];
        g[0][0] = 1.0 / f; // g_rr
        g[1][1] = r * r; // g_θθ
        g[2][2] = r * r * theta.sin().powi(2); // g_φφ
        g[3][3] = -f; // g_tt (mostly-plus, so timelike sign is negative)
        Ok(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curvature::scalar_curvature;
    use approx::assert_abs_diff_eq;

    #[test]
    fn signature_is_3_plus_1() {
        let s = Schwarzschild { mass: 1.0 }.signature();
        assert_eq!(s, Signature::minkowski(4));
    }

    #[test]
    fn vacuum_ricci_scalar_is_zero_outside_the_horizon() {
        let s = Schwarzschild { mass: 1.0 };
        // Pick a sensible point: r = 10M, θ = π/2 (equatorial), φ = 0, t = 0.
        let scalar = scalar_curvature(&s, &[10.0, std::f64::consts::FRAC_PI_2, 0.0, 0.0]).unwrap();
        // Numerical differentiation is the main source of noise here; ~1e-4
        // is a realistic bound with the default fd_step.
        assert_abs_diff_eq!(scalar, 0.0, epsilon = 5e-4);
    }
}
