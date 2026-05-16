use std::env;
use tracing::info;

/// Submit a `trigger_halt` invocation to the GuardianProxy contract.
/// In production this builds and signs a Soroban transaction; here we call
/// the Soroban RPC `simulateTransaction` + `sendTransaction` flow.
pub async fn trigger(current_balance: &u64) -> anyhow::Result<()> {
    let contract_id = env::var("GUARDIAN_PROXY_CONTRACT_ID")
        .expect("GUARDIAN_PROXY_CONTRACT_ID required");

    // Build minimal XDR invoke-contract operation via RPC
    let params = serde_json::json!({
        "transaction": build_invoke_xdr(&contract_id, *current_balance),
    });

    let result: serde_json::Value = crate::rpc::call("sendTransaction", params).await?;
    info!("trigger_halt tx: {}", result["hash"].as_str().unwrap_or("unknown"));
    Ok(())
}

/// Placeholder — in a real deployment this would use stellar-xdr to build
/// the InvokeHostFunction operation XDR and sign it with the guardian key.
fn build_invoke_xdr(contract_id: &str, balance: u64) -> String {
    format!("INVOKE:{}:trigger_halt:{}", contract_id, balance)
}
