#!/bin/bash
set -ex

SCRIPT_DIR="$( cd "$(dirname "$0")" ; pwd -P )"
BASE_IMAGE="dlt-shortener-base:0.0.1"

docker build -f $SCRIPT_DIR/Dockerfile.base -t $BASE_IMAGE $SCRIPT_DIR/..
docker build -f $SCRIPT_DIR/Dockerfile.test -t dlt-shortener-test:0.0.1 --build-arg "BASE_IMAGE=$BASE_IMAGE" $SCRIPT_DIR/..
