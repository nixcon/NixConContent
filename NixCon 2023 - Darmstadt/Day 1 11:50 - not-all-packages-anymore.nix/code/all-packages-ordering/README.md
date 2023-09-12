This was used to generate the plot of the ordering in `all-packages.nix`

To generate the data in `data.json` from `~/src/nixpkgs`:
```
./data.sh
```

To generate the plot from the `data.json` in `plot.png`, this uses [Vega-Lite](https://vega.github.io/vega-lite/) locally:
```
./plot.sh
```

# Simulation over time

See [./history](./history/README.md)
