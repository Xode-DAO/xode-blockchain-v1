<div align="center">

# Xode Blockchain

</div>

## Xode Blockchain Structure

Xode Blockchain is consists of:

* The [Node](./node/README.md) - xode-node binary application.
* The [Runtime](./runtime/README.md) - xode-runtime WASM application.
* The [Pallets](./pallets/README.md) - xode pallets.
    * xode-consensus 
    * xode-staking
* The [Test](./zombienet/README.md) - xode test network (zombienet).

## Getting Started

* 🦀 Xode Blockchain is using the Rust language.

### Build

🔨 Use the following command to build and launch the node:

```sh
cargo build --package xode-node --release
./zombienet/zombienet-launch.sh
```

## Getting Help

* 🧑‍🏫 To learn about Xode in general, [Xode Blockchain](https://wiki.xode.net/).

