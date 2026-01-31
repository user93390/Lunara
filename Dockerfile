FROM rust:alpine AS builder

WORKDIR /app

COPY init.sql /docker-entrypoint-initdb.d/init.sql

ENV CI=true

RUN apk add --no-cache npm
RUN npm install -g bun

RUN apk add make

COPY . .

RUN make build

FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache keyutils perl

COPY --from=builder /app/target/release/Lunara /usr/local/bin/Lunara
COPY --from=builder /app/static /app/static
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
CMD ["Lunara"]