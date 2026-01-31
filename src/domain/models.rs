use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct ArbitrageOpportunity {
    pub exchange_pair: String,
    pub spread_pct: f64,
    pub volume_usd: f64,
    pub detected_at: i64,
    pub chain: String,
}
