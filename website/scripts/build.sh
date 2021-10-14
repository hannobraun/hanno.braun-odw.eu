#!/usr/bin/env bash

set -e

curl -fsSL https://deno.land/x/install/install.sh | sh -s v1.0.0

export DENO_INSTALL="/opt/buildhome/.deno"
export PATH="$DENO_INSTALL/bin:$PATH"

zola build
deno run -A https://deno.land/x/lume/ci.ts
