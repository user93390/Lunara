FROM rust:alpine AS builder

WORKDIR /app

COPY init.sql /docker-entrypoint-initdb.d/init.sql

ENV CI=true

COPY . .

RUN cargo test
RUN cargo build

FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache keyutils

# Setup keyring service
RUN keyctl new_session

ENTRYPOINT ["/entrypoint.sh"]
CMD ["Lunara"]
