FROM rust:alpine AS builder

WORKDIR /app

COPY init.sql /docker-entrypoint-initdb.d/init.sql

RUN apk add --no-cache nodejs npm
RUN npm install -g pnpm

COPY web ./web
RUN cd web && pnpm install --frozen-lockfile && pnpm run build
RUN mkdir -p static && cp -r web/dist/* static/

COPY Cargo.toml ./
COPY src ./src
COPY static ./static

RUN cargo install --path . --root /build

FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache keyutils

COPY --from=builder /build/bin/Lunara /usr/local/bin/Lunara
COPY --from=builder /app/static /app/static

CMD sh -c "keyctl session && Lunara"
