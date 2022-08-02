#!/bin/bash
set -ex

DOCKER_IMG_NAME=${1:-"dlt-shortener"}
VERSION=${2:-"0.0.1"}

SCRIPT_DIR="$( cd "$(dirname "$0")" ; pwd -P )"
BASE_IMAGE="dlt-shortener-base:$VERSION"
FULL_DOCKER_IMG_NAME="$DOCKER_IMG_NAME:$VERSION"

docker build -f $SCRIPT_DIR/Dockerfile.base -t $BASE_IMAGE $SCRIPT_DIR/..
docker build -f $SCRIPT_DIR/Dockerfile.prod -t $FULL_DOCKER_IMG_NAME --build-arg "BASE_IMAGE=$BASE_IMAGE" $SCRIPT_DIR/..
