#!/usr/bin/env bash

set -e

UNIT_DIR=/usr/local/lib/systemd/system
SERVE_DIR=/srv/made-by.braun-odw.eu

cargo build --release

mkdir -p $UNIT_DIR
rm -r $SERVE_DIR

cp -f target/release/backend /usr/local/bin
cp backend.service $UNIT_DIR
cp -r static $SERVE_DIR

systemctl enable backend.service
systemctl restart backend.service
