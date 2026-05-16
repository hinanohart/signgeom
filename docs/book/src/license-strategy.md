# Licence strategy

signgeom is permissively licensed under Apache-2.0. The strategy below
explains how an Egan-themed library avoids Egan's copyright.

## The library code

All code in `crates/` and `web/` is original. Every formula was
re-derived from one of:

- a standard textbook (Wald, *General Relativity*; do Carmo, *Riemannian
  Geometry*; Petersen, *Riemannian Geometry*);
- a peer-reviewed paper (Schwarzschild 1916, Penrose–Terrell 1959,
  Born 1909, Mermin 1990, Unruh 1976, Klein 1879);
- a preprint on arXiv (Flow-Lenia: 2212.07906; einstein-hat tile:
  2303.10798).

Public-domain mathematics, in other words. No Egan text was
transcribed or paraphrased. No source file from
[gregegan.net](https://www.gregegan.net) was inspected.

## Egan-themed names

signgeom uses the strings "Orthogonal" and "Dichronauts" in three places:

1. doc comments, as plain-English pointers to the relevant fiction;
2. the named constructors `Signature::orthogonal4` and
   `Signature::dichronauts4`;
3. the README and the present book.

We rely on the **idea / expression dichotomy** of copyright: the *idea*
of a 4-signature universe (or a 2 + 2 signature universe) is not
copyrightable. The names "Orthogonal" and "Dichronauts" are book titles
that have entered cultural usage; using them in technical references is
nominative use, not trademark infringement. There are no Egan-owned
trademarks for either word.

## The `plugins/egan-applets/` reservation

The directory `plugins/egan-applets/` is *reserved* for a future plugin
that will reimplement Egan's interactive physics applets using only
public-domain mathematics. The plugin is empty in v0.1.0. Its
`LICENSE-PENDING.md` enforces three rules on any future contributor:

1. Do not inspect `gregegan.net` applet source. Ever.
2. Re-derive every formula from a citable, non-Egan source.
3. Use only one-line "Inspired by …" attributions in comments; no Egan
   prose, no logos, no trademarks.

These rules are mechanically enforceable in CI by string-matching the
plugin against a small banned-token list.

## When Egan does notice

If Greg Egan stumbles across signgeom and is unhappy, the maintainers
will (in order):

1. Apologise.
2. Read whatever specific concern is raised.
3. If a name needs to change, change it. (Renaming `Signature::orthogonal4`
   to `Signature::pos4` is a one-PR change.)
4. If a chapter of this book references the fiction too closely, prune
   it.

We do not anticipate any of those steps being necessary: independent
re-derivation of public mathematics is exactly the kind of homage Egan
has welcomed in the past.
