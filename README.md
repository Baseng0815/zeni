# kane

Expense management with integrated inventory management.

Kane tracks two things that are usually kept in separate apps but are really one
problem: what you spent, and what you got for it. Buying groceries is both a
ledger entry and a change in what is in the pantry.

**Status: preliminary structure only.** The crates, module layout, and build
environment are in place. No domain logic is implemented yet, and no storage
backend has been chosen — module docs state what each module is for.

## Layout

```
kane/
├── flake.nix        nix dev shell: nightly rust + wasm32 target, dioxus-cli
├── Cargo.toml       workspace root, shared deps and lints
├── finance/         expense tracking and the double-entry ledger
├── inventory/       items, stock lots, movements, shopping lists
└── web/             dioxus-fullstack app: server + browser client
```

`finance` and `inventory` are pure domain crates — no HTTP, no SQL, no async
runtime. Each defines persistence *traits* in its `store` module and leaves the
implementation to the host. Neither depends on the other; the one place they meet
is a purchase, and `web` is what joins them.

`web` is a single dioxus-fullstack crate compiled twice — to
`wasm32-unknown-unknown` for the browser and natively for the server:

| Path              | Compiled into        | Holds                                        |
| ----------------- | -------------------- | -------------------------------------------- |
| `src/views/`      | both                 | one module per route                         |
| `src/components/` | both                 | UI shared across views                       |
| `src/api/`        | both (split bodies)  | server functions and their DTOs — the wire   |
| `src/server/`     | server only          | app state, stores, sessions                  |

`finance` and `inventory` are **optional** dependencies enabled by the `server`
feature, so they cannot end up in the wasm bundle. That is what forces API
signatures to use DTOs from `src/api/` rather than domain types.

### Package naming

Directories and code-level names are `finance`, `inventory`, `web`; the Cargo
package names are `kane-finance`, `kane-inventory`, `kane-web`. The prefix avoids
shadowing a registry crate — `inventory` is a real crate on crates.io, pulled in
transitively by dioxus for server-function registration. Dependents alias the
packages back, so code still reads `use finance::…`.

## Development

The dev shell provides the nightly toolchain (with the `wasm32-unknown-unknown`
target), `dx` (dioxus-cli 0.7.9), and the LLVM tools:

```sh
nix develop          # or: direnv allow, once
```

Run the app — `dx` builds both halves and hot-reloads:

```sh
cd web && dx serve
```

Checks:

```sh
cargo check --workspace
cargo check -p kane-web --no-default-features --features server   # server half
cargo clippy --workspace --all-targets
cargo fmt --all                                                   # nightly only
```

`rustfmt.toml` uses nightly-only options, so format with the nightly toolchain —
the dev shell's default already is. Under rustup, prefix `+nightly`.

## Next steps

Deliberately open, in rough dependency order:

1. Pick a storage backend and implement the `store` traits (SQLite via `sqlx` is
   the obvious default for a self-hosted app; nothing above depends on the
   choice).
2. Fill in `finance::money` and `inventory::unit` first — every other type is
   denominated in them.
3. Accounts and sessions in `web/src/server/`, since every store query is scoped
   by user.
4. The purchase flow that writes a ledger transaction and a stock movement
   together.
