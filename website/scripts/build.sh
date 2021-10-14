#!/usr/bin/env bash

set -e

brew install https://raw.githubusercontent.com/Homebrew/homebrew-core/d77989e0193c1c9dd9aa462d4179d11653e41ca5/Formula/deno.rb

zola build
deno run -A https://deno.land/x/lume/ci.ts
