mod rpc;
mod scraper;
mod halt;
mod cap0077;

use dotenv::dotenv;
use std::env;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let watched   = env::var("WATCHED_CONTRACT").expect("WATCHED_CONTRACT required");
    let threshold = env::var("THRESHOLD_BPS").unwrap_or_else(|_| "2000".into())
        .parse::<u64>().expect("THRESHOLD_BPS must be a number");
    let poll_ms   = env::var("POLL_INTERVAL_MS").unwrap_or_else(|_| "5000".into())
        .parse::<u64>().expect("POLL_INTERVAL_MS must be a number");

    info!("HaltEngine backend starting — watching {watched}");

    let mut baseline: Option<u64> = None;

    loop {
        match scraper::fetch_balance(&watched).await {
            Ok(balance) => {
                let base = *baseline.get_or_insert(balance);
                let drain_bps = if balance < base {
                    (base - balance) * 10_000 / base
                } else {
                    0
                };

                if drain_bps >= threshold {
                    warn!("DRAIN DETECTED: {drain_bps} bps — triggering halt");
                    if let Err(e) = halt::trigger(&balance).await {
                        error!("halt tx failed: {e}");
                    }
                    let params = cap0077::draft(drain_bps, balance, base);
                    cap0077::broadcast(&params).await;
                    // Back off — wait for admin reset
                    sleep(Duration::from_secs(60)).await;
                    baseline = None; // re-baseline after reset
                } else {
                    info!("OK — balance: {balance}, drain: {drain_bps} bps");
                }
            }
            Err(e) => error!("scraper error: {e}"),
        }
        sleep(Duration::from_millis(poll_ms)).await;
    }
}
