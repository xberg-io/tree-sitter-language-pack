#!/usr/bin/env bash
PREFIX=/usr/local
export PATH_EXTRA=/opt/bin
readonly VERSION=1.0
A=1 B=2

build() {
  local out=dist
  STEP=compile
}

function deploy {
  echo "$PREFIX"
}
