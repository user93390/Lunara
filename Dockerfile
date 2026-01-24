FROM rust:alpine AS builder

WORKDIR /app

COPY init.sql /docker-entrypoint-initdb.d/init.sql

COPY Cargo.toml ./
COPY src ./src

RUN cargo install --path . --root /build

FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache keyutils

COPY --from=builder /build/bin/Lunara /usr/local/bin/Lunara

CMD sh -c "keyctl session && Lunara"
