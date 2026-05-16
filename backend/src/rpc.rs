use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct RpcRequest<'a> {
    jsonrpc: &'a str,
    id:      u32,
    method:  &'a str,
    params:  serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub struct RpcResponse<T> {
    pub result: Option<T>,
    pub error:  Option<serde_json::Value>,
}

pub async fn call<T: for<'de> Deserialize<'de>>(
    method: &str,
    params: serde_json::Value,
) -> anyhow::Result<T> {
    let url = env::var("SOROBAN_RPC_URL")
        .unwrap_or_else(|_| "https://soroban-testnet.stellar.org".into());

    let body = RpcRequest { jsonrpc: "2.0", id: 1, method, params };
    let resp: RpcResponse<T> = Client::new()
        .post(&url)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    resp.result.ok_or_else(|| {
        anyhow::anyhow!("RPC error: {:?}", resp.error)
    })
}
