#!/bin/bash
export DOCKER_BUILDKIT=1
docker build -t nutrient_calculator --build-arg FEATURES=tui .
