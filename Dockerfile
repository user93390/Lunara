FROM rust:alpine AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev bash curl git libstdc++ libgcc

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

FROM alpine:latest AS flutter_builder

WORKDIR /flutter_app

RUN apk add --no-cache \
    bash \
    ca-certificates \
    curl \
    git \
    unzip \
    xz \
    libc6-compat \
    gcompat \
    libstdc++ \
    libgcc

RUN git clone https://github.com/flutter/flutter.git /flutter
ENV PATH="/flutter/bin:${PATH}"

RUN flutter config --no-analytics
RUN flutter doctor

COPY flutter ./

RUN flutter build web --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /build/bin/Lunara /usr/local/bin/Lunara
COPY database.properties /app/database.properties
COPY --from=flutter_builder /flutter_app/build/web /app/flutter/build/web

CMD ["Lunara"]