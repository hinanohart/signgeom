# Contributing to signgeom

Thanks for your interest. This document describes how to set up a development
environment, what we look for in a PR, and the small set of unusual
conventions in this repository.

## Development environment

- **Rust:** pinned in `rust-toolchain.toml` (1.85, minimal profile + clippy
  + rustfmt). Installing `rustup` is enough; the toolchain will be fetched
  automatically.
- **Node:** ≥ 22, only required if you touch `web/`.
- **`gh` CLI:** only required if you want to interact with the GitHub
  release flow locally.

```bash
git clone https://github.com/hinanohart/signgeom
cd signgeom
cargo test       # core test suite
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## What we look for in a PR

1. **Tests.** Every public function has at least one unit test; every numerical
   primitive has at least one property test. Bug fixes ship with a regression
   test that fails before the fix and passes after.
2. **No silent precision changes.** If a tolerance changes in a numerical
   test, the PR description must say why and link to the relevant equations.
3. **Signature-aware by default.** New geometric operations should accept the
   signature as a type parameter rather than hardcoding Euclidean or
   Minkowski conventions.
4. **`clippy -D warnings`** must pass. If a lint is genuinely wrong here,
   prefer a narrow `#[allow]` with a comment over disabling globally.
5. **Greg Egan licence rule (important).** If your change touches anything in
   `plugins/egan-applets/`, you must *not* have looked at the source of any
   applet on `gregegan.net`. See `plugins/egan-applets/LICENSE-PENDING.md`.

## Failure museum

When a phase of work is genuinely stuck and we decide to abandon a branch,
the working tree is moved to `experiments/_wip/<topic>-<YYYYMMDD>/` rather
than deleted. This is intentional: failed attempts often turn out to contain
exactly the lemma we need three months later. Do not `rm -rf` this directory.

## Commit messages

We use [Conventional Commits](https://www.conventionalcommits.org/) loosely.
Types we actually use: `feat`, `fix`, `perf`, `refactor`, `test`, `docs`,
`chore`, `bench`. The body should explain the *why*; the diff already shows
the *what*.

## Releasing

The release process is documented in `docs/book/src/release-process.md`.
