FROM rust:1.68.2 AS builder

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
FROM ubuntu:22.04

RUN apt update && apt install wget -y

# Use these for amd-based devices
RUN wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.0g-2ubuntu4_amd64.deb
RUN dpkg -i libssl1.1_1.1.0g-2ubuntu4_amd64.deb
# Use these for arm-based devices
#RUN wget http://ports.ubuntu.com/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2_arm64.deb
#RUN dpkg -i libssl1.1_1.1.1f-1ubuntu2_arm64.deb

# copy build artifacts from build stage
COPY --from=builder /rpc-compat/target/release/rpc-compat .

ENV RUST_LOG=debug

ENTRYPOINT ["./rpc-compat"]
