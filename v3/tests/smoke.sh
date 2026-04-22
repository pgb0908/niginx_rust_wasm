#!/usr/bin/env bash

set -eu

BASE_URL="${BASE_URL:-http://localhost:8082}"

echo "[1] protected route"
curl -i -H 'x-api-key: users-secret' -H 'x-tenant-id: team-a' "${BASE_URL}/users/10"

echo
echo "[2] public route"
curl -i -H 'x-tenant-id: team-b' "${BASE_URL}/orders/99"

echo
echo "[3] unauthorized route"
curl -i -H 'x-tenant-id: team-a' "${BASE_URL}/users/10"
