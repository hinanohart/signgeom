//! Geodesic-integration benchmark.
//!
//! Three signatures are exercised:
//!   - Minkowski (3, 1, 0) — the relativistic baseline
//!   - Orthogonal (4, 0, 0) — Egan's signature
//!   - Schwarzschild non-trivial curvature
//!
//! The point of the comparison is not raw speed (the kernel is `Vec<Vec<f64>>`
//! and intentionally simple) but to make the cost of *signature changes* and
//! the cost of *curvature* legible. With criterion's HTML output you can see
//! that Schwarzschild is roughly an order of magnitude slower than flat —
//! the bottleneck is the central-difference call inside `christoffel`.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use signgeom_core::{integrate_geodesic, GeodesicConfig, MinkowskiFlat, Schwarzschild, Signature};

fn bench_flat(c: &mut Criterion) {
    let mut group = c.benchmark_group("geodesic/flat");
    let cfg = GeodesicConfig {
        steps: 200,
        proper_time: 5.0,
        ..GeodesicConfig::default()
    };
    for (name, sig) in [
        ("minkowski-4", Signature::minkowski(4)),
        ("orthogonal-4", Signature::orthogonal4()),
        ("dichronauts-4", Signature::dichronauts4()),
    ] {
        let m = MinkowskiFlat::new(sig);
        group.bench_with_input(BenchmarkId::from_parameter(name), &sig, |b, _| {
            b.iter(|| {
                integrate_geodesic(
                    &m,
                    black_box(&[0.0_f64; 4]),
                    black_box(&[0.5, 0.0, 0.0, 1.0]),
                    cfg,
                )
                .unwrap()
            });
        });
    }
    group.finish();
}

fn bench_schwarzschild(c: &mut Criterion) {
    let mut group = c.benchmark_group("geodesic/schwarzschild");
    let m = Schwarzschild { mass: 1.0 };
    for steps in [50, 200] {
        let cfg = GeodesicConfig {
            steps,
            proper_time: 4.0,
            ..GeodesicConfig::default()
        };
        group.bench_with_input(BenchmarkId::from_parameter(steps), &cfg, |b, &cfg| {
            b.iter(|| {
                integrate_geodesic(
                    &m,
                    black_box(&[10.0_f64, std::f64::consts::FRAC_PI_2, 0.0, 0.0]),
                    black_box(&[0.0, 0.0, 0.08, 1.0]),
                    cfg,
                )
                .unwrap()
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_flat, bench_schwarzschild);
criterion_main!(benches);
