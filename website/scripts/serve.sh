#!/usr/bin/env bash

set -e

zola build
lume --serve
