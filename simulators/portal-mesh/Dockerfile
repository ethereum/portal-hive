FROM rust:1.71.1 AS builder

# create a new empty shell project
RUN USER=root cargo new --bin portal-mesh
WORKDIR /portal-mesh

# copy over manifests and source to build image
COPY ./simulators/portal-mesh/Cargo.lock ./Cargo.lock
COPY ./simulators/portal-mesh/Cargo.toml ./Cargo.toml
COPY ./simulators/portal-mesh/src ./src
COPY ./hivesim-rs ./../../hivesim-rs

# build for release
RUN cargo build --release

# final base
FROM ubuntu:22.04

RUN apt update && apt install wget -y

# copy build artifacts from build stage
COPY --from=builder /portal-mesh/target/release/portal-mesh .

ENV RUST_LOG=debug

ENTRYPOINT ["./portal-mesh"]
