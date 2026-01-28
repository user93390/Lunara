#!/usr/bin/env just --justfile

# Cargo helper functions
build: clean
  # Build frontend
  cd web && pnpm install && pnpm run build
  rm -rf static && mkdir -p static
  cp -r web/dist/* static/
  # Build backend
  cargo build --release

check:
  cargo check --release

clean: dock_stop
  cargo clean --release
  rm -rf static
  rm -rf web/dist
  rm -rf web/node_modules

# Docker helper functions
dock_init:
  cd web && pnpm install --lockfile-only
  cargo generate-lockfile
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