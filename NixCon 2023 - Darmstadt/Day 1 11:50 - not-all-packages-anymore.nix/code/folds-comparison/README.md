This was used to generate the plot comparing the linear and the tree folds over time.

To generate the data in `data.json` from `~/src/nixpkgs` (needs to have commit `5fc95eef55ead8b721808f7f235c6fb032e68276`), this takes a couple minutes:
```
./data.sh
```

To generate the plot from the `data.json` in `plot.png`, this uses [Vega-Lite](https://vega.github.io/vega-lite/) locally:
```
./plot.sh
```
