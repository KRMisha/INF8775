# TP1

## Generate data

```sh
mkdir data
cd data
../inst_gen.py -S 3 -t 8 -n 5 -r 8775
```

## Build implementation

```sh
cd implementation
cargo build --release
```

## Compare execution time for different Strassen thresholds

```sh
./analyze.py threshold
```

## Measure and save execution times for algorithms

```sh
./analyze.py measure
```

## Generate graphs and equations for power, ratio and constants tests

```sh
./analyze.py complexity
```
