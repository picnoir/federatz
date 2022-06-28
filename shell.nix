{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rust-analyzer
    pkgs.pkg-config
    pkgs.gtk4
  ];
}
