#!/usr/bin/env bash

set -e

UNIT_DIR=/usr/local/lib/systemd/system
STATIC_DIR=/srv/made-by.braun-odw.eu/static

cargo build --release
sudo cp -f target/release/backend /usr/local/bin

sudo mkdir -p $UNIT_DIR
sudo cp backend.service $UNIT_DIR
sudo systemctl enable backend.service
sudo systemctl restart backend.service

(cd zola; zola build)
sudo rm -rf $STATIC_DIR
sudo mkdir -p $STATIC_DIR
sudo cp -r zola/public/* static/* -t $STATIC_DIR
