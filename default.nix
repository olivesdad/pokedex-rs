{
  pkgs ? import <nixpkgs> { },
}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "pokedex-rs";
  version = "0.1.0";
  cargoLock.lockFile = ./pokedex-rs/Cargo.lock;
  src = pkgs.lib.cleanSource ./pokedex-rs/.;
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
  buildInputs = with pkgs; [
    openssl
  ];
}
