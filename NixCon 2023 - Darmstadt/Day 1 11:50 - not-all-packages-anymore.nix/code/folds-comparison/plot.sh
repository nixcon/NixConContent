#!/usr/bin/env nix-shell
#!nix-shell -i bash -p nodePackages.vega-lite

vl2png -s 4 plot.json > plot.png
