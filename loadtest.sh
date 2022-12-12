#!/usr/bin/env bash

# Run this after docker compose shows kong as healthy

set -x

{
    date
    hey -c 8 -z 2m http://localhost/go
    date
} > metrics.log &

sleep 1
docker compose exec kong kong reload

wait

cat metrics.log
