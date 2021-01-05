# Benchmarking Demo

## Run

```shell
cd node

build --release --features runtime-benchmarks

# benchmark balances pallet
./target/release/node-template benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_balances --extrinsic transfer --steps 50 --repeat 20 --raw > balances_transfer.txt

# benchmark our demo
./target/release/node-template benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_benchmark_demo --extrinsic do_something --steps 50 --repeat 20
```

## Resources

[Substrate Benchmarking Documentation by Shawn](https://www.shawntabrizi.com/substrate-graph-benchmarks/docs/#/)