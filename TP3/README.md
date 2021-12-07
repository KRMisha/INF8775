# TP3

## Build implementation

```sh
cd implementation
cargo build --release
```

## Run implementation

```sh
./tp.sh -e <filename>
```

## Run implementation for 3 minutes and check solution validity

```sh
{timeout 180s ./tp.sh -e <filename> -p; exit 0} | ./check_sol.py -e <filename> -s /dev/stdout
```
