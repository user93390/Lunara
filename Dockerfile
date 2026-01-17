FROM rust:alpine AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev

# linux and windows. x64 targets
RUN rustup target add x86_64-unknown-linux-musl x86_64-pc-windows-msvc 

# ARM systems. Optional but recommended.
RUN rustup target add aarch64-unknown-linux-gnu \ 
    aarch64-apple-darwin \ 
    aarch64-pc-windows-msvc

COPY Cargo.toml ./
COPY src ./src
COPY database.properties ./

# Useful tools
RUN rustup component add rust-docs

RUN cargo install --path . --root /build

FROM alpine:latest

WORKDIR /app

COPY --from=builder /build/bin/Lunara /usr/local/bin/Lunara
COPY database.properties /app/database.properties
COPY flutter/build/web /app/flutter/build/web

CMD ["Lunara"]