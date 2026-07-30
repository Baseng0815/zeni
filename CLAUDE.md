# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Code guidelines

1. Prefer small, descriptive functions.
2. Add as few comments as possible. At most a single sentence per struct or function, and only if it is ABSOLUTELY necessary — do not add comments that are not extremely useful. This applies to doc comments too: the workspace sets `missing_docs = "warn"`, but do not add doc comments just to silence it.
3. Do not add any unit or integration tests unless specifically asked to.

## Commands

Development happens inside the Nix dev shell (`nix develop`, or `direnv allow` once), which provides the nightly toolchain with the `wasm32-unknown-unknown` target and `dx` (dioxus-cli).

- Run the app (builds both halves, hot-reloads): `cd web && dx serve`
- Type-check: `cargo check --workspace`
- Check the server half of `web`: `cargo check -p kane-web --no-default-features --features server`
- Lint: `cargo clippy --workspace --all-targets`
- Format: `cargo fmt --all` — must run on nightly (`rustfmt.toml` uses nightly-only options; stable silently skips them and reports spurious diffs). Under rustup: `cargo +nightly fmt --all`.
- Test one crate: `cargo test -p kane-finance`; a single test: `cargo test -p kane-finance <test_name>`

Package names carry a `kane-` prefix (`kane-finance`, `kane-inventory`, `kane-web`) so they never shadow a registry crate (`inventory` is a real crate pulled in transitively by dioxus). Dependents alias them back, so imports read `use finance::…`.

## Architecture

Three workspace crates. **Status: preliminary structure only** — no domain logic yet, no storage backend chosen.

- `finance/` — expense tracking and the double-entry ledger.
- `inventory/` — items, stock lots, movements, shopping lists.
- `web/` — dioxus-fullstack app: server and browser client in one crate.

`finance` and `inventory` are pure domain crates: no HTTP, no SQL, no async runtime, and no dependency on each other. Each defines persistence traits in its `store` module and leaves the implementation to the host. The one place the two domains meet is a purchase (a ledger transaction plus a stock movement), and `web` is what joins them.

`web` is compiled twice: to `wasm32-unknown-unknown` for the browser (`web` feature, default) and natively for the server (`server` feature). The domain crates are optional dependencies enabled only by the `server` feature, so they can never reach the wasm bundle — anything crossing the wire must use serde DTOs from `web/src/api/`, not domain types.

| `web/src` path | Compiled into       | Holds                                      |
| -------------- | ------------------- | ------------------------------------------ |
| `views/`       | both                | one module per route                       |
| `components/`  | both                | UI shared across views                     |
| `api/`         | both (split bodies) | server functions and their DTOs — the wire |
| `server/`      | server only         | app state, stores, sessions                |

Money and quantity arithmetic is the core of this workspace: the workspace lints warn on all numeric-cast precision-loss lints, `float_arithmetic`, and `unwrap_used`. `clippy.toml` forbids holding dioxus signal borrows (`GenerationalRef`, `WriteLock`) across await points — doing so deadlocks the signal.
