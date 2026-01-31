#!/bin/sh

keyctl new_session

exec "$@"
