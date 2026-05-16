//! Christoffel symbols of the second kind.
//!
//! Given a metric `g_ij(x)` and its first derivatives `∂_k g_ij(x)`, the
//! Levi-Civita connection coefficients are
//!
//! ```text
//! Γ^k_{ij} = (1/2) g^{kl} ( ∂_i g_{jl} + ∂_j g_{il} − ∂_l g_{ij} )
//! ```
//!
//! In flat coordinates of any constant metric they vanish identically — this
//! is the cheapest sanity check, see `tests::flat_minkowski_is_zero`.

use crate::error::Error;
use crate::manifold::{invert_symmetric, Manifold};
use crate::{DEFAULT_FD_STEP, DEFAULT_SINGULAR_TOL};

/// Christoffel symbols of the second kind at `point`, indexed
/// `gamma[k][i][j]`.
///
/// The implementation calls `manifold.metric` and `manifold.metric_partials`
/// once, then performs `O(n^4)` arithmetic. For 4-dimensional manifolds this
/// is well under a microsecond.
pub fn christoffel<M: Manifold + ?Sized>(
    manifold: &M,
    point: &[f64],
) -> Result<Vec<Vec<Vec<f64>>>, Error> {
    christoffel_with(manifold, point, DEFAULT_FD_STEP, DEFAULT_SINGULAR_TOL)
}

/// As [`christoffel`], but lets the caller choose the finite-difference step
/// (only used when the manifold relies on the default `metric_partials`) and
/// the singular-matrix tolerance.
pub fn christoffel_with<M: Manifold + ?Sized>(
    manifold: &M,
    point: &[f64],
    fd_step: f64,
    singular_tol: f64,
) -> Result<Vec<Vec<Vec<f64>>>, Error> {
    let n = manifold.dim();
    if point.len() != n {
        return Err(Error::DimensionMismatch {
            expected: n,
            actual: point.len(),
        });
    }
    let g = manifold.metric(point)?;
    let g_inv = invert_symmetric(&g, singular_tol)?;
    let dg = manifold.metric_partials(point, fd_step)?;
    let mut gamma = vec![vec![vec![0.0; n]; n]; n];
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let mut acc = 0.0;
                for l in 0..n {
                    acc += g_inv[k][l] * (dg[i][j][l] + dg[j][i][l] - dg[l][i][j]);
                }
                gamma[k][i][j] = 0.5 * acc;
            }
        }
    }
    Ok(gamma)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flat::MinkowskiFlat;
    use crate::signature::Signature;

    #[test]
    fn flat_minkowski_is_zero() {
        let m = MinkowskiFlat::new(Signature::minkowski(4));
        let g = christoffel(&m, &[0.1, 0.2, 0.3, 0.4]).unwrap();
        for row in &g {
            for col in row {
                for v in col {
                    assert!(v.abs() < 1e-9, "expected ~0, got {v}");
                }
            }
        }
    }

    #[test]
    fn flat_orthogonal_is_zero() {
        let m = MinkowskiFlat::new(Signature::orthogonal4());
        let g = christoffel(&m, &[0.0, 0.0, 0.0, 0.0]).unwrap();
        for row in &g {
            for col in row {
                for v in col {
                    assert!(v.abs() < 1e-9);
                }
            }
        }
    }

    #[test]
    fn flat_dichronauts_is_zero() {
        let m = MinkowskiFlat::new(Signature::dichronauts4());
        let g = christoffel(&m, &[1.0, -1.0, 0.5, -0.5]).unwrap();
        for row in &g {
            for col in row {
                for v in col {
                    assert!(v.abs() < 1e-9);
                }
            }
        }
    }
}
