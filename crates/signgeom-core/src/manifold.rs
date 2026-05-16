//! The [`Manifold`] trait and a thin matrix newtype.
//!
//! Implementations supply a metric tensor `g_ij(x)`. The trait provides a
//! default numerical implementation of `metric_partials` based on
//! second-order central finite differences, so simple manifolds can be added
//! by writing only the metric.

use crate::error::Error;
use crate::signature::Signature;

/// A square symmetric matrix used as a metric tensor.
///
/// Indexed `m[i][j]` with `i, j ∈ 0..dim`. Stored row-major as a `Vec<Vec<f64>>`
/// for legibility; the geometry kernels are dimension-erased, so performance
/// is not the priority of this newtype.
pub type MetricTensor = Vec<Vec<f64>>;

/// A (possibly pseudo-)Riemannian manifold.
///
/// The minimum required behaviour is to return the metric tensor at a point.
/// Partial derivatives default to central finite differences.
pub trait Manifold {
    /// Manifold dimension.
    fn dim(&self) -> usize;

    /// Signature of the metric (must satisfy `signature.dim() == dim()`).
    fn signature(&self) -> Signature;

    /// Metric tensor `g_ij(x)` at the given point.
    ///
    /// `point.len()` must equal [`Self::dim`].
    fn metric(&self, point: &[f64]) -> Result<MetricTensor, Error>;

    /// Partial derivatives `∂_k g_ij(x)`, returned as `out[k][i][j]`.
    ///
    /// The default implementation uses second-order central differences with
    /// step `h`. Override this when an analytic derivative is available — it
    /// improves both precision and runtime by an order of magnitude.
    fn metric_partials(&self, point: &[f64], h: f64) -> Result<Vec<Vec<Vec<f64>>>, Error> {
        let n = self.dim();
        if point.len() != n {
            return Err(Error::DimensionMismatch {
                expected: n,
                actual: point.len(),
            });
        }
        let mut out = vec![vec![vec![0.0; n]; n]; n];
        let mut buf = point.to_vec();
        for k in 0..n {
            let orig = buf[k];
            buf[k] = orig + h;
            let g_plus = self.metric(&buf)?;
            buf[k] = orig - h;
            let g_minus = self.metric(&buf)?;
            buf[k] = orig;
            for i in 0..n {
                for j in 0..n {
                    out[k][i][j] = (g_plus[i][j] - g_minus[i][j]) / (2.0 * h);
                }
            }
        }
        Ok(out)
    }
}

/// Solve `A x = b` for a symmetric matrix `A` using Gauss–Jordan with partial
/// pivoting. Returns `Err(SingularMetric)` when the determinant falls below
/// `tol`.
pub(crate) fn solve_symmetric(a: &MetricTensor, b: &[f64], tol: f64) -> Result<Vec<f64>, Error> {
    let n = a.len();
    debug_assert_eq!(b.len(), n);
    let mut aug = vec![vec![0.0; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            aug[i][j] = a[i][j];
        }
        aug[i][n] = b[i];
    }
    let mut det_sign = 1.0;
    let mut det_mag = 1.0;
    for col in 0..n {
        // Partial pivoting.
        let mut pivot_row = col;
        let mut pivot_val = aug[col][col].abs();
        for r in (col + 1)..n {
            if aug[r][col].abs() > pivot_val {
                pivot_val = aug[r][col].abs();
                pivot_row = r;
            }
        }
        if pivot_val < tol {
            return Err(Error::SingularMetric { det: 0.0, tol });
        }
        if pivot_row != col {
            aug.swap(col, pivot_row);
            det_sign = -det_sign;
        }
        let pivot = aug[col][col];
        det_mag *= pivot.abs();
        for r in 0..n {
            if r != col {
                let factor = aug[r][col] / pivot;
                for c in col..=n {
                    aug[r][c] -= factor * aug[col][c];
                }
            }
        }
    }
    let det = det_sign * det_mag;
    if det.abs() < tol {
        return Err(Error::SingularMetric { det, tol });
    }
    Ok((0..n).map(|i| aug[i][n] / aug[i][i]).collect())
}

/// Invert a small symmetric matrix. Returns `Err(SingularMetric)` if the
/// determinant is below `tol`.
pub(crate) fn invert_symmetric(a: &MetricTensor, tol: f64) -> Result<MetricTensor, Error> {
    let n = a.len();
    let mut inv = vec![vec![0.0; n]; n];
    for col in 0..n {
        let mut e = vec![0.0; n];
        e[col] = 1.0;
        let x = solve_symmetric(a, &e, tol)?;
        for row in 0..n {
            inv[row][col] = x[row];
        }
    }
    Ok(inv)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    fn diag(d: &[f64]) -> MetricTensor {
        let n = d.len();
        let mut m = vec![vec![0.0; n]; n];
        for i in 0..n {
            m[i][i] = d[i];
        }
        m
    }

    #[test]
    fn invert_minkowski_is_minkowski() {
        let g = diag(&[1.0, 1.0, 1.0, -1.0]);
        let inv = invert_symmetric(&g, 1e-12).unwrap();
        for i in 0..4 {
            for j in 0..4 {
                let expect = if i == j { g[i][j] } else { 0.0 };
                assert_abs_diff_eq!(inv[i][j], expect, epsilon = 1e-12);
            }
        }
    }

    #[test]
    fn invert_2x2_general() {
        let a: MetricTensor = vec![vec![4.0, 1.0], vec![1.0, 3.0]];
        let inv = invert_symmetric(&a, 1e-12).unwrap();
        // a * inv = I
        for i in 0..2 {
            for j in 0..2 {
                let mut acc = 0.0;
                for k in 0..2 {
                    acc += a[i][k] * inv[k][j];
                }
                let expect = if i == j { 1.0 } else { 0.0 };
                assert_abs_diff_eq!(acc, expect, epsilon = 1e-12);
            }
        }
    }

    #[test]
    fn singular_metric_is_rejected() {
        let a: MetricTensor = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let err = invert_symmetric(&a, 1e-9).unwrap_err();
        match err {
            Error::SingularMetric { .. } => (),
            other => panic!("expected SingularMetric, got {other}"),
        }
    }
}
