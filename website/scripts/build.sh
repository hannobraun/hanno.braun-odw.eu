#!/usr/bin/env bash

set -e

brew install deno@1.14.3

zola build
deno run -A https://deno.land/x/lume/ci.ts
