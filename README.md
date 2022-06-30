# Humidefi Node

Freshly hacked FRAME-based [Substrate](https://www.substrate.io/) node. :rocket:

Features:
- Rewarded Proof-of-Authority (PoA)
- Smart Contract

More information in my blog: [Blog](https://hgminerva.wordpress.com/)

## Getting Started

Follow the steps below to get started with the Humidefi Node.

## Setup 

Use Ubuntu 22.04

```sh
$ git clone https://github.com/hgminerva/humidefi-node.git
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
$ rustc --version
$ rustup default stable
$ rustup update
$ rustup update nightly
$ rustup target add wasm32-unknown-unknown --toolchain nightly
$ rustup show
$ rustup +nightly show
```

## Build 

```sh
$ cd  humidefi-node
$ cargo b -r
```

## Run Node in Development Mode

```sh
$ ./substrate-dev-run.sh
```

## Purge Node

If you are purging the bootnode (node01).  Purge the database then the keystore.

```sh
$ ./target/release/node-template purge-chain --base-path /tmp/node01
$ rm -rf /tmp/node01/chains/local_testnet/keystore
```

## Generate Aura (SR25519) and Grandpa (ED25519) keys

Make sure you install Subkey utility first.

```sh
$ subkey generate --scheme sr25519
$ subkey inspect --scheme ed25519 "<generated_secret_phrase>"
```

## Run Node in Testnet Mode

Make sure to run the two shell into two separate terminals.  Sometimes the node id of the bootnode will change, if so modify the node02 shell to match the bootnode node id.

```sh
$ ./substrate-testnet-bootnode-run.sh
$ ./substrate-testnet-node02-run.sh
```