{ pkgs, nixpkgs, system, makeRustPlatform, rust-overlay }:
let
  rustPkgs = import nixpkgs {
    inherit system;
    overlays = [ (import rust-overlay) ];
  };

  rustVersion = "1.93.0";
  wasmUnknownUknown = "wasm32-unknown-unknown";
  wasm32Wasi = "wasm32-wasip2";

  rustDefaultTarget = rustPkgs.rust-bin.stable.${rustVersion}.default;

  rustWithWasiTarget = rustPkgs.rust-bin.stable.${rustVersion}.default.override {
    targets = [ wasm32Wasi ];
  };

  rustWithWasmTarget = rustPkgs.rust-bin.stable.${rustVersion}.default.override {
    targets = [ wasmUnknownUknown ];
  };

  rustPlatform = makeRustPlatform {
    cargo = rustDefaultTarget;
    rustc = rustDefaultTarget;
  };

  rustPlatformWasm = makeRustPlatform {
    cargo = rustWithWasmTarget;
    rustc = rustWithWasmTarget;
  };

  common = {
    version = "0.2.5";
    src = self;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [ pkgs.pkg-config ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
in {
  workspace = pkgs.rustPlatform.buildRustPackage (common // {
    cargoBuildFlags = "--release --workspace --features full";
  });
}