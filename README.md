# Cash:web Payment Server

The goal is to provide a safe, high performance, and quick to setup service which compliments your existing service stack allowing you to easily accept Bitcoin payments via the BIP70 protocol.

### Setting up Bitcoin

Bitcoin must be run with [RPC](https://bitcoin.org/en/developer-reference#remote-procedure-calls-rpcs) and block hash [ZMQ](https://github.com/bitcoin/bitcoin/blob/master/doc/zmq.md) enabled.

### Build

Install [Rust](https://www.rust-lang.org/tools/install) then

```bash
sudo apt install -y clang pkg-config libssl-dev libzmq3-dev
cargo build --release
```

The executable will be located at `./target/release/payment-server`.
