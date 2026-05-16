//! Riemann, Ricci, and scalar curvatures.
//!
//! These are derived numerically from the Christoffel symbols. The Riemann
//! tensor of the second kind we compute is
//!
//! ```text
//! R^a_{bcd} = ∂_c Γ^a_{db} − ∂_d Γ^a_{cb} + Γ^a_{ce} Γ^e_{db} − Γ^a_{de} Γ^e_{cb}
//! ```
//!
//! Partial derivatives of the Christoffel symbols are obtained by central
//! differences of the metric, with the same step parameter used everywhere
//! else in the crate.

use crate::christoffel::christoffel_with;
use crate::error::Error;
use crate::manifold::Manifold;
use crate::{DEFAULT_FD_STEP, DEFAULT_SINGULAR_TOL};

/// Riemann curvature tensor `R^a_{bcd}(x)`, indexed `r[a][b][c][d]`.
pub fn riemann<M: Manifold + ?Sized>(
    manifold: &M,
    point: &[f64],
) -> Result<Vec<Vec<Vec<Vec<f64>>>>, Error> {
    riemann_with(manifold, point, DEFAULT_FD_STEP, DEFAULT_SINGULAR_TOL)
}

/// As [`riemann`], with explicit step and tolerance.
pub fn riemann_with<M: Manifold + ?Sized>(
    manifold: &M,
    point: &[f64],
    fd_step: f64,
    singular_tol: f64,
) -> Result<Vec<Vec<Vec<Vec<f64>>>>, Error> {
    let n = manifold.dim();
    if point.len() != n {
        return Err(Error::DimensionMismatch {
            expected: n,
            actual: point.len(),
        });
    }
    let gamma0 = christoffel_with(manifold, point, fd_step, singular_tol)?;

    // dGamma[c][a][b][d] = ∂_c Γ^a_{db}
    let mut dgamma = vec![vec![vec![vec![0.0; n]; n]; n]; n];
    let mut buf = point.to_vec();
    for c in 0..n {
        let orig = buf[c];
        buf[c] = orig + fd_step;
        let g_plus = christoffel_with(manifold, &buf, fd_step, singular_tol)?;
        buf[c] = orig - fd_step;
        let g_minus = christoffel_with(manifold, &buf, fd_step, singular_tol)?;
        buf[c] = orig;
        for a in 0..n {
            for d in 0..n {
                for b in 0..n {
                    dgamma[c][a][b][d] = (g_plus[a][d][b] - g_minus[a][d][b]) / (2.0 * fd_step);
                }
            }
        }
    }

    // R^a_{bcd} = ∂_c Γ^a_{db} − ∂_d Γ^a_{cb} + Γ^a_{ce} Γ^e_{db} − Γ^a_{de} Γ^e_{cb}
    let mut r = vec![vec![vec![vec![0.0; n]; n]; n]; n];
    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                for d in 0..n {
                    let mut acc = dgamma[c][a][b][d] - dgamma[d][a][b][c];
                    for e in 0..n {
                        acc += gamma0[a][c][e] * gamma0[e][d][b];
                        acc -= gamma0[a][d][e] * gamma0[e][c][b];
                    }
                    r[a][b][c][d] = acc;
                }
            }
        }
    }
    Ok(r)
}

/// Ricci curvature tensor `Ric_{bd}(x) = R^a_{bad}`, indexed `ric[b][d]`.
pub fn ricci<M: Manifold + ?Sized>(manifold: &M, point: &[f64]) -> Result<Vec<Vec<f64>>, Error> {
    let r = riemann(manifold, point)?;
    let n = manifold.dim();
    let mut ric = vec![vec![0.0; n]; n];
    for b in 0..n {
        for d in 0..n {
            let mut acc = 0.0;
            for a in 0..n {
                acc += r[a][b][a][d];
            }
            ric[b][d] = acc;
        }
    }
    Ok(ric)
}

/// Scalar curvature `R(x) = g^{bd} Ric_{bd}`.
pub fn scalar_curvature<M: Manifold + ?Sized>(manifold: &M, point: &[f64]) -> Result<f64, Error> {
    let ric = ricci(manifold, point)?;
    let g = manifold.metric(point)?;
    let g_inv = crate::manifold::invert_symmetric(&g, DEFAULT_SINGULAR_TOL)?;
    let n = manifold.dim();
    let mut acc = 0.0;
    for b in 0..n {
        for d in 0..n {
            acc += g_inv[b][d] * ric[b][d];
        }
    }
    Ok(acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flat::MinkowskiFlat;
    use crate::signature::Signature;
    use approx::assert_abs_diff_eq;

    #[test]
    fn flat_space_is_flat_in_all_three_signatures() {
        for sig in [
            Signature::minkowski(4),
            Signature::orthogonal4(),
            Signature::dichronauts4(),
        ] {
            let m = MinkowskiFlat::new(sig);
            let r = riemann(&m, &[0.1, 0.0, -0.1, 0.0]).unwrap();
            for a in 0..4 {
                for b in 0..4 {
                    for c in 0..4 {
                        for d in 0..4 {
                            assert!(
                                r[a][b][c][d].abs() < 1e-6,
                                "R^{a}_{b}{c}{d} = {} for sig {sig:?}",
                                r[a][b][c][d]
                            );
                        }
                    }
                }
            }
            let s = scalar_curvature(&m, &[0.0; 4]).unwrap();
            assert_abs_diff_eq!(s, 0.0, epsilon = 1e-6);
        }
    }
}
