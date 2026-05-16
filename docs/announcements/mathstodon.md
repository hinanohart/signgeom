# mathstodon.xyz post — draft

**Visibility:** public.
**Content warning:** none.
**Language:** English.
**Length:** ≤ 500 chars (Mastodon limit).

**Body candidate 1 (concise, ~490 chars):**

```text
just released signgeom 0.1 — a tiny Rust library where the *signature*
(p, q, r) of a (pseudo-)Riemannian metric is a value in the public API.
the same kernel does Riemannian, Minkowski, Egan's "Orthogonal"
(4,0,0) and "Dichronauts" (2,2,0). Christoffel + Ricci + RK4 geodesics,
plus a small WebGPU demo. Apache-2.0; mathematics independently
re-derived from textbooks, no code from gregegan.net was looked at.

https://github.com/hinanohart/signgeom
#rustlang #geometry #relativity
```

**Body candidate 2 (more technical, ~500 chars):**

```text
signgeom 0.1 is out. a Rust workspace that treats Sylvester's
(p, q, r) signature as a value rather than a compile-time choice.
the same Christoffel/Ricci/RK4-geodesic kernel runs in Euclidean,
Lorentzian, Egan's "Orthogonal" (4,0,0) and split (2,2,0)
"Dichronauts". WGSL compute path (4D flat) cross-checked against the
CPU integrator within single-precision tolerance.
Apache-2.0, math independently re-derived.

https://github.com/hinanohart/signgeom
#math #rustlang #differentialgeometry
```

## Known limitations (link in profile or first reply)

If the post lands well, post a follow-up reply with:

```text
v0.1.x scope honesty:
- WGSL kernel covers 4D flat metrics only; higher dim & curved on CPU.
- CPU/GPU agree to single-precision tolerance (≤ 1e-5 rel), not bitwise.
- Egan-Lenia ships with a Euclidean kernel; signature-aware kernel = v0.2.
- Wang tiles included; einstein hat monotile = v0.1.x roadmap.
- Curvature primitives validated on Schwarzschild, 2-sphere, 2-hyperbolic
  and flat Euclidean / Orthogonal / Dichronauts cases.
```

**Hashtag strategy:**

- `#rustlang` — broad Rust audience
- `#geometry` or `#differentialgeometry` — mathstodon's strongest crowd
- `#relativity` — pulls physicists who follow @gregeganSF without
  pinging him directly
- avoid `#egan` and `@gregeganSF` (the licence-decision says we do not
  initiate contact)

**Pre-post checklist:**

- [ ] Pin to profile for 48 h
- [ ] Boost any thoughtful reply
- [ ] DO NOT @ Greg Egan; per `architecture/license-decision.md` mode is
      independent-reimpl-only and we do not initiate contact
- [ ] If Egan boosts or replies organically, thank him in a follow-up
      reply only — do not start a thread
