# https://discourse.nixos.org/t/any-ideas-to-marry-libxml2-pip-and-nix-shell/277
# https://nixos.wiki/wiki/Python#Emulating_virtualenv_with_nix-shell
let
  pkgs = import <nixpkgs> {};
  # nanomsg-py = ...build expression for this python library...;
in pkgs.mkShell {
  nativeBuildInputs = [
    # pkgs.python38Packages.pip
    # pkgs.python38Packages.setuptools
  ];
  buildInputs = [
    pkgs.python38Packages.pip
    pkgs.icu.dev
    # pkgs.libxml2
    # pkgs.libxslt   
    # pkgs.ncurses
    # nanomsg-py
  ];
  shellHook = ''
    # Tells pip to put packages into $PIP_PREFIX instead of the usual locations.
    # See https://pip.pypa.io/en/stable/user_guide/#environment-variables.
    python -m venv .venv
    . .venv/bin/activate
    # export PIP_PREFIX=$(pwd)/.pip/pip_packages
    # export PYTHONPATH="$PIP_PREFIX/${pkgs.python3.sitePackages}:$PYTHONPATH"
    # export PATH="$PIP_PREFIX/bin:$PATH"
    unset SOURCE_DATE_EPOCH
  '';
}
