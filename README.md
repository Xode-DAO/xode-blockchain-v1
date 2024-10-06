<div align="center">

# Xode Blockchain

</div>

## Xode Blockchain Structure

Xode Blockchain is consists of:

* 💿 a [Node](./node/README.md) - the binary application.
* 🧮 the [Runtime](./runtime/README.md) - the core logic of the parachain.
* 🎨 the [Pallets](./pallets/README.md) - from which the runtime is constructed.

## Getting Started

* 🦀 Xode Blockchain is using the Rust language.

### Build

🔨 Use the following command to build the node without launching it:

```sh
cargo build --package xode-node --release
```

🐳 Alternatively, build the docker image:

```sh
docker build . -t xode-node
```

## Getting Help

* 🧑‍🏫 To learn about Xode in general, [Xode Blockchain](https://wiki.xode.net/).

