FROM rust:1.83.0 AS builder

WORKDIR /usr/src/RUST-Shell

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

RUN ls -la /usr/src/RUST-Shell/target/release/

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/RUST-Shell/target/release/RUST-Shell /usr/local/bin/RUST-Shell

ENTRYPOINT ["RUST-Shell"]