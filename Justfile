#!/usr/bin/env just --justfile

# Cargo helper functions
build:
  cargo build --release    

check:
  cargo check --release

clean:
  cargo clean --release

run:
  cargo run -- release

# Docker helper functions
docker-init:
  docker build -t lunara .

docker-compose:
  docker compose up -d