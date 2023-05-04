FROM rust:1.68.2 AS builder

# create a new empty shell project
RUN USER=root cargo new --bin portal-interop
WORKDIR /portal-interop

# copy over manifests and source to build image
COPY Cargo.toml ./Cargo.toml
COPY src ./src

# build for release
RUN cargo build --release

# final base
FROM ubuntu:22.04

RUN apt update && apt install wget -y

RUN wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.0g-2ubuntu4_amd64.deb
RUN dpkg -i libssl1.1_1.1.0g-2ubuntu4_amd64.deb

# copy build artifacts from build stage
COPY --from=builder /portal-interop/target/release/portal-interop .

ENV RUST_LOG=debug

ENTRYPOINT ["./portal-interop"]
