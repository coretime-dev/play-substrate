# Proof of Existence

## Run benchmark

```shell
# compile with runtime-benchmarks feature
cargo build --release --features runtime-benchmarks

# benchmark dispatchables in poe pallet
# download frame-weight-template.hbs from [Substrate repo](https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs).
./target/release/node-template benchmark pallet --chain dev --execution wasm --wasm-execution compiled --pallet pallet_poe --extrinsic "*" --steps 20 --repeat 10 --output ./pallets/poe/src/weights.rs --template .maintain/frame-weight-template.hbs
```
