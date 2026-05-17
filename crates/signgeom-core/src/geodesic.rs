//! Geodesic integration.
//!
//! The geodesic equation on a manifold with Christoffel symbols `Γ^k_{ij}` is
//!
//! ```text
//! d²x^k/dτ² + Γ^k_{ij} (dx^i/dτ) (dx^j/dτ) = 0
//! ```
//!
//! We integrate the equivalent first-order system `(x, v)` with classical
//! fourth-order Runge–Kutta. The integrator allocates a path of length
//! `steps + 1`; if the user only wants the endpoint, that is `path.last()`.

use crate::christoffel::christoffel_with;
use crate::error::Error;
use crate::manifold::Manifold;

/// Configuration for [`integrate_geodesic`].
#[derive(Debug, Clone, Copy)]
pub struct GeodesicConfig {
    /// Number of RK4 steps. Must be ≥ 1.
    pub steps: usize,
    /// Total proper time / affine parameter range. Sign is significant.
    pub proper_time: f64,
    /// Finite-difference step used inside `christoffel`.
    pub fd_step: f64,
    /// Singular-metric tolerance.
    pub singular_tol: f64,
    /// Abort if any coordinate exceeds this magnitude.
    pub blowup_threshold: f64,
}

impl Default for GeodesicConfig {
    fn default() -> Self {
        Self {
            steps: 1000,
            proper_time: 1.0,
            fd_step: crate::DEFAULT_FD_STEP,
            singular_tol: crate::DEFAULT_SINGULAR_TOL,
            blowup_threshold: 1e9,
        }
    }
}

/// A single sample along a geodesic.
#[derive(Debug, Clone)]
pub struct GeodesicState {
    /// Position vector (length = manifold dim).
    pub position: Vec<f64>,
    /// Velocity vector (length = manifold dim).
    pub velocity: Vec<f64>,
    /// Affine parameter (proper time) at this sample.
    pub tau: f64,
}

/// Integrate a geodesic with initial position and velocity, returning the
/// full path of length `config.steps + 1`.
pub fn integrate_geodesic<M: Manifold + ?Sized>(
    manifold: &M,
    initial_position: &[f64],
    initial_velocity: &[f64],
    config: GeodesicConfig,
) -> Result<Vec<GeodesicState>, Error> {
    let n = manifold.dim();
    if initial_position.len() != n {
        return Err(Error::DimensionMismatch {
            expected: n,
            actual: initial_position.len(),
        });
    }
    if initial_velocity.len() != n {
        return Err(Error::DimensionMismatch {
            expected: n,
            actual: initial_velocity.len(),
        });
    }
    if config.steps == 0 {
        return Err(Error::Config("steps must be >= 1"));
    }

    let dt = config.proper_time / (config.steps as f64);
    let mut path = Vec::with_capacity(config.steps + 1);
    path.push(GeodesicState {
        position: initial_position.to_vec(),
        velocity: initial_velocity.to_vec(),
        tau: 0.0,
    });

    let mut x = initial_position.to_vec();
    let mut v = initial_velocity.to_vec();

    for step in 0..config.steps {
        let k1 = rk_derivative(manifold, &x, &v, &config)?;
        let (x2, v2) = scale_add(&x, &v, &k1.0, &k1.1, 0.5 * dt);
        let k2 = rk_derivative(manifold, &x2, &v2, &config)?;
        let (x3, v3) = scale_add(&x, &v, &k2.0, &k2.1, 0.5 * dt);
        let k3 = rk_derivative(manifold, &x3, &v3, &config)?;
        let (x4, v4) = scale_add(&x, &v, &k3.0, &k3.1, dt);
        let k4 = rk_derivative(manifold, &x4, &v4, &config)?;

        for i in 0..n {
            x[i] += dt / 6.0 * (k1.0[i] + 2.0 * k2.0[i] + 2.0 * k3.0[i] + k4.0[i]);
            v[i] += dt / 6.0 * (k1.1[i] + 2.0 * k2.1[i] + 2.0 * k3.1[i] + k4.1[i]);
            if !x[i].is_finite() || !v[i].is_finite() {
                return Err(Error::GeodesicDiverged {
                    step: step + 1,
                    what: "non-finite state",
                });
            }
            if x[i].abs() > config.blowup_threshold {
                return Err(Error::GeodesicDiverged {
                    step: step + 1,
                    what: "position exceeded blowup_threshold",
                });
            }
        }

        path.push(GeodesicState {
            position: x.clone(),
            velocity: v.clone(),
            tau: dt * (step as f64 + 1.0),
        });
    }

    Ok(path)
}

fn rk_derivative<M: Manifold + ?Sized>(
    manifold: &M,
    x: &[f64],
    v: &[f64],
    cfg: &GeodesicConfig,
) -> Result<(Vec<f64>, Vec<f64>), Error> {
    let gamma = christoffel_with(manifold, x, cfg.fd_step, cfg.singular_tol)?;
    let n = x.len();
    let dx = v.to_vec();
    let mut dv = vec![0.0; n];
    for k in 0..n {
        let mut acc = 0.0;
        for i in 0..n {
            for j in 0..n {
                acc += gamma[k][i][j] * v[i] * v[j];
            }
        }
        dv[k] = -acc;
    }
    Ok((dx, dv))
}

fn scale_add(x: &[f64], v: &[f64], dx: &[f64], dv: &[f64], h: f64) -> (Vec<f64>, Vec<f64>) {
    let n = x.len();
    let mut x_out = vec![0.0; n];
    let mut v_out = vec![0.0; n];
    for i in 0..n {
        x_out[i] = x[i] + h * dx[i];
        v_out[i] = v[i] + h * dv[i];
    }
    (x_out, v_out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flat::MinkowskiFlat;
    use crate::signature::Signature;
    use approx::assert_abs_diff_eq;

    #[test]
    fn straight_line_in_flat_minkowski() {
        let m = MinkowskiFlat::new(Signature::minkowski(4));
        let cfg = GeodesicConfig {
            steps: 200,
            proper_time: 5.0,
            ..GeodesicConfig::default()
        };
        let path =
            integrate_geodesic(&m, &[0.0, 0.0, 0.0, 0.0], &[0.5, 0.0, 0.0, 1.0], cfg).unwrap();
        let last = path.last().unwrap();
        assert_abs_diff_eq!(last.position[0], 2.5, epsilon = 1e-6);
        assert_abs_diff_eq!(last.position[3], 5.0, epsilon = 1e-6);
    }

    #[test]
    fn straight_line_in_orthogonal_4d() {
        // (4,0,0) — light cones do not exist; this is just a straight line.
        let m = MinkowskiFlat::new(Signature::orthogonal4());
        let cfg = GeodesicConfig {
            steps: 100,
            proper_time: 1.0,
            ..GeodesicConfig::default()
        };
        let path = integrate_geodesic(&m, &[0.0; 4], &[1.0, 1.0, 1.0, 1.0], cfg).unwrap();
        let last = path.last().unwrap();
        for i in 0..4 {
            assert_abs_diff_eq!(last.position[i], 1.0, epsilon = 1e-6);
        }
    }
}
