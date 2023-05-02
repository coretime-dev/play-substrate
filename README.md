# Play Substrate

A few simple pallets for learning and demo the capability of Substrate blockchain framework.

## Runtime Modules

Disclaimer: these modules are for learning purpose, you should never use them in *production*.

- **Template Pallet**, A simple module to experience simple storage data type, error handling, test and benchmark structure.
- **Proof of Existence Pallet**, A simple module to store a claim (BoundedVec\<u8\>), revoke it, and transfer the claim to someone else. This pallet includes,
  - busisness logic to create/revoke/transfer a claim.
  - unit test of logic
  - benchmark of weights used for each dispatchables.
- **Data Type Pallet**, It helps with better understanding what Rust types are supported in Substrate blockchain development.
- **Coin Flip Game Pallet**, *Outdated*
- **Benchmark Demo**, *Deprecated*, you can find similar code in Proof of Existence pallet.
- **Offchain Worker**
  - Offchain worker - send unsigned transaction (outdated)
  - Offchain worker - send signed transaction (outdated)
- **Genesis Config Pallet**, *Outdated*
- **Custome Weight**, *Outdated*

## Getting Started

### Install Dependencies

Before you start developing with Substrate, you need to prepare your environment by following the [doc](https://docs.substrate.io/main-docs/install/).

### Compile and Run

```shell
# compile the code into binary
cargo build --release

# start the compiled binary with local dev network
./target/release/node-template --dev
```
### Connect with Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### Test the code

```shell
# test only one pallet
cargo test -p pallet-poe

# test all the pallets
cargo test
```

### Benchmark

Compile the node with `runtime-benchmarks` feature,
```shell
cargo build --release --features runtime-benchmarks
```

Run the benchmark for Proof of Existence pallet,
```shell
./target/release/node-template benchmark pallet --chain dev --execution wasm --wasm-execution compiled --pallet pallet_poe --extrinsic "*" --steps 20 --repeat 10 --output ./pallets/poe/src/weights.rs --template .maintain/frame-weight-template.hbs
```

Show all the available benchmarks,
```shell
./target/release/node-template benchmark pallet --chain dev --pallet "*" --extrinsic "*" --repeat 0
```
