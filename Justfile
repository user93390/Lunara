#!/usr/bin/env just --justfile

# Cargo helper functions
build: clean
  cargo build --release

check:
  cargo check --release

clean: dock_stop
  cargo clean --release

# Docker helper functions
dock_init:
  docker build -t lunara .

dock_compose:
  docker-compose up -d

# Not really recommended.
kill_force:
  docker-compose down -v --rmi all --remove-orphans

# This better.
dock_stop:
  docker-compose down

dock_auto: build_all dock_compose
build_all: clean build dock_init