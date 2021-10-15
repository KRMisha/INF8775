# TP1

## Generate data

```sh
mkdir data
cd data
../inst_gen.py -S 5 -t 7 -n 5 -r 8775
```

## Build implementation

```sh
cd implementation
cargo build --release
```

## Analyze execution time results

```sh
./analyze.py
```
