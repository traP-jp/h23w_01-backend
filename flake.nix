{
  description = "23冬ハッカソンチーム01 バックエンド";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustPlatform = pkgs.makeRustPlatform {
          rustc = toolchain;
          cargo = toolchain;
        };
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = with pkgs; [ openssl libiconv ] ++ lib.optionals stdenvNoCC.isDarwin [ darwin.Security ];
        defaultBuildArgs = {
          pname = "h23w_01-backend";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          inherit nativeBuildInputs buildInputs;
          doCheck = false;
          buildType = "debug";
        };
        buildRustPackage = attrs: rustPlatform.buildRustPackage (defaultBuildArgs // attrs);
      in
      {
        devShells.default = pkgs.stdenv.mkDerivation {
          name = "h23w_01-backend";
          nativeBuildInputs = nativeBuildInputs ++ [ toolchain ];
          inherit buildInputs;
        };
        packages.default = buildRustPackage { };
      }
    );
}
