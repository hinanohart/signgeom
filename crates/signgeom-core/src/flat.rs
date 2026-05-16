//! Flat manifold with a constant diagonal metric.
//!
//! This is the simplest possible [`Manifold`] implementation: it ignores the
//! point and always returns the canonical Minkowski-style diagonal metric
//! `diag([+1; p], [−1; q], [0; r])`. It is useful as a sanity check (all
//! curvatures vanish) and as a backdrop for non-relativistic physics on
//! arbitrary signatures.

use crate::error::Error;
use crate::manifold::{Manifold, MetricTensor};
use crate::signature::Signature;

/// Flat manifold with a constant metric of given signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinkowskiFlat {
    signature: Signature,
}

impl MinkowskiFlat {
    /// Construct a flat manifold with the given signature.
    #[must_use]
    pub const fn new(signature: Signature) -> Self {
        Self { signature }
    }
}

impl Manifold for MinkowskiFlat {
    fn dim(&self) -> usize {
        self.signature.dim()
    }

    fn signature(&self) -> Signature {
        self.signature
    }

    fn metric(&self, point: &[f64]) -> Result<MetricTensor, Error> {
        let n = self.dim();
        if point.len() != n {
            return Err(Error::DimensionMismatch {
                expected: n,
                actual: point.len(),
            });
        }
        let diag = self.signature.canonical_diagonal();
        let mut m = vec![vec![0.0; n]; n];
        for i in 0..n {
            m[i][i] = diag[i];
        }
        Ok(m)
    }

    fn metric_partials(&self, point: &[f64], _h: f64) -> Result<Vec<Vec<Vec<f64>>>, Error> {
        let n = self.dim();
        if point.len() != n {
            return Err(Error::DimensionMismatch {
                expected: n,
                actual: point.len(),
            });
        }
        // Constant metric, so all derivatives are exactly zero — short-circuit
        // the central-difference default to avoid round-off noise leaking into
        // the curvature tests.
        Ok(vec![vec![vec![0.0; n]; n]; n])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dim_matches_signature() {
        let m = MinkowskiFlat::new(Signature::minkowski(4));
        assert_eq!(m.dim(), 4);
    }

    #[test]
    fn metric_dimension_mismatch_is_an_error() {
        let m = MinkowskiFlat::new(Signature::riemannian(3));
        let err = m.metric(&[0.0, 0.0]).unwrap_err();
        match err {
            Error::DimensionMismatch { expected, actual } => {
                assert_eq!(expected, 3);
                assert_eq!(actual, 2);
            }
            other => panic!("expected DimensionMismatch, got {other}"),
        }
    }

    #[test]
    fn metric_partials_are_exactly_zero() {
        let m = MinkowskiFlat::new(Signature::dichronauts4());
        let dg = m.metric_partials(&[0.0, 0.0, 0.0, 0.0], 1e-4).unwrap();
        for k in 0..4 {
            for i in 0..4 {
                for j in 0..4 {
                    assert_eq!(dg[k][i][j], 0.0);
                }
            }
        }
    }
}
