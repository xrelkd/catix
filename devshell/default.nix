{ rustToolchain
, cargoArgs
, unitTestArgs
, pkgs
, Security
, SystemConfiguration
, ...
}:

let
  cargo-ext = pkgs.callPackage ./cargo-ext.nix { inherit cargoArgs unitTestArgs; };
in
pkgs.mkShell {
  name = "dev-shell";

  buildInputs =
    with pkgs; [ ] ++
      lib.optionals stdenv.isDarwin [
        Security
        SystemConfiguration
      ];

  nativeBuildInputs = with pkgs; [
    cargo-ext.cargo-build-all
    cargo-ext.cargo-clippy-all
    cargo-ext.cargo-doc-all
    cargo-ext.cargo-nextest-all
    cargo-ext.cargo-test-all
    cargo-nextest
    rustToolchain

    tokei

    jq

    hclfmt
    nixpkgs-fmt
    nodePackages.prettier
    shfmt
    taplo
    treefmt

    shellcheck

    libgit2
    pkg-config
  ];

  shellHook = ''
    export NIX_PATH="nixpkgs=${pkgs.path}"
  '';
}
