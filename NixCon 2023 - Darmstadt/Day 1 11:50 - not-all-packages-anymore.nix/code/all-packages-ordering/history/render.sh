#!/usr/bin/env nix-shell
#!nix-shell -i bash -p ffmpeg
set -euo pipefail

rm -rf frames-chronological
mkdir -p frames-chronological

total=$(ls frames | wc -l)
index=$(( total - 1 ))

# We need to reverse the frames
while (( index >= 0 )); do
  reverse=$(( total - index ))
  cp frames/plot-"$(printf %06d "$index")".png \
    frames-chronological/plot-"$(printf %06d "$reverse")".png
  (( index-- )) || true
done

ffmpeg -r 60 -i frames-chronological/plot-%06d.png -pix_fmt yuv420p -vsync 0 video.mp4
