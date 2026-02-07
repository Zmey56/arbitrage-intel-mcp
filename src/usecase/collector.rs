use crate::domain::models::ArbitrageOpportunity;
use crate::infrastructure::clickhouse::ClickHouseRepository;
use anyhow::Result;
use chrono::Utc;

//A "collector" that will simulate receiving data (or in the future collect it through exchange WebSockets)
//  and send it to the repository.

pub struct DataCollector {
    repo: ClickHouseRepository,
}

impl DataCollector {
    pub fn new(repo: ClickHouseRepository) -> Self {
        Self { repo }
    }

    pub async fn seed_mock_data(&self, count: usize) -> Result<()> {
        let pairs = vec!["BTC/USDT", "ETH/USDT", "SOL/USDC", "DOT/USDT"];
        let chains = vec!["Ethereum", "Solana", "Binance", "Polygon"];

        let mut bundle = Vec::with_capacity(count);

        for i in 0..count {
            bundle.push(ArbitrageOpportunity {
                exchange_pair: pairs[i % pairs.len()].to_string(),
                spread_pct: 0.5 + (i as f64 * 0.01),
                volume_usd: 1000.0 + (i as f64 * 50.0),
                detected_at: Utc::now().timestamp_millis() - (i as i64 * 1000), // каждую секунду назад
                chain: chains[i % chains.len()].to_string(),
            });
        }

        // Вставляем всю пачку разом
        self.repo.insert_batch(bundle).await?;
        Ok(())
    }
}
