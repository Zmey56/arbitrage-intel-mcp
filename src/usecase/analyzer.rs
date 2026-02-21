use crate::domain::models::ArbitrageOpportunity;
use crate::infrastructure::clickhouse::ClickHouseRepository;
use anyhow::Result;

// src/usecase/analyzer.rs

#[derive(Serialize)]
pub struct AnalysisResult {
    pub spread_pct: f64,
    pub volume_usd: f64,
    pub liquidity_depth: f64, // New field for context
    pub risk_score: u8,       // Score from 0 to 100
}

impl ArbitrageAnalyzer {
    pub async fn analyze_opportunity(&self, opp: ArbitrageOpportunity) -> AnalysisResult {
        //The logic of calculating risk based on volume and spread
        let risk = if opp.volume_usd < 500.0 { 80 } else { 10 };
        AnalysisResult {
            spread_pct: opp.spread_pct,
            volume_usd: opp.volume_usd,
            liquidity_depth: opp.volume_usd * 0.8, // simplified model
            risk_score: risk,
        }
    }
}
