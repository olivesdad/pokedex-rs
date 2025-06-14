{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
          (self: super: {
            rust-toolchain = self.rust-bin.fromRustupToolchainFile ./toolchain.toml;
          })
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

      in
      {
        devShells.default =
          with pkgs;
          mkShell rec {
            buildInputs = [
              rust-toolchain
              rust-bin.beta.latest.default
            ];
            packages = with pkgs; [
              openssl
              pkg-config
            ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            shellHook = ''
              	   export RUST_SRC_PATH="${rust-toolchain}/lib/rustlib/src/rust/library" 
            '';
          };
        packages = with pkgs; {
            pokedex-rs =  pkgs.callPackage ./default.nix {}; 
           };
        formatter = pkgs.nixfmt;
      }
    );
}
