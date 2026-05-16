//! GuardianProxy — Soroban Circuit Breaker Contract
//!
//! Wraps a client DApp and enforces an emergency halt when a drain threshold
//! is breached. The off-chain backend triggers halts; the admin resets them.

#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

// ── Storage keys ─────────────────────────────────────────────────────────────
const ADMIN: Symbol    = symbol_short!("ADMIN");
const GUARDIAN: Symbol = symbol_short!("GUARDIAN");
const WATCHED: Symbol  = symbol_short!("WATCHED");
const HALTED: Symbol   = symbol_short!("HALTED");
const THRESHOLD: Symbol= symbol_short!("THRESH");
const BASELINE: Symbol = symbol_short!("BASELINE");

// ── Events ────────────────────────────────────────────────────────────────────
const EVT_HALT:  Symbol = symbol_short!("halt");
const EVT_RESET: Symbol = symbol_short!("reset");
const EVT_DRAIN: Symbol = symbol_short!("drain");

// ── Types ─────────────────────────────────────────────────────────────────────
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct HaltStatus {
    pub halted: bool,
    pub triggered_by: Address,
    pub ledger: u32,
}

// ── Contract ──────────────────────────────────────────────────────────────────
#[contract]
pub struct GuardianProxy;

#[contractimpl]
impl GuardianProxy {
    /// One-time initialisation.
    /// - `admin`         – can reset and reconfigure
    /// - `guardian`      – off-chain backend address allowed to trigger halts
    /// - `watched`       – DApp contract being protected
    /// - `threshold_bps` – drain threshold in basis points (e.g. 2000 = 20 %)
    pub fn initialize(
        env: Env,
        admin: Address,
        guardian: Address,
        watched: Address,
        threshold_bps: u32,
    ) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&ADMIN,      &admin);
        env.storage().instance().set(&GUARDIAN,   &guardian);
        env.storage().instance().set(&WATCHED,    &watched);
        env.storage().instance().set(&THRESHOLD,  &threshold_bps);
        env.storage().instance().set(&HALTED,     &false);
    }

    /// Admin records the healthy baseline balance (in stroops).
    pub fn set_baseline(env: Env, baseline: i128) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();
        env.storage().instance().set(&BASELINE, &baseline);
    }

    /// Guardian triggers a halt when a drain is detected off-chain.
    /// `current_balance` is the balance observed at trigger time.
    pub fn trigger_halt(env: Env, current_balance: i128) {
        let guardian: Address = env.storage().instance().get(&GUARDIAN).unwrap();
        guardian.require_auth();

        let baseline: i128 = env.storage().instance().get(&BASELINE).unwrap_or(0);
        let threshold_bps: u32 = env.storage().instance().get(&THRESHOLD).unwrap();

        if baseline > 0 {
            let drain_bps = ((baseline - current_balance).max(0) * 10_000 / baseline) as u32;
            if drain_bps < threshold_bps {
                panic!("drain below threshold");
            }
            env.events().publish((EVT_DRAIN,), drain_bps);
        }

        env.storage().instance().set(&HALTED, &true);
        env.events().publish(
            (EVT_HALT,),
            HaltStatus {
                halted: true,
                triggered_by: guardian,
                ledger: env.ledger().sequence(),
            },
        );
    }

    /// Admin resets the circuit breaker after the incident is resolved.
    pub fn reset(env: Env) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();
        env.storage().instance().set(&HALTED, &false);
        env.events().publish((EVT_RESET,), env.ledger().sequence());
    }

    /// Admin rotates the guardian keypair.
    pub fn set_guardian(env: Env, new_guardian: Address) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();
        env.storage().instance().set(&GUARDIAN, &new_guardian);
    }

    /// Admin updates the drain threshold.
    pub fn set_threshold(env: Env, threshold_bps: u32) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();
        env.storage().instance().set(&THRESHOLD, &threshold_bps);
    }

    // ── Views ─────────────────────────────────────────────────────────────────
    pub fn is_halted(env: Env)      -> bool    { env.storage().instance().get(&HALTED).unwrap_or(false) }
    pub fn get_threshold(env: Env)  -> u32     { env.storage().instance().get(&THRESHOLD).unwrap_or(0) }
    pub fn get_watched(env: Env)    -> Address { env.storage().instance().get(&WATCHED).unwrap() }
    pub fn get_guardian(env: Env)   -> Address { env.storage().instance().get(&GUARDIAN).unwrap() }
    pub fn get_baseline(env: Env)   -> i128    { env.storage().instance().get(&BASELINE).unwrap_or(0) }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    fn setup() -> (Env, GuardianProxyClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register_contract(None, GuardianProxy);
        let client = GuardianProxyClient::new(&env, &id);
        let admin    = Address::generate(&env);
        let guardian = Address::generate(&env);
        let watched  = Address::generate(&env);
        client.initialize(&admin, &guardian, &watched, &2000u32);
        client.set_baseline(&1_000_000i128);
        (env, client)
    }

    #[test]
    fn halt_on_drain() {
        let (_, c) = setup();
        c.trigger_halt(&700_000i128); // 30% drain
        assert!(c.is_halted());
    }

    #[test]
    #[should_panic(expected = "drain below threshold")]
    fn no_halt_below_threshold() {
        let (_, c) = setup();
        c.trigger_halt(&950_000i128); // 5% drain
    }

    #[test]
    fn reset_clears_halt() {
        let (_, c) = setup();
        c.trigger_halt(&700_000i128);
        c.reset();
        assert!(!c.is_halted());
    }
}
