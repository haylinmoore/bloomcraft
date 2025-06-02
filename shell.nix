{ pkgs ? import <nixpkgs> {}}:
let
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
  ];

  RUST_BACKTRACE = 1;
}
