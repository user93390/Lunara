#!/bin/sh

packages="git curl gnome-keyring just docker docker-compose"

for value in $packages
do
  apk add "$value"
done

addgroup "$(whoami)" docker
