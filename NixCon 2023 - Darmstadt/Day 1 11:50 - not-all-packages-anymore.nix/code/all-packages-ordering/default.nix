let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/bfb7dfec93f3b5d7274db109f2990bc889861caf";
  pkgs = import nixpkgs {};
in
pkgs.rustPlatform.buildRustPackage {
  name = "nix-spp";
  src = pkgs.lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;
  passthru.pkgs = pkgs;
  passthru.shell = pkgs.mkShell {
    packages = [
      pkgs.cargo
      pkgs.rustfmt
      pkgs.rust-analyzer
      pkgs.rustc
    ];
  };
}
