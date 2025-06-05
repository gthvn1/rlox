{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz") {} }:

pkgs.mkShell {
  packages = with pkgs; [
    cargo
    clippy
    lld
    rust-analyzer
    rustc
    rustfmt
    wasm-pack
  ];
}

