#!/bin/bash
export DOCKER_BUILDKIT=1
docker build -t rustoku --build-arg FEATURES=tui .
