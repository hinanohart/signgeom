# Release process

signgeom uses [semantic versioning](https://semver.org). Until v1.0 we
treat every `0.x` bump as potentially breaking. The release process is
deliberately small.

## Pre-release checklist

1. `cargo test --workspace` is green.
2. `cargo clippy --workspace --all-targets -- -D warnings` is green.
3. `cargo fmt --all -- --check` is green.
4. `cargo audit` and `cargo deny check` are green.
5. `gitleaks detect` reports no leaks.
6. `web/`: `pnpm typecheck && pnpm build` succeed.
7. `mdbook build` succeeds in `docs/book/`.
8. `CHANGELOG.md` has a new section for the version being released.

## Tagging

```bash
git tag -a v0.1.0 -m "v0.1.0"
git push origin v0.1.0
```

The `release.yml` GitHub workflow takes over from there: it builds the CLI
for three platforms, packages them, and creates a GitHub Release with the
artefacts attached and `CHANGELOG.md` as the body.

## Publishing to crates.io

Each member crate is published in dependency order. `cargo publish` is
not run automatically — that is a deliberate choice: an irreversible
publish should be an opt-in human action.

```bash
(cd crates/signgeom-core && cargo publish --dry-run)
(cd crates/signgeom-aperiodic && cargo publish --dry-run)
(cd crates/signgeom-lenia && cargo publish --dry-run)
(cd crates/signgeom-cli && cargo publish --dry-run)
# review output, then drop --dry-run
```

## After release

Update `CHANGELOG.md` to start a new `[Unreleased]` section. Open a
follow-up issue for any limitation listed under "Known limitations" so
that it gets attention before v0.2.
