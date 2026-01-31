use crate::domain::models::ArbitrageOpportunity;
use crate::infrastructure::clickhouse::ClickHouseRepository;
use anyhow::Result;

pub struct ArbitrageAnalyzer {
    repo: ClickHouseRepository,
}

impl ArbitrageAnalyzer {
    pub fn new(repo: ClickHouseRepository) -> Self {
        Self { repo }
    }

    pub async fn get_analytics(&self, limit: usize) -> Result<Vec<ArbitrageOpportunity>> {
        // Here we could add complex filtering logic.,
        // but for now, we're just asking the repository to return the latest data.
        self.repo.fetch_last_opportunities(limit).await
    }
}
