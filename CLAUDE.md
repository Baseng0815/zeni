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
- Check the server half of `web`: `cargo check -p zeni-web --no-default-features --features server`
- Check the browser half of `web`: `cargo check -p zeni-web --target wasm32-unknown-unknown` — the check that catches a non-wasm dependency leaking into the bundle; `cargo check --workspace` compiles the `web` feature natively and will not catch it
- Lint: `cargo clippy --workspace --all-targets`
- Format: `cargo fmt --all` — must run on nightly (`rustfmt.toml` uses nightly-only options; stable silently skips them and reports spurious diffs). Under rustup: `cargo +nightly fmt --all`.
- Test one crate: `cargo test -p zeni-finance`; a single test: `cargo test -p zeni-finance <test_name>`

Package names carry a `zeni-` prefix (`zeni-finance`, `zeni-inventory`, `zeni-web`) so they never shadow a registry crate (`inventory` is a real crate pulled in transitively by dioxus). Dependents alias them back, so imports read `use finance::…`.

## Architecture

Three workspace crates. **Status: preliminary structure only** — no domain logic yet, no storage backend chosen.

- `finance/` — expense tracking and the double-entry ledger.
- `inventory/` — items, stock lots, movements, shopping lists.
- `web/` — dioxus-fullstack app: server and browser client in one crate.

`finance` and `inventory` are domain crates: no SQL and no async runtime. Each defines persistence traits in its `store` module and leaves the implementation to the host. The one exception to "no HTTP" is `inventory`'s receipt extraction, which calls out to a vision model and is confined to the `extractors` feature. The one place the two domains meet is a purchase (a ledger transaction plus a stock movement), and `web` is what joins them.

`web` is compiled twice: to `wasm32-unknown-unknown` for the browser (`web` feature, default) and natively for the server (`server` feature). Both halves depend on the domain crates, so server functions in `web/src/api/` take and return domain types directly — there is no DTO layer. Every type that crosses the wire therefore needs `Serialize` and `Deserialize`.

What keeps the wasm bundle small is `inventory`'s `extractors` feature, not the crate boundary. It gates receipt extraction — the only code needing an HTTP client (`reqwest`) and image decoding (`image`, `schemars`) — and the workspace dependency leaves it off. `web`'s `server` feature turns it back on for the native half. Anything added to `inventory` that cannot compile to wasm belongs behind that feature.

Both domain crates carry target-gated `js` features for `uuid` and `jiff`: on `wasm32-unknown-unknown` the browser has to supply the randomness that uuid v7 needs and the clock that jiff needs. Without them `uuid` fails to build and `jiff` fails at runtime.

| `web/src` path | Compiled into       | Holds                                       |
| -------------- | ------------------- | ------------------------------------------- |
| `views/`       | both                | one module per route                        |
| `components/`  | both                | UI shared across views                      |
| `api/`         | both (split bodies) | server functions over domain types — the wire |
| `server/`      | server only         | app state, stores, sessions                 |

Server functions return `ServerFnResult<T>` from `dioxus::prelude` — there is no bare `Result<T>` alias. A function's signature is compiled into both halves; only its body is server-only, so server-only imports go inside the body.

Money and quantity arithmetic is the core of this workspace: the workspace lints warn on all numeric-cast precision-loss lints, `float_arithmetic`, and `unwrap_used`. `clippy.toml` forbids holding dioxus signal borrows (`GenerationalRef`, `WriteLock`) across await points — doing so deadlocks the signal.
