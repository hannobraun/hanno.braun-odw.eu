#!/usr/bin/env bash

set -e

UNIT_DIR=/usr/local/lib/systemd/system
SERVE_DIR=/srv/made-by.braun-odw.eu

cargo build --release
cp -f target/release/backend /usr/local/bin

mkdir -p $UNIT_DIR
cp backend.service $UNIT_DIR

(cd zola; zola build)
rm -r $SERVE_DIR
mkdir -p $SERVE_DIR
cp -r zola/public/* static/* -t $SERVE_DIR

systemctl enable backend.service
systemctl restart backend.service
