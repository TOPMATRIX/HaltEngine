# HaltEngine

**Automated Circuit Breaker & Smart Contract Emergency System for Stellar**

HaltEngine is an open-source developer tool that acts as an external "smart fuse" for Stellar DApps. It watches a contract's balance in real time and automatically triggers an on-chain halt when a drain event is detected — and is the first platform designed to dynamically draft and submit **CAP-0077 Quorum Freeze** state machine parameters to network validators during severe contract drains.

---

## Structure

```
HaltEngine/
├── contracts/
│   └── guardian-proxy/        # Soroban circuit breaker contract
│       └── src/lib.rs
├── backend/                   # Rust log scraper + halt trigger
│   └── src/
│       ├── main.rs            # Polling loop
│       ├── scraper.rs         # Horizon balance fetcher
│       ├── rpc.rs             # Soroban JSON-RPC client
│       ├── halt.rs            # trigger_halt invocation
│       └── cap0077.rs         # CAP-0077 quorum freeze drafter
├── frontend/                  # React emergency console
│   └── src/
│       ├── App.tsx
│       ├── components/
│       │   ├── PanicButton.tsx   # Manual halt trigger
│       │   ├── ResetButton.tsx   # Admin reset
│       │   └── StatusBadge.tsx   # HALTED / ACTIVE indicator
│       ├── hooks/
│       │   └── useContractState.ts
│       └── lib/
│           └── stellar.ts        # Soroban tx builder
├── scripts/
│   ├── deploy.sh              # Build WASM + deploy + initialize
│   └── reset.sh               # Admin reset
├── .github/workflows/ci.yml
├── Dockerfile                 # Backend container
└── docker-compose.yml
```

---

## How It Works

1. **Monitor** — The Rust backend polls the watched contract's XLM balance via Horizon every `POLL_INTERVAL_MS` ms.
2. **Detect** — If the balance drops more than `THRESHOLD_BPS` basis points from the baseline, a drain is flagged.
3. **Halt** — The guardian keypair invokes `trigger_halt` on the GuardianProxy Soroban contract, recording the halt on-chain.
4. **CAP-0077** — Simultaneously, the backend drafts a quorum freeze parameter package and broadcasts it to configured validator endpoints. Freeze duration scales with severity:
   - 20–40% drain → 120 ledgers (~10 min)
   - 40–70% drain → 720 ledgers (~1 hr)
   - >70% drain   → 4320 ledgers (~6 hr)
5. **Console** — The React emergency console lets admins manually trigger a halt (Panic Button) or reset the circuit breaker.

---

## GuardianProxy Contract API

| Function | Auth | Description |
|---|---|---|
| `initialize(admin, guardian, watched, threshold_bps)` | admin | One-time setup |
| `set_baseline(baseline)` | admin | Record healthy balance |
| `trigger_halt(current_balance)` | guardian | Halt if drain ≥ threshold |
| `reset()` | admin | Clear halt flag |
| `set_guardian(new_guardian)` | admin | Rotate guardian key |
| `set_threshold(threshold_bps)` | admin | Update drain threshold |
| `is_halted()` | — | Read halt state |
| `get_threshold()` / `get_watched()` / `get_guardian()` / `get_baseline()` | — | View functions |

---

## Quick Start

### Prerequisites
- Rust + `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli)
- Node.js 20+

### 1. Deploy the contract

```bash
cp backend/.env.example backend/.env
# Fill in ADMIN_SECRET_KEY, GUARDIAN_PUBLIC_KEY, WATCHED_CONTRACT
bash scripts/deploy.sh
# → outputs GUARDIAN_PROXY_CONTRACT_ID
```

### 2. Start the backend

```bash
# Add GUARDIAN_PROXY_CONTRACT_ID and GUARDIAN_SECRET_KEY to backend/.env
cargo run -p halt-engine-backend
```

### 3. Start the frontend

```bash
cp frontend/.env.example frontend/.env
# Add VITE_GUARDIAN_PROXY_CONTRACT_ID
cd frontend && npm install && npm run dev
```

### 4. Docker (backend only)

```bash
docker compose up --build
```

### 5. Reset after an incident

```bash
bash scripts/reset.sh
```

---

## Security

- The **guardian key** is limited to `trigger_halt` only — keep it in the backend env.
- The **admin key** controls reset and config — use a hardware wallet in production.
- CAP-0077 validator endpoints should use mTLS or API key auth in production.

---

## License

MIT
