FROM rust:alpine AS builder

WORKDIR /app

COPY init.sql /docker-entrypoint-initdb.d/init.sql

ENV CI=true

COPY . .

# Test & Build
RUN cargo test
RUN cargo build --release

FROM alpine:latest

WORKDIR /app

# Add keyctl
RUN apk add keyutils

ENTRYPOINT ["keyctl", "session", "-", "Lunara"]
