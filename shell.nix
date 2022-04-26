{ pkgs ? import <nixpkgs> {} }:
with pkgs;
mkShell {
  buildInputs = [
    openssl
    pkgconfig
    protobuf3_9
  ];
  shellHook = ''
  '';
}
