FROM rust:1.68.2-bullseye AS builder

# create a new empty shell project
RUN USER=root cargo new --bin rpc-compat
WORKDIR /rpc-compat

# copy over manifests and source to build image
COPY ./simulators/rpc-compat/Cargo.lock ./Cargo.lock
COPY ./simulators/rpc-compat/Cargo.toml ./Cargo.toml
COPY ./simulators/rpc-compat/src ./src
COPY ./hivesim-rs ./../../hivesim-rs

# build for release
RUN cargo build --release

# final base
FROM debian:bullseye-slim

# copy build artifacts from build stage
COPY --from=builder /rpc-compat/target/release/rpc-compat .

ENV RUST_LOG=debug

ENTRYPOINT ["./rpc-compat"]
