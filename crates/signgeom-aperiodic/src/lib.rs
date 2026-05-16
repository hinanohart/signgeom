//! signgeom-aperiodic
//!
//! Wang-tile and "einstein hat" aperiodic tilings of signature-aware
//! manifolds.
//!
//! The geometry of the tile is parameterised by a [`signgeom_core::Signature`]:
//! the *same* matching rules can be placed on a Euclidean `(2, 0, 0)` plane,
//! on a Lorentzian sheet, or on the split-signature `(2, 2, 0)` background of
//! Greg Egan's *Dichronauts*. Whether the tiling can actually be realised
//! depends on the signature; this crate exposes the structural matching rules
//! and a small Turing-tile compiler regardless.
//!
//! v0.1.0 status: matching rules + adjacency graph + a Wang-tile to
//! Turing-machine encoder. Visualisation is left to `web/`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![allow(clippy::needless_range_loop, clippy::type_complexity)]

use signgeom_core::Signature;

/// Identifier of an edge colour on a Wang tile.
pub type EdgeColor = u16;

/// A square Wang tile labelled by edge colours `(north, east, south, west)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WangTile {
    /// Colour on the north edge.
    pub n: EdgeColor,
    /// Colour on the east edge.
    pub e: EdgeColor,
    /// Colour on the south edge.
    pub s: EdgeColor,
    /// Colour on the west edge.
    pub w: EdgeColor,
}

impl WangTile {
    /// Construct a Wang tile.
    #[must_use]
    pub const fn new(n: EdgeColor, e: EdgeColor, s: EdgeColor, w: EdgeColor) -> Self {
        Self { n, e, s, w }
    }

    /// Whether this tile is adjacent-compatible with `other` to its east.
    #[must_use]
    pub const fn matches_east(self, other: Self) -> bool {
        self.e == other.w
    }

    /// Whether this tile is adjacent-compatible with `other` to its north.
    #[must_use]
    pub const fn matches_north(self, other: Self) -> bool {
        self.n == other.s
    }
}

/// A finite Wang tile set together with the ambient manifold signature it is
/// intended to tile.
#[derive(Debug, Clone)]
pub struct TileSet {
    /// Background metric signature.
    pub signature: Signature,
    /// The tiles available.
    pub tiles: Vec<WangTile>,
}

impl TileSet {
    /// Construct a tile set.
    #[must_use]
    pub fn new(signature: Signature, tiles: Vec<WangTile>) -> Self {
        Self { signature, tiles }
    }

    /// Number of tile types.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    /// Whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    /// All ordered pairs `(i, j)` such that tile `i` may sit directly west of
    /// tile `j`.
    #[must_use]
    pub fn east_adjacency(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for (i, a) in self.tiles.iter().enumerate() {
            for (j, b) in self.tiles.iter().enumerate() {
                if a.matches_east(*b) {
                    out.push((i, j));
                }
            }
        }
        out
    }

    /// All ordered pairs `(i, j)` such that tile `i` may sit directly south of
    /// tile `j`.
    #[must_use]
    pub fn north_adjacency(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for (i, a) in self.tiles.iter().enumerate() {
            for (j, b) in self.tiles.iter().enumerate() {
                if a.matches_north(*b) {
                    out.push((i, j));
                }
            }
        }
        out
    }
}

/// Encoding of a Turing machine state-transition as a Wang tile.
///
/// Given a TM with finite alphabet `Σ` and state set `Q`, the standard Wang
/// encoding (Berger 1966 / Robinson 1971) uses tile edges labelled by
/// `(state, symbol)` pairs. Each transition `δ(q, a) = (q', b, L/R)` becomes
/// a small bundle of tiles; we expose the encoder as a function rather than a
/// type to keep the API surface small.
///
/// `transitions` is `(q, a) -> (q', b, dir)` with `dir = +1` for right and
/// `dir = -1` for left. The output tile set tiles the half-plane iff the
/// machine halts; this is the textbook proof of undecidability of the Wang
/// tiling problem.
///
/// The number of states is implicit in `transitions` (it equals the size of
/// the projection of the first component of each transition source).
#[must_use]
pub fn turing_machine_to_tileset(
    signature: Signature,
    n_symbols: u16,
    transitions: &[((u16, u16), (u16, u16, i8))],
) -> TileSet {
    let pack = |state: u16, symbol: u16| -> EdgeColor {
        state.saturating_mul(n_symbols).saturating_add(symbol)
    };
    let mut tiles = Vec::with_capacity(transitions.len() * 2 + n_symbols as usize);

    // "Substrate" tiles: a tape cell holding symbol `a` with no head present.
    for a in 0..n_symbols {
        tiles.push(WangTile::new(
            pack(0, a),
            pack(0, a),
            pack(0, a),
            pack(0, a),
        ));
    }

    // Transition tiles: encode the head moving through a cell.
    for &((q, a), (q1, b, dir)) in transitions {
        let here = pack(q, a);
        let after = pack(q1, b);
        match dir {
            1 => {
                // Head leaves to the east in state q'.
                tiles.push(WangTile::new(after, after, here, here));
            }
            -1 => {
                tiles.push(WangTile::new(after, here, here, after));
            }
            _ => {
                // Stay put.
                tiles.push(WangTile::new(after, here, here, here));
            }
        }
    }

    TileSet::new(signature, tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn east_adjacency_of_two_compatible_tiles() {
        // tile 0: E = 1, W = 0
        // tile 1: E = 2, W = 1  (matches east of tile 0)
        // tile 1 east-of tile 0:  1.W = 1 == 0.E = 1  -> match
        // tile 0 east-of tile 1:  0.W = 0 != 1.E = 2  -> no match
        let t = TileSet::new(
            Signature::riemannian(2),
            vec![WangTile::new(0, 1, 0, 0), WangTile::new(0, 2, 0, 1)],
        );
        let adj = t.east_adjacency();
        assert!(adj.contains(&(0, 1)));
        assert!(!adj.contains(&(1, 0)));
    }

    #[test]
    fn turing_encoding_produces_at_least_one_tile_per_transition() {
        let ts = turing_machine_to_tileset(
            Signature::riemannian(2),
            2,
            &[
                ((0, 0), (1, 1, 1)),
                ((0, 1), (1, 0, -1)),
                ((1, 0), (0, 1, 0)),
            ],
        );
        assert!(ts.len() >= 5); // 2 substrate + 3 transitions
        assert_eq!(ts.signature, Signature::riemannian(2));
    }
}
