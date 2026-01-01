FROM rust:alpine AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev
RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/Lunara /usr/local/bin/Lunara

CMD ["Lunara"]