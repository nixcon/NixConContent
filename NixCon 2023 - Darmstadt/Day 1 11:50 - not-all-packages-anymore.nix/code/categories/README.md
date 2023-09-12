This was used to generate the plot of the category hierarchy in Nixpkgs.

To generate the plot from the `data.json` in `plot.png`, this uses [Vega-Lite](https://vega.github.io/vega-lite/) locally:
```
./plot.sh
```

The generate the initial data in `data.json` from `~/src/nixpkgs`, this takes a couple minutes (this has a lot of optimisation potential though):
```
./data.sh
```

Afterwards the `data.json` file needs to be manually edited to remove entries that aren't categories. Also the numbers may be swapped a bunch to make parts of the graph be swapped.
