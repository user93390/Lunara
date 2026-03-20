FROM rust:alpine AS builder

WORKDIR /app

COPY init.sql /docker-entrypoint-initdb.d/init.sql

ENV CI=true

COPY . .

# Build runtime binary
RUN cargo build --release

FROM alpine:latest

WORKDIR /app

# Add keyctl
RUN apk add --no-cache keyutils

COPY --from=builder /app/target/release/Lunara /app/lunara
RUN chmod +x /app/lunara

ENTRYPOINT ["keyctl", "session", "-", "/app/lunara"]
