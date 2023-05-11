# Start from a Rust base image
FROM rust:1.67 as builder


# Create a new empty shell project
WORKDIR /usr/src/app

# Copy over the source code
COPY . .

# Build the project with the release profile
RUN cargo build --release

# Start a new stage with a minimal image
# Because we're only bringing a single binary over, we don't actually need any dependencies at runtime
FROM alpine:3.14

# Copy over the build artifact from the previous stage and set the entry point
COPY --from=builder /usr/src/app/target/release/kalz_websocket_server /usr/local/bin

CMD ["kalz_websocket_server"]
