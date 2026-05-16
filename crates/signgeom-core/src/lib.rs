//! signgeom-core
//!
//! Signature-parametric Riemannian and pseudo-Riemannian geometry.
//!
//! The crate is organised around three ideas:
//!
//! 1. A [`Signature`] value `(p, q, r)` encodes the signs of the eigenvalues
//!    of the metric (positive, negative, degenerate). The same code handles
//!    Euclidean `(n,0,0)`, Minkowski `(n−1,1,0)`, "Orthogonal" `(4,0,0)`
//!    (the geometry of Greg Egan's *Orthogonal* trilogy), and split
//!    `(2,2,0)` (*Dichronauts*) geometries.
//! 2. A [`Manifold`] trait exposes a metric tensor `g_ij(x)` and (optionally)
//!    its partial derivatives. Default implementations use second-order
//!    central differences when an analytic derivative is not provided.
//! 3. Pure functions on `dyn Manifold` (or generic `M: Manifold`) compute
//!    Christoffel symbols, Riemann/Ricci/scalar curvatures and integrate
//!    geodesics with RK4.
//!
//! The kernel is `no_std`-compatible in spirit (no I/O, no global state) and
//! depends only on `num-traits` plus a thin error type.
//!
//! ## Quick example
//!
//! ```
//! use signgeom_core::{Signature, MinkowskiFlat, Manifold, christoffel};
//!
//! // 3+1 Minkowski space-time, mostly-plus.
//! let m = MinkowskiFlat::new(Signature::minkowski(4));
//! let gamma = christoffel(&m, &[0.0, 0.0, 0.0, 0.0]).unwrap();
//! // Flat space-time has vanishing Christoffel symbols.
//! for row in &gamma {
//!     for col in row {
//!         for v in col { assert!(v.abs() < 1e-9); }
//!     }
//! }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]
// Tensor maths reads naturally with nested index loops on small Vec<Vec<…>>
// matrices; the alternative iterator+enumerate chains hurt readability without
// changing performance for `n ≤ 6`. v0.2 will migrate to `nalgebra::DMatrix`
// once benchmarks justify the API churn (decision-log D8).
#![allow(clippy::needless_range_loop, clippy::type_complexity)]

/// Default finite-difference step used by Christoffel and curvature
/// numerical kernels. Chosen so that the central-difference truncation
/// error is around `1e-6` for well-behaved metrics with derivatives of
/// O(1). Override via the `*_with` variants of each function.
pub(crate) const DEFAULT_FD_STEP: f64 = 1e-3;

/// Default tolerance for declaring a metric tensor singular when its
/// determinant magnitude drops below this value during Gauss-Jordan
/// inversion. Override via the `*_with` variants of each function.
pub(crate) const DEFAULT_SINGULAR_TOL: f64 = 1e-12;

mod christoffel;
mod curvature;
mod error;
mod flat;
mod geodesic;
mod manifold;
mod schwarzschild;
mod signature;

pub use christoffel::christoffel;
pub use curvature::{ricci, riemann, scalar_curvature};
pub use error::Error;
pub use flat::MinkowskiFlat;
pub use geodesic::{integrate_geodesic, GeodesicConfig, GeodesicState};
pub use manifold::{Manifold, MetricTensor};
pub use schwarzschild::Schwarzschild;
pub use signature::Signature;
