#!/usr/bin/env bash

set -e

# Required because `brew tap-new` initializes a Git repository.
git config --global user.name "Hanno Braun"
git config --global user.email "hanno@braun-odw.eu"

TAP=hannobraun/deno
MODULE=deno
VERSION=1.14.3

brew tap-new $TAP
brew extract --version $VERSION $MODULE $TAP
brew install $TAP/$MODULE@$VERSION

zola build
deno run -A https://deno.land/x/lume/ci.ts
