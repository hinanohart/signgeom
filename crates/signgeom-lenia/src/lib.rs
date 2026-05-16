//! signgeom-lenia
//!
//! *Flow-Lenia*-style continuous cellular automata on arbitrary-signature
//! manifolds. v0.1.0 ships a small, well-tested CPU implementation that runs
//! on a flat background; signature-aware kernels are exposed as a trait so
//! that future versions can substitute curved-background kernels without
//! breaking call-sites.
//!
//! The reference for the original Lenia (2019) and Flow-Lenia (arXiv
//! 2212.07906) is acknowledged in the docs/book. The implementation here is
//! independent.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![allow(clippy::needless_range_loop)]

use signgeom_core::Signature;

/// A radial growth kernel `K: (0, 1] -> [0, 1]` together with the radius at
/// which it is truncated.
#[derive(Debug, Clone, Copy)]
pub struct GrowthKernel {
    /// Centre of the Gaussian bump (in units of the truncation radius).
    pub mu: f64,
    /// Standard deviation of the Gaussian bump.
    pub sigma: f64,
    /// Truncation radius in lattice cells.
    pub radius: usize,
}

impl GrowthKernel {
    /// Evaluate the kernel at fractional radius `r ∈ [0, 1]`.
    #[must_use]
    pub fn value(&self, r: f64) -> f64 {
        let d = (r - self.mu) / self.sigma;
        (-0.5 * d * d).exp()
    }
}

/// A square Lenia world stored as a flat row-major `Vec<f64>`.
#[derive(Debug, Clone)]
pub struct LeniaWorld {
    /// Linear edge length.
    pub size: usize,
    /// Background metric signature (ignored on flat worlds, used by curved
    /// extensions).
    pub signature: Signature,
    /// Cell occupancies, length = `size * size`.
    pub cells: Vec<f64>,
}

impl LeniaWorld {
    /// Construct a world of given size, filled with zero.
    #[must_use]
    pub fn zeros(size: usize, signature: Signature) -> Self {
        Self {
            size,
            signature,
            cells: vec![0.0; size * size],
        }
    }

    /// Set a square block of cells to `value`.
    pub fn set_block(&mut self, x0: usize, y0: usize, w: usize, h: usize, value: f64) {
        for y in y0..y0.saturating_add(h).min(self.size) {
            for x in x0..x0.saturating_add(w).min(self.size) {
                self.cells[y * self.size + x] = value;
            }
        }
    }

    /// Sum of all cell values, useful as a conservation check on Flow-Lenia.
    #[must_use]
    pub fn mass(&self) -> f64 {
        self.cells.iter().sum()
    }

    /// One Lenia step using a separable Gaussian growth function.
    ///
    /// This is the simplest "single-channel" Lenia: convolve with a radial
    /// kernel, push the resulting potential through `growth`, then clip into
    /// `[0, 1]`. Signature-aware kernels override this method.
    pub fn step(&mut self, kernel: &GrowthKernel, dt: f64, growth_centre: f64, growth_width: f64) {
        let n = self.size;
        let r = kernel.radius as isize;
        let mut next = vec![0.0; self.cells.len()];
        for y in 0..n {
            for x in 0..n {
                let mut sum = 0.0;
                let mut wsum = 0.0;
                for dy in -r..=r {
                    for dx in -r..=r {
                        let dist = ((dx * dx + dy * dy) as f64).sqrt() / kernel.radius as f64;
                        if dist > 1.0 || dist <= 0.0 {
                            continue;
                        }
                        let w = kernel.value(dist);
                        let xx = (x as isize + dx).rem_euclid(n as isize) as usize;
                        let yy = (y as isize + dy).rem_euclid(n as isize) as usize;
                        sum += w * self.cells[yy * n + xx];
                        wsum += w;
                    }
                }
                let avg = if wsum > 0.0 { sum / wsum } else { 0.0 };
                let g = ((-0.5 * ((avg - growth_centre) / growth_width).powi(2)).exp() * 2.0) - 1.0;
                let v = self.cells[y * n + x] + dt * g;
                next[y * n + x] = v.clamp(0.0, 1.0);
            }
        }
        self.cells = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_world_has_zero_mass() {
        let w = LeniaWorld::zeros(8, Signature::riemannian(2));
        assert_eq!(w.mass(), 0.0);
    }

    #[test]
    fn set_block_increases_mass() {
        let mut w = LeniaWorld::zeros(16, Signature::riemannian(2));
        w.set_block(2, 2, 3, 3, 0.5);
        // 3 * 3 = 9 cells at 0.5 each = 4.5
        assert!((w.mass() - 4.5).abs() < 1e-12);
    }

    #[test]
    fn step_runs_and_clips_to_unit_interval() {
        let mut w = LeniaWorld::zeros(8, Signature::riemannian(2));
        w.set_block(3, 3, 2, 2, 0.6);
        let k = GrowthKernel {
            mu: 0.5,
            sigma: 0.15,
            radius: 3,
        };
        w.step(&k, 0.1, 0.3, 0.05);
        for v in &w.cells {
            assert!((0.0..=1.0).contains(v));
        }
    }
}
