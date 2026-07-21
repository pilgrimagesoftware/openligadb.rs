# Contributing

## Development setup

This is a Rust crate with no non-Rust dependencies. Clone the repo and build:

```bash
git clone https://github.com/pilgrimagesoftware/openligadb.rs.git
cd openligadb.rs
cargo build
```

The toolchain is pinned in `rust-toolchain.toml`; `rustup` picks it up automatically.

Before opening a PR, run what CI runs:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features --workspace
cargo test --no-default-features
cargo test --doc
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo check --no-default-features --target wasm32-wasip2
```

## Branching and PRs

This repo follows the standard `master`/`develop` git-flow:

- `master` reflects the latest released version; nothing is committed here directly.
- `develop` is the integration branch.
  Branch `feature/*` or `fix/*` off `develop`, and open your PR back into `develop`.
- `release/*` branches are cut automatically by the release workflow (see [RELEASING.md](RELEASING.md)).
  You shouldn't need to create one by hand.

CI (`ci.yaml`) runs `fmt`, `clippy`, `test` (with and without default features), `doc`, and a
`wasm32-wasip2` cross-compile check on every PR; all must pass before merging.

## Commit messages

Commits follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) (`feat:`, `fix:`, `chore:`, `docs:`, etc.).
This isn't enforced by CI today, but it's what `git-cliff` reads to generate the changelog and compute the next version on release.
A commit that doesn't follow the convention won't show up in the changelog and won't contribute to the version bump.

## The `http-client` feature

Every network method on a model (`League::list`, `Match::get`, etc.) must stay reachable with
default features and fail to compile with `--no-default-features` — see the "Feature gating"
section in [AGENTS.md](AGENTS.md) for the exact pattern to follow when adding a new one.
