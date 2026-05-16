//! Error type for signgeom-core.

use thiserror::Error;

/// Errors produced by signgeom-core operations.
#[derive(Debug, Error)]
pub enum Error {
    /// The point passed to a manifold has the wrong number of coordinates.
    #[error("dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch {
        /// Expected number of coordinates.
        expected: usize,
        /// Actual number of coordinates.
        actual: usize,
    },

    /// The metric tensor at the requested point was numerically singular
    /// (determinant below tolerance), so it cannot be inverted to compute
    /// Christoffel symbols of the second kind.
    #[error("metric is singular at the requested point (det = {det:e}, tol = {tol:e})")]
    SingularMetric {
        /// Determinant of the metric.
        det: f64,
        /// Numerical tolerance used.
        tol: f64,
    },

    /// A geodesic integration step produced a non-finite value (NaN/inf).
    #[error("geodesic integration diverged at step {step}: {what}")]
    GeodesicDiverged {
        /// Step index at which divergence was detected.
        step: usize,
        /// Description of what went non-finite.
        what: &'static str,
    },

    /// The requested signature is inconsistent with the dimension of the
    /// underlying manifold.
    #[error("signature {p}+{q}+{r} does not match manifold dimension {dim}")]
    SignatureMismatch {
        /// Number of positive eigenvalues.
        p: usize,
        /// Number of negative eigenvalues.
        q: usize,
        /// Number of degenerate eigenvalues.
        r: usize,
        /// Manifold dimension.
        dim: usize,
    },

    /// A configuration option had an out-of-range value.
    #[error("invalid configuration: {0}")]
    Config(&'static str),
}
