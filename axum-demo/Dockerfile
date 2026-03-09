FROM rust:1.93-slim-bookworm

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch --locked

WORKDIR /app
