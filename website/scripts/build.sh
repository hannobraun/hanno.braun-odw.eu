#!/usr/bin/env bash

set -e

brew install deno

zola build
deno run -A https://deno.land/x/lume/ci.ts
