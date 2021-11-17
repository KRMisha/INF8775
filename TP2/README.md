# TP2

## Generate data

```sh
./generate_instances.sh
```

The generated instances will be saved to the `data/generated/` directory.

## Build implementation

```sh
cd implementation
cargo build --release
```

## Measure and save execution times and graph coloring results for algorithms

```sh
./analyze.py measure
```

## Generate graphs and equations for power, ratio, and constants tests

```sh
./analyze.py complexity
```
