# Egan's physics, briefly

This chapter is a thin pointer-page, not a tutorial. The mathematics that
underpins signgeom long predates Greg Egan; the fiction provides the
naming and the pedagogical motivation, not the formulas.

## "Orthogonal" `(4, 0, 0)`

In Egan's *Orthogonal* trilogy (*The Clockwork Rocket*, *The Eternal
Flame*, *The Arrows of Time*) the universe has signature `(4, 0, 0)`. Every
direction is geometrically interchangeable; the velocity-time-symmetry that
underwrites special relativity is replaced by a full four-rotation
symmetry. Light has a maximum frequency rather than a maximum speed, and
elementary particles have a *positive* rest energy density rather than a
negative one. signgeom does not implement any of those physical
predictions — it just gives you the geometry and lets you check the
kinematics.

## "Dichronauts" `(2, 2, 0)`

In *Dichronauts* there are two spacelike directions and two timelike. The
inhabitants navigate by "looking sideways" (using the second time
direction) and have a permanently constrained physiology. The signgeom
example `dichronauts_geodesic` shows the three causal classes — spacelike,
null, timelike — that the same kinematics produces in `(2, 2, 0)`.

## *Schild's Ladder*, *Permutation City* and friends

Other Egan books touch on metric signature less directly: *Schild's
Ladder* features parallel transport along a connection (the namesake
construction!), *Permutation City* leans on Dust Theory and computational
geometry, *Wang's Carpets* and *Diaspora* on Wang-tile cellular automata.
signgeom's `signgeom-aperiodic` and `signgeom-lenia` crates are thin
homages to the cellular-automaton side of that work.

## Why the library is not called `eganlib`

Because the mathematics is not Egan's — it is Sylvester's, Riemann's,
Levi-Civita's, Schwarzschild's. The library aims to be useful for
relativity, machine learning on manifolds, and Galilean geometry just as
much as for reading Egan. The branding chooses the *math* over the
*fiction*.
