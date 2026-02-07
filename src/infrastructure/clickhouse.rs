use crate::domain::models::ArbitrageOpportunity;
use anyhow::{Context, Result};
use clickhouse::Client;

#[derive(Clone)]
pub struct ClickHouseRepository {
    pub client: clickhouse::Client,
}

impl ClickHouseRepository {
    pub fn new() -> Result<Self> {
        let host = std::env::var("CLICKHOUSE_HOST").context("CLICKHOUSE_HOST not set")?;
        let user = std::env::var("CLICKHOUSE_USER").unwrap_or_else(|_| "default".into());
        let pass = std::env::var("CLICKHOUSE_PASSWORD").unwrap_or_default();
        let db = std::env::var("CLICKHOUSE_DB").unwrap_or_else(|_| "arbitrage_db".into());

        let client = Client::default()
            .with_url(format!("http://{}:8123", host))
            .with_user(user)
            .with_password(pass)
            .with_database(db);

        Ok(Self { client })
    }

    pub async fn init_tables(&self) -> Result<()> {
        let sql = r#"
            CREATE TABLE IF NOT EXISTS arbitrage_opportunities (
                exchange_pair String,
                spread_pct Float64,
                volume_usd Float64,
                detected_at DateTime64(3, 'UTC'),
                chain String
            ) ENGINE = MergeTree()
            ORDER BY (detected_at, exchange_pair)
        "#;
        self.client.query(sql).execute().await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn insert_opportunity(&self, opp: &ArbitrageOpportunity) -> Result<()> {
        let mut insert = self
            .client
            .insert::<ArbitrageOpportunity>("arbitrage_opportunities")
            .await?;

        insert.write(opp).await?;
        insert.end().await?;

        Ok(())
    }

    pub async fn fetch_last_opportunities(
        &self,
        limit: usize,
    ) -> Result<Vec<ArbitrageOpportunity>> {
        let mut cursor = self
            .client
            .query("SELECT * FROM arbitrage_opportunities ORDER BY detected_at DESC LIMIT ?")
            .bind(limit as u32)
            .fetch::<ArbitrageOpportunity>()?;

        let mut result = Vec::new();
        while let Some(row) = cursor.next().await? {
            result.push(row);
        }
        Ok(result)
    }

    pub async fn insert_batch(&self, opportunities: Vec<ArbitrageOpportunity>) -> Result<()> {
        let mut insert = self
            .client
            .insert::<ArbitrageOpportunity>("arbitrage_opportunities")
            .await?;

        for opp in opportunities {
            insert.write(&opp).await?;
        }

        insert.end().await?;
        Ok(())
    }
}
