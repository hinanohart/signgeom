# Introduction

signgeom is a small Rust library for *signature-parametric* (pseudo-)
Riemannian geometry, with a WebGPU front end. "Signature-parametric" means
the triple `(p, q, r)` — number of positive, negative and degenerate
eigenvalues of the metric — is a value in the public API. The same kernel
computes Christoffel symbols, geodesics and curvature in:

- `(n, 0, 0)` — ordinary Euclidean / Riemannian geometry;
- `(n−1, 1, 0)` — Minkowski / Lorentzian geometry, the geometry of
  relativity;
- `(4, 0, 0)` — Greg Egan's *Orthogonal* trilogy, where light cones do
  not exist;
- `(2, 2, 0)` — Egan's *Dichronauts*, where there are two timelike
  directions;
- `(n, 0, 1)` — Galilean / Newton–Cartan geometry, with one degenerate
  direction.

## Why does this exist?

Three reasons.

1. **Pedagogy.** General relativity textbooks make `(3, 1, 0)` look like
   "the" metric signature. Reading Egan and then re-deriving general
   relativity in `(4, 0, 0)` quickly reveals which lemmas depend on the
   Lorentzian split and which are genuinely Riemannian. A library where
   the signature is *one parameter* makes that exercise tractable.
2. **Software ergonomics.** Existing Rust crates for differential geometry
   target a single signature and a single backend; existing Python and
   Julia libraries are dimension-erased but not signature-erased and ship
   no WebGPU path. We wanted the cross product to exist.
3. **Greg Egan deserves better software tooling.** The fiction is built
   on real, computable mathematics; until now those computations have not
   had a permissively-licensed open-source kernel that anyone can embed
   in a teaching applet.

## Status

This book documents v0.1.0. The library is small, well-tested, and slow:
the dimension-erased kernels in `signgeom-core` use plain
`Vec<Vec<f64>>` matrices, not SIMD or BLAS. Performance work is on the
roadmap for v0.2.

## A note on the maths

Every formula in this book and in the source has been re-derived from
public-domain mathematics. No code, applet source or asset from
[gregegan.net](https://www.gregegan.net) has been inspected or copied
during development. The thematic resonance is intentional; the
implementation is independent. See the [licence strategy](./license-strategy.md)
chapter for the details.
