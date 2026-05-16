#!/usr/bin/env bash
# reset.sh — Reset the GuardianProxy circuit breaker after an incident
set -euo pipefail

source "$(dirname "$0")/../backend/.env" 2>/dev/null || true

NETWORK="${NETWORK:-testnet}"
CONTRACT_ID="${GUARDIAN_PROXY_CONTRACT_ID:?Set GUARDIAN_PROXY_CONTRACT_ID}"
ADMIN_SECRET="${ADMIN_SECRET_KEY:?Set ADMIN_SECRET_KEY}"

stellar contract invoke \
  --id "$CONTRACT_ID" --source "$ADMIN_SECRET" --network "$NETWORK" \
  -- reset

echo "✅ Circuit breaker reset."
