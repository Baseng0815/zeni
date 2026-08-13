{
  description = "zeni — expense and inventory management";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      # Keep a single nixpkgs in the lock; rust-overlay only uses its own for checks.
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # flake-utils has no nixpkgs input to follow — it takes `systems` instead.
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [ (import rust-overlay) ];
        };

        # wasm32 is not optional here: `dx serve` builds the client half of the
        # fullstack app for wasm32-unknown-unknown and the server half natively.
        rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [ "wasm32-unknown-unknown" ];
        });

        # Single knob for the shell's LLVM/clang version (stdenv, tools).
        llvmPackages = pkgs.llvmPackages;

        # nixpkgs defaults `cudaSupport` to false, which yields a CPU-only
        # llama-server that silently ignores `-ngl` — ~7 t/s instead of ~92.
        # Overriding per-package rather than via `config.cudaSupport` keeps the
        # CUDA rebuild off every other package in the shell.
        llamaCpp = pkgs.llama-cpp.override { cudaSupport = true; };
      in {
        devShells.default = (pkgs.mkShell.override { stdenv = llvmPackages.stdenv; }) {
          nativeBuildInputs = with pkgs; [
            rustToolchain
            dioxus-cli
            pkg-config
            cmake
            # `dx` shells out to wasm-opt for release web builds.
            binaryen
            # `dot`, for rendering the graphs written by `Ledger::save_as_graph`.
            graphviz
            jetbrains.rust-rover
            # CUDA-only, so it drops out on darwin — `eachDefaultSystem` covers
            # systems that cannot build it.
          ] ++ lib.optional stdenv.isLinux llamaCpp
            ++ (with llvmPackages; [
            clang-tools
            llvm
            lld
            lldb
          ]);

          buildInputs = with pkgs; [
            openssl
          ];

          shellHook = ''
            mkdir -p ~/.rust-rover/toolchain

            ln -sfn ${rustToolchain}/lib ~/.rust-rover/toolchain
            ln -sfn ${rustToolchain}/bin ~/.rust-rover/toolchain

            export RUST_SRC_PATH="$HOME/.rust-rover/toolchain/lib/rustlib/src/rust/library"
            # Drop into zsh only for interactive `nix develop`. direnv runs this
            # hook in a non-interactive bash while evaluating the environment;
            # spawning a shell there re-triggers the direnv hook and recurses.
            case $- in *i*) zsh ;; esac
          '';
        };
      }
    );
}
