use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct AccountEntry {
    balance: String,
}

#[derive(Deserialize, Debug)]
struct LedgerEntries {
    entries: Option<Vec<LedgerEntry>>,
}

#[derive(Deserialize, Debug)]
struct LedgerEntry {
    xdr: String,
}

/// Fetch the XLM balance of `contract_id` via Horizon (stroops).
pub async fn fetch_balance(contract_id: &str) -> anyhow::Result<u64> {
    let horizon = env::var("HORIZON_URL")
        .unwrap_or_else(|_| "https://horizon-testnet.stellar.org".into());

    let url = format!("{horizon}/accounts/{contract_id}");
    let resp: serde_json::Value = reqwest::get(&url).await?.json().await?;

    let balance_str = resp["balances"]
        .as_array()
        .and_then(|bs| bs.iter().find(|b| b["asset_type"] == "native"))
        .and_then(|b| b["balance"].as_str())
        .unwrap_or("0");

    // Convert XLM string to stroops (1 XLM = 10_000_000 stroops)
    let xlm: f64 = balance_str.parse().unwrap_or(0.0);
    Ok((xlm * 10_000_000.0) as u64)
}
