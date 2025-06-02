{
  pkgs ? import <nixpkgs> { },
}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "pokedex-rs";
  version = "0.1.0";
  cargoLock.lockFile = ./pokedex-rs/Cargo.lock;
  src = pkgs.lib.cleanSource ./pokedex-rs/.;
  nativeBuildInputs = with pkgs; [
    openssl
    pkg-config
    
  ];
  env =  {
     PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig";
  };
 
}
