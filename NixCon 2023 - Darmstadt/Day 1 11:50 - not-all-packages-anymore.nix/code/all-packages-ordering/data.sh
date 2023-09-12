#!/usr/bin/env nix-shell
#!nix-shell -i bash

set -euo pipefail

nixpkgs=~/src/nixpkgs

cargo run -- "$nixpkgs"/pkgs/top-level/all-packages.nix \
  | jq -s . > data.json
