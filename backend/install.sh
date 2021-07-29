#!/usr/bin/env bash

set -e

cargo build --release

cp target/release/backend /usr/local/bin
cp backend.service /usr/local/lib/systemd/system

systemctl enable backend.service
systemctl start backend.service
