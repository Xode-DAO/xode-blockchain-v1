# Zombienet (Development Platform)

ℹ️ Zombienet is a testing tool for the Polkadot and Substrate ecosystems. It allows developers to spin up and test local, multi-node networks (such as parachains or validators) in a fast and efficient way. 

## Steps
1. Download the polkadot, zombienet and xode binaries for your operating system.
2. Run chmod +x all binaries and script
3. Run cargo build --release at the root
4. Run ./zombienet-launch.sh
5. Open the appropriate link in polkadotJS

## Polkadot binaries
https://github.com/paritytech/polkadot/releases

## Compiling  Polkadot
```
git clone https://github.com/paritytech/polkadot
cd polkadot
git checkout release-v1.0.0
cargo build --release
```

## Zombienet binaries
https://github.com/paritytech/zombienet/releases