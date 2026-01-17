#!/usr/bin/env just --justfile

# Cargo helper functions
build:
  cargo build --release    
  cd flutter && flutter build web --wasm

check:
  cargo check --release

clean:
  cargo clean --release

  cd flutter && flutter clean

run:
  cargo run -- release

# Docker helper functions

build_front:
  cd flutter && flutter build web

dock_init: build_front
  docker build -t lunara .

dock_compose:
  docker-compose up -d

build_all: dock_stop clean build dock_init dock_compose

# Not really recommended.
kill_force:
  docker-compose down -v --rmi all --remove-orphans

# This better.
dock_stop:
  docker-compose down