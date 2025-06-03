# Pokedex-rs
Doesnt really do much just practice with rust + Nix packaging.

## Installation
### nix flake

```nix
# flake.nix
  inputs = {
    # NixOS official package source, using the nixos-23.11 branch here
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    # pokedex-rs
    pokedex.url = "github:olivesdad/pokedex-rs";
  };


  outputs =
    { self, pokedex, nixpkgs, nixpkgs-unstable, ... }@inputs:
    let
    pokedex-rs = pokedex.packages."x86_64-linux".pokedex-rs;

    in {
    nixsConfigurations.<nixos> = nixpkgs.lib.nixosSystem{
      specialArgs = { pokedex-rs };
    ...
    ... 
```
Then add to packages!

## nix build
`nix build github:olivesdad/pokedex-rs#pokedex-rs`

## nix run
`nix run github:olivesdad/pokedex-rs#pokedex-rs -- -p pikachu`
