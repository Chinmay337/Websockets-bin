# Rust Websockets Server

High Performance Websockets Server written in `Rust`.

Includes a simple client for sending and receiving messages written in Pure `Vanilla Javascript`.

## Features

- Simple Design and Implementation.

- Broadcasts messages to all connected clients.

- Buffers messages in memory.

## Running

```bash
# Running Locally
cargo run

# Deploying

# Build Binary
cargo build

# Deploy Binaries to Servers
```

## Production

- Deploy binaries to globally distributed servers.

- Use `Georouting` based off of `DNS` or `IP Header Information` to route to nearest Server for `Lowest Latency` and `Optimal Performance`

- Use existing Client code in `ws-client/` as a starting point for your WebSockets applications to build on top of.
