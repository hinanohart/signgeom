//! Metric signatures.
//!
//! A *signature* is the triple `(p, q, r)` describing how many of the
//! eigenvalues of a (possibly degenerate) bilinear form are positive,
//! negative, or zero. The convention used here is "mostly-plus" for
//! Lorentzian signatures, i.e. `(p, 1, 0)` means one timelike direction.
//!
//! The type is a plain value (`Copy` + `Eq`) so that it can be used as a
//! `const`-time tag, embedded in shader uniforms, or compared in tests.

/// Metric signature `(p, q, r)`.
///
/// - `p` — number of positive eigenvalues
/// - `q` — number of negative eigenvalues
/// - `r` — number of degenerate (zero) eigenvalues
///
/// The dimension of the manifold is `p + q + r`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Signature {
    /// Number of positive eigenvalues.
    pub p: usize,
    /// Number of negative eigenvalues.
    pub q: usize,
    /// Number of zero (degenerate) eigenvalues.
    pub r: usize,
}

impl Signature {
    /// Construct a signature from its three components.
    #[must_use]
    pub const fn new(p: usize, q: usize, r: usize) -> Self {
        Self { p, q, r }
    }

    /// Dimension of the manifold.
    #[must_use]
    pub const fn dim(&self) -> usize {
        self.p + self.q + self.r
    }

    /// Euclidean (Riemannian) signature `(n, 0, 0)`.
    #[must_use]
    pub const fn riemannian(n: usize) -> Self {
        Self::new(n, 0, 0)
    }

    /// Lorentzian signature `(n−1, 1, 0)` in mostly-plus convention.
    ///
    /// Requires `n >= 1`. For `n = 4` this is the signature of ordinary
    /// general relativity.
    #[must_use]
    pub const fn minkowski(n: usize) -> Self {
        assert!(n >= 1, "Minkowski signature requires n >= 1");
        Self::new(n - 1, 1, 0)
    }

    /// "Orthogonal" 4-signature `(4, 0, 0)` — the geometry of Greg Egan's
    /// *Orthogonal* trilogy where there is no Lorentzian split.
    #[must_use]
    pub const fn orthogonal4() -> Self {
        Self::new(4, 0, 0)
    }

    /// "Dichronauts" 4-signature `(2, 2, 0)` — the split-signature geometry
    /// of Greg Egan's *Dichronauts*.
    #[must_use]
    pub const fn dichronauts4() -> Self {
        Self::new(2, 2, 0)
    }

    /// Galilean / Newton–Cartan signature `(n, 0, 1)` — one degenerate
    /// direction representing absolute time.
    #[must_use]
    pub const fn galilean(n: usize) -> Self {
        Self::new(n, 0, 1)
    }

    /// Whether the metric is positive-definite (no negative or zero
    /// directions).
    #[must_use]
    pub const fn is_riemannian(&self) -> bool {
        self.q == 0 && self.r == 0
    }

    /// Whether the metric has exactly one timelike direction and no
    /// degenerate directions.
    #[must_use]
    pub const fn is_lorentzian(&self) -> bool {
        self.q == 1 && self.r == 0
    }

    /// Whether the metric is non-degenerate (`r == 0`).
    #[must_use]
    pub const fn is_non_degenerate(&self) -> bool {
        self.r == 0
    }

    /// Diagonal entries of the canonical Minkowski-style flat metric, in the
    /// order `[+1; p] ++ [−1; q] ++ [0; r]`.
    #[must_use]
    pub fn canonical_diagonal(&self) -> Vec<f64> {
        let n = self.dim();
        let mut diag = vec![0.0; n];
        for i in 0..self.p {
            diag[i] = 1.0;
        }
        for i in self.p..(self.p + self.q) {
            diag[i] = -1.0;
        }
        diag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dim_is_sum() {
        assert_eq!(Signature::new(3, 1, 0).dim(), 4);
        assert_eq!(Signature::new(2, 2, 1).dim(), 5);
    }

    #[test]
    fn named_signatures_match_their_definitions() {
        assert_eq!(Signature::riemannian(3), Signature::new(3, 0, 0));
        assert_eq!(Signature::minkowski(4), Signature::new(3, 1, 0));
        assert_eq!(Signature::orthogonal4(), Signature::new(4, 0, 0));
        assert_eq!(Signature::dichronauts4(), Signature::new(2, 2, 0));
        assert_eq!(Signature::galilean(3), Signature::new(3, 0, 1));
    }

    #[test]
    fn classification_predicates() {
        assert!(Signature::riemannian(4).is_riemannian());
        assert!(!Signature::minkowski(4).is_riemannian());
        assert!(Signature::minkowski(4).is_lorentzian());
        assert!(!Signature::dichronauts4().is_lorentzian());
        assert!(Signature::dichronauts4().is_non_degenerate());
        assert!(!Signature::galilean(3).is_non_degenerate());
    }

    #[test]
    fn canonical_diagonal_minkowski_3_plus_1() {
        let d = Signature::minkowski(4).canonical_diagonal();
        assert_eq!(d, vec![1.0, 1.0, 1.0, -1.0]);
    }

    #[test]
    fn canonical_diagonal_dichronauts() {
        let d = Signature::dichronauts4().canonical_diagonal();
        assert_eq!(d, vec![1.0, 1.0, -1.0, -1.0]);
    }
}
