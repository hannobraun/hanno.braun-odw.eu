#!/usr/bin/env bash

set -e

UNIT_DIR=/usr/local/lib/systemd/system
SERVE_DIR=/srv/made-by.braun-odw.eu/static

cargo build --release
sudo cp -f target/release/backend /usr/local/bin

sudo mkdir -p $UNIT_DIR
sudo cp backend.service $UNIT_DIR
sudo systemctl enable backend.service
sudo systemctl restart backend.service

(cd zola; zola build)
sudo rm -rf $SERVE_DIR
sudo mkdir -p $SERVE_DIR
sudo cp -r zola/public/* static/* -t $SERVE_DIR
