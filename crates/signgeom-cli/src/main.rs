//! Command-line front-end for the signgeom library.

#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use signgeom_aperiodic::{turing_machine_to_tileset, TileSet, WangTile};
use signgeom_core::{
    integrate_geodesic, scalar_curvature, GeodesicConfig, MinkowskiFlat, Schwarzschild, Signature,
};
use signgeom_lenia::{GrowthKernel, LeniaWorld};

#[derive(Debug, Parser)]
#[command(name = "signgeom", version, about = "Signature-parametric geometry CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Print a manifold's signature and metric at a point.
    Signature(SignatureArgs),
    /// Integrate a geodesic in a flat or Schwarzschild manifold.
    Geodesic(GeodesicArgs),
    /// Print the Ricci scalar at a Schwarzschild point.
    Ricci(RicciArgs),
    /// Encode a small Turing machine as a Wang tile set.
    Tiling(TilingArgs),
    /// Run a few steps of a Flow-Lenia automaton.
    Lenia(LeniaArgs),
}

#[derive(Debug, clap::Args)]
struct SignatureArgs {
    /// One of `riemannian-N`, `minkowski-N`, `orthogonal4`, `dichronauts4`, `galilean-N`.
    #[arg(long, default_value = "minkowski-4")]
    kind: String,
}

#[derive(Debug, clap::Args)]
struct GeodesicArgs {
    /// `flat` or `schwarzschild`.
    #[arg(long, default_value = "flat")]
    manifold: String,
    /// Initial position, comma-separated (`x0,x1,x2,x3`).
    #[arg(long, default_value = "10.0,1.5707963267948966,0.0,0.0")]
    x0: String,
    /// Initial velocity, comma-separated.
    #[arg(long, default_value = "0.0,0.0,0.1,1.0")]
    v0: String,
    /// Integration steps.
    #[arg(long, default_value_t = 200)]
    steps: usize,
    /// Proper time / affine parameter range.
    #[arg(long, default_value_t = 10.0)]
    proper_time: f64,
    /// Schwarzschild mass (only when `--manifold schwarzschild`).
    #[arg(long, default_value_t = 1.0)]
    mass: f64,
    /// Signature when manifold = `flat`. One of `minkowski-4`, `orthogonal4`, `dichronauts4`, `riemannian-N`.
    #[arg(long, default_value = "minkowski-4")]
    flat_signature: String,
}

#[derive(Debug, clap::Args)]
struct RicciArgs {
    /// Schwarzschild mass.
    #[arg(long, default_value_t = 1.0)]
    mass: f64,
    /// Radius (must be > 2M).
    #[arg(long, default_value_t = 10.0)]
    r: f64,
}

#[derive(Debug, clap::Args)]
struct TilingArgs {
    /// Number of TM states.
    #[arg(long, default_value_t = 2)]
    states: u16,
    /// Number of TM symbols.
    #[arg(long, default_value_t = 2)]
    symbols: u16,
}

#[derive(Debug, clap::Args)]
struct LeniaArgs {
    /// World size (square).
    #[arg(long, default_value_t = 32)]
    size: usize,
    /// Number of steps to advance.
    #[arg(long, default_value_t = 16)]
    steps: usize,
    /// Time step.
    #[arg(long, default_value_t = 0.1)]
    dt: f64,
}

fn parse_signature(s: &str) -> Result<Signature, String> {
    let s = s.trim().to_lowercase();
    if let Some(rest) = s.strip_prefix("riemannian-") {
        let n: usize = rest.parse().map_err(|e| format!("{e}"))?;
        return Ok(Signature::riemannian(n));
    }
    if let Some(rest) = s.strip_prefix("minkowski-") {
        let n: usize = rest.parse().map_err(|e| format!("{e}"))?;
        return Ok(Signature::minkowski(n));
    }
    if let Some(rest) = s.strip_prefix("galilean-") {
        let n: usize = rest.parse().map_err(|e| format!("{e}"))?;
        return Ok(Signature::galilean(n));
    }
    match s.as_str() {
        "orthogonal4" | "orthogonal-4" => Ok(Signature::orthogonal4()),
        "dichronauts4" | "dichronauts-4" => Ok(Signature::dichronauts4()),
        other => Err(format!("unknown signature `{other}`")),
    }
}

fn parse_vec(s: &str) -> Result<Vec<f64>, String> {
    s.split(',')
        .map(|t| t.trim().parse::<f64>().map_err(|e| format!("{e}")))
        .collect()
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Signature(args) => {
            let s = parse_signature(&args.kind)?;
            println!(
                "signature = (p={}, q={}, r={})  dim={}",
                s.p,
                s.q,
                s.r,
                s.dim()
            );
            println!("canonical diagonal = {:?}", s.canonical_diagonal());
        }
        Cmd::Geodesic(args) => {
            let cfg = GeodesicConfig {
                steps: args.steps,
                proper_time: args.proper_time,
                ..GeodesicConfig::default()
            };
            let x0 = parse_vec(&args.x0)?;
            let v0 = parse_vec(&args.v0)?;
            let path = match args.manifold.as_str() {
                "flat" => {
                    let sig = parse_signature(&args.flat_signature)?;
                    let m = MinkowskiFlat::new(sig);
                    integrate_geodesic(&m, &x0, &v0, cfg)?
                }
                "schwarzschild" => {
                    let m = Schwarzschild { mass: args.mass };
                    integrate_geodesic(&m, &x0, &v0, cfg)?
                }
                other => return Err(format!("unknown manifold `{other}`").into()),
            };
            println!("steps = {}", path.len() - 1);
            let last = path.last().unwrap();
            println!(
                "final tau = {:.6}\nfinal position = {:?}\nfinal velocity = {:?}",
                last.tau, last.position, last.velocity
            );
        }
        Cmd::Ricci(args) => {
            if args.r <= 2.0 * args.mass {
                return Err("r must be greater than the Schwarzschild radius 2M".into());
            }
            let m = Schwarzschild { mass: args.mass };
            let theta = std::f64::consts::FRAC_PI_2;
            let s = scalar_curvature(&m, &[args.r, theta, 0.0, 0.0])?;
            println!("R (scalar curvature) = {s:.3e}  (vacuum prediction: 0)");
        }
        Cmd::Tiling(args) => {
            let ts: TileSet = turing_machine_to_tileset(
                Signature::riemannian(2),
                args.states,
                args.symbols,
                &[
                    ((0, 0), (1, 1, 1)),
                    ((0, 1), (1, 0, -1)),
                    ((1, 0), (0, 1, 0)),
                ],
            );
            println!("encoded {} tiles", ts.len());
            for (i, t) in ts.tiles.iter().enumerate() {
                let WangTile { n, e, s, w } = *t;
                println!("  #{i}: N={n} E={e} S={s} W={w}");
            }
        }
        Cmd::Lenia(args) => {
            let mut world = LeniaWorld::zeros(args.size, Signature::riemannian(2));
            world.set_block(args.size / 2 - 2, args.size / 2 - 2, 4, 4, 0.6);
            let kernel = GrowthKernel {
                mu: 0.5,
                sigma: 0.15,
                radius: 4,
            };
            for step in 0..args.steps {
                world.step(&kernel, args.dt, 0.3, 0.05);
                println!("step {step:3}: mass = {:.4}", world.mass());
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
