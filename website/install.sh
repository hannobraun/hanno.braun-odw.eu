#!/usr/bin/env bash

set -e

UNIT_DIR=/usr/local/lib/systemd/system
SERVE_DIR=/srv/made-by.braun-odw.eu
STATIC_DIR=$SERVE_DIR/static
ZOLA_DIR=$SERVE_DIR/zola

cargo build --release
sudo cp -f ../target/release/website /usr/local/bin

sudo mkdir -p $UNIT_DIR
sudo cp backend.service $UNIT_DIR
sudo systemctl enable backend.service
sudo systemctl restart backend.service

sudo rm -rf $SERVE_DIR
sudo mkdir -p $SERVE_DIR

sudo cp -r static/ $STATIC_DIR

(cd zola; zola build)
sudo cp -r zola/public/ $ZOLA_DIR
