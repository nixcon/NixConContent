#!/usr/bin/env nix-shell
#!nix-shell -i bash -p nodePackages.vega-lite

set -euo pipefail

nixpkgs=~/src/nixpkgs
lastCommit=5fc95eef55ead8b721808f7f235c6fb032e68276
file=pkgs/top-level/all-packages.nix

cargo build --release

attr-order() {
  ../target/release/attr-order "$@"
}

endEpoch=$(git -C ~/src/nixpkgs show "$lastCommit" --format=%ct)
# With 60 FPS, that's one year per second
secondsPerFrame=$(( 365 * 24 * 60 ))
frameCount=0

rm -rf frames
mkdir -p frames

while mapfile -t -n 4 ary && (( ${#ary[@]} != 0 )); do
  rev=${ary[0]}
  date=${ary[1]}
  #empty=${ary[2]}
  file=${ary[3]}

  age=$(( endEpoch - date ))

  targetFrameCount=$(( age / secondsPerFrame ))

  # This is a bit weird, but the idea was to make sure that each frame corresponds to the same amount of time
  # so if there's a bunch of commits that don't change all-packages.nix, we need to generate multiple frames for that
  framesToEmit=$(( targetFrameCount - frameCount ))


  if (( framesToEmit == 0 )); then
    continue
  fi

  if true; then
    echo "Emitting $framesToEmit for commit $rev"
    git -C "$nixpkgs" show "$rev":"$file" > contents.nix

    if ! attr-order contents.nix | jq -s . > data.json; then
      echo "Failed to run the tool on commit $rev"
      exit 1
    fi


    for frameIndex in $(seq "$frameCount" "$(( targetFrameCount - 1 ))"); do
      targetDate=$(( endEpoch - frameIndex * secondsPerFrame ))
      day=$(date -d@"$targetDate" -I)
      sed "s/REPLACEME/$day/" plot.json > plot-replaced.json
      vl2png -s 4 plot-replaced.json > plot.png
      cp plot.png frames/plot-"$(printf %06d "$frameIndex")".png
    done
  else
    for frameIndex in $(seq "$frameCount" "$(( targetFrameCount - 1 ))"); do
      echo frames/plot-"$(printf %06d "$frameIndex")".png
    done
  fi
  
  frameCount=$targetFrameCount

done < \
  <(git -C "$nixpkgs" log --first-parent --format='%H%n%ct' --name-only --follow "$lastCommit" -- "$file")

