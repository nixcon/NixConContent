#!/usr/bin/env bash

set -euo pipefail
shopt -s nullglob

nixpkgs=~/src/nixpkgs
# Only include directories with at least 30 subpaths
cutoff=30

nextIndex=1

doit() {
  pushd "$2" >/dev/null
  for f in *; do
    c=$(find "$f" | wc -l)
    if (( c >= cutoff )); then
      myIndex=$nextIndex
      (( nextIndex++ )) || true
      echo "{ \"name\": \"$f\", \"id\": $myIndex, \"parent\": $1 }"
      doit "$myIndex" "$f"
    fi
  done
  popd >/dev/null
}

{
  echo "{ \"name\": \"root\", \"id\": 0 }";
  doit 0 "$nixpkgs/pkgs";
} | tee /dev/stderr \
  | jq -s . > data.json
