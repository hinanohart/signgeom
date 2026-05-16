# Metric signatures

A *metric signature* `(p, q, r)` describes the signs of the eigenvalues of a
symmetric bilinear form `g` on a real `n`-dimensional vector space.
Sylvester's law of inertia (1852) guarantees that `(p, q, r)` is an
intrinsic invariant: any basis change preserves the count of positive,
negative and zero eigenvalues. signgeom takes this triple as the *first*
piece of data when describing a geometry.

| Signature | Familiar name | Example |
|---|---|---|
| `(n, 0, 0)` | Riemannian | Euclidean `n`-space, ML embeddings |
| `(n−1, 1, 0)` | Lorentzian, mostly-plus | special relativity, GR |
| `(4, 0, 0)` | Egan's "Orthogonal" | four spacelike directions, no light cone |
| `(2, 2, 0)` | split / neutral | Egan's "Dichronauts" |
| `(n, 0, r)` with `r > 0` | degenerate | Galilean / Newton–Cartan |

## Canonical diagonal

For a non-degenerate signature the canonical "flat" metric is
`diag(+1, …, +1, −1, …, −1, 0, …, 0)` with `p` positive entries, `q`
negative, and `r` zero. signgeom exposes this as
`Signature::canonical_diagonal`:

```rust,no_run
use signgeom_core::Signature;
let d = Signature::dichronauts4().canonical_diagonal();
assert_eq!(d, vec![1.0, 1.0, -1.0, -1.0]);
```

## Why "mostly plus"?

In Minkowski signature, both `(p, q) = (1, 3)` (mostly-minus) and
`(p, q) = (3, 1)` (mostly-plus) are used in the literature. signgeom
chooses mostly-plus throughout: the time-like direction sits at the *last*
index. There are two reasons:

1. It places the unique-sign axis at the same position across `(3, 1)`,
   `(2, 2)` (last two indices), `(4, 0)` (no unique axis), and the
   degenerate `(n, 0, r)` (last `r` indices are degenerate). Iterators
   over coordinates read uniformly.
2. The canonical diagonal `[+1, +1, +1, −1]` makes the four-position the
   "odd one out", which is a useful mnemonic in pedagogy.

## What signature is *not*

- It is not the *dimension* of the manifold; that is `p + q + r`.
- It is not the *signature* in the sense of "type signature" or "function
  signature"; the word is overloaded.
- It is not (by itself) a frame or a basis. A frame chooses a particular
  ordered orthonormal basis adapted to the diagonal; signgeom's kernels
  do not require one, but the canonical-diagonal helper provides one
  when needed.
