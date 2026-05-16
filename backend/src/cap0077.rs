use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::warn;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cap0077Params {
    pub version:                &'static str,
    pub target_contract:        String,
    pub drain_bps:              u64,
    pub current_balance:        u64,
    pub baseline_balance:       u64,
    pub freeze_type:            &'static str,
    pub quorum_threshold:       f64,
    pub freeze_duration_ledgers: u32,
    pub drafted_at:             String,
}

pub fn draft(drain_bps: u64, current_balance: u64, baseline: u64) -> Cap0077Params {
    // Scale freeze duration with severity (1 ledger ≈ 5 s)
    let freeze_duration_ledgers = match drain_bps {
        d if d >= 7000 => 4320, // >70% → ~6 hr
        d if d >= 4000 => 720,  // >40% → ~1 hr
        _              => 120,  // >20% → ~10 min
    };

    Cap0077Params {
        version: "cap-0077-v1",
        target_contract: env::var("WATCHED_CONTRACT").unwrap_or_default(),
        drain_bps,
        current_balance,
        baseline_balance: baseline,
        freeze_type: "contract",
        quorum_threshold: 0.67,
        freeze_duration_ledgers,
        drafted_at: chrono_now(),
    }
}

pub async fn broadcast(params: &Cap0077Params) {
    let endpoints: Vec<String> = env::var("VALIDATOR_ENDPOINTS")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    if endpoints.is_empty() {
        warn!("[CAP-0077] No validator endpoints configured — skipping broadcast");
        return;
    }

    let client = Client::new();
    for url in &endpoints {
        match client.post(format!("{url}/cap0077/freeze-request"))
            .json(params)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(r)  => tracing::info!("[CAP-0077] {url} → {}", r.status()),
            Err(e) => warn!("[CAP-0077] {url} failed: {e}"),
        }
    }
}

fn chrono_now() -> String {
    // No chrono dep — use a simple placeholder; replace with chrono in prod
    "2026-01-01T00:00:00Z".to_string()
}
