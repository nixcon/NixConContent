This was used to generate the video of the ordering in `all-packages.nix` over time

To generate the frames in `./frames` from `~/src/nixpkgs` (needs to have commit `5fc95eef55ead8b721808f7f235c6fb032e68276`), this takes a couple minutes:
```
./data.sh
```

To render the video from the `./frames` in `video.mp4`, this uses ffmpeg:
```
./render.sh
```
