#!/usr/bin/env bash
# deploy.sh — Build WASM and deploy GuardianProxy to Stellar testnet
set -euo pipefail

source "$(dirname "$0")/../backend/.env" 2>/dev/null || true

NETWORK="${NETWORK:-testnet}"
ADMIN_SECRET="${ADMIN_SECRET_KEY:?Set ADMIN_SECRET_KEY}"
GUARDIAN_PUB="${GUARDIAN_PUBLIC_KEY:?Set GUARDIAN_PUBLIC_KEY}"
WATCHED="${WATCHED_CONTRACT:?Set WATCHED_CONTRACT}"
THRESHOLD="${THRESHOLD_BPS:-2000}"

echo "==> Building contract WASM..."
cargo build -p guardian-proxy --target wasm32-unknown-unknown --release

WASM="$(dirname "$0")/../target/wasm32-unknown-unknown/release/guardian_proxy.wasm"

echo "==> Deploying to $NETWORK..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM" \
  --source "$ADMIN_SECRET" \
  --network "$NETWORK")

echo "Contract ID: $CONTRACT_ID"

ADMIN_PUB=$(node -e "const s=require('@stellar/stellar-sdk');console.log(s.Keypair.fromSecret('$ADMIN_SECRET').publicKey())")

echo "==> Initializing..."
stellar contract invoke \
  --id "$CONTRACT_ID" --source "$ADMIN_SECRET" --network "$NETWORK" \
  -- initialize \
  --admin "$ADMIN_PUB" --guardian "$GUARDIAN_PUB" \
  --watched "$WATCHED" --threshold_bps "$THRESHOLD"

echo ""
echo "✅ GuardianProxy deployed: $CONTRACT_ID"
echo "Add to backend/.env:  GUARDIAN_PROXY_CONTRACT_ID=$CONTRACT_ID"
echo "Add to frontend/.env: VITE_GUARDIAN_PROXY_CONTRACT_ID=$CONTRACT_ID"
