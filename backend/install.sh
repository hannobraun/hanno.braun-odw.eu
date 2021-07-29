#!/usr/bin/env bash

set -e

UNIT_DIR=/usr/local/lib/systemd/system

cargo build --release

cp target/release/backend /usr/local/bin
cp backend.service $UNIT_DIR

systemctl enable backend.service
systemctl start backend.service
