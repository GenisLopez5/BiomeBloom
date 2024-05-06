{
  pkgs ? import <nixpkgs> { },
  lib,
}:
let
  packages = with pkgs; [
    cargo
    rustc
    rust-analyzer
    rustfmt
    clippy
    clang
    mold
    gnumake
    cmake
    sfml

  ];
in
pkgs.mkShell {
  nativeBuildInputs = packages;
  buildInputs = packages;
  env = {
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";
  };
}
