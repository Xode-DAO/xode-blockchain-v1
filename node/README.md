# Node

ℹ️ A node -  in Polkadot - is a binary executable, whose primary purpose is to execute the [runtime](../runtime/README.md).

👇 Here are the most important files in this node template:

- [`chain_spec.rs`](./src/chain_spec.rs): A chain specification is a source code file that defines the chain's
initial (genesis) state.
- [`service.rs`](./src/service.rs): This file defines the node implementation.
It's a place to configure consensus-related topics.
