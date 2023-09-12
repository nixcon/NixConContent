#!/usr/bin/env nix-shell
#!nix-shell -i bash -p nodePackages.vega-cli

vg2png -s 3 plot.json > plot.png
