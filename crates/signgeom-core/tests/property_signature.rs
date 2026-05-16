//! Property tests for the Signature value type.

use proptest::prelude::*;
use signgeom_core::Signature;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn dim_is_p_plus_q_plus_r(p in 0_usize..6, q in 0_usize..6, r in 0_usize..6) {
        let s = Signature::new(p, q, r);
        prop_assert_eq!(s.dim(), p + q + r);
    }

    #[test]
    fn canonical_diagonal_has_right_counts(p in 0_usize..6, q in 0_usize..6, r in 0_usize..6) {
        prop_assume!(p + q + r > 0);
        let s = Signature::new(p, q, r);
        let d = s.canonical_diagonal();
        prop_assert_eq!(d.len(), p + q + r);
        prop_assert_eq!(d.iter().filter(|&&v| v == 1.0).count(), p);
        prop_assert_eq!(d.iter().filter(|&&v| v == -1.0).count(), q);
        prop_assert_eq!(d.iter().filter(|&&v| v == 0.0).count(), r);
    }
}

#[test]
fn riemannian_classification() {
    assert!(Signature::riemannian(7).is_riemannian());
    assert!(!Signature::minkowski(4).is_riemannian());
    assert!(!Signature::dichronauts4().is_riemannian());
}
