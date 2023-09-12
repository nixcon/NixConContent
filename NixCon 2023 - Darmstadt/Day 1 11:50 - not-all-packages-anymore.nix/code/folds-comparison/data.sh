#!/usr/bin/env bash

set -euo pipefail

nixpkgs=~/src/nixpkgs

cargo build --release

logfile() {
  local origCommit=$1
  local origFile=$2
  local skipCount=10
  local i=0
  while mapfile -t -n 4 ary && (( ${#ary[@]} != 0 )); do
    if (( i % skipCount != 0 )); then
      (( i++ )) || true
      continue
    fi
    (( i++ )) || true
    rev=${ary[0]}
    date=${ary[1]}
    #empty=${ary[2]}
    file=${ary[3]}
    if ! result=$(git -C "$nixpkgs" show "$rev":"$file" | target/release/attr-order); then
      continue
    fi
    read -r linear tree <<< "$result"

    echo "{ \"rev\": \"$rev\", \"date\": \"$date\", \"linear\": $linear, \"tree\": $tree }"
  done < \
    <(git -C "$nixpkgs" log --first-parent --format='%H%n%cI' --name-only --follow "$origCommit" -- "$origFile")
}

{
  logfile 5fc95eef55ead8b721808f7f235c6fb032e68276 pkgs/top-level/all-packages.nix
  logfile 1894a8b86c4f5233f4f28af3b93156aaaee44af1~ pkgs-ng/system/all-packages.fix
} | tee /dev/stderr \
  | jq -s . > data.json
