mod domain;
mod infrastructure;
mod usecase;

use dotenvy::dotenv;
use infrastructure::clickhouse::ClickHouseRepository;
use infrastructure::mcp::McpServer;
use usecase::analyzer::ArbitrageAnalyzer;
use usecase::collector::DataCollector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Loading environment variables
    dotenv().ok();

    // Infrastructure initialization (ClickHouse)
    // Assuming ClickHouseRepository implements Clone
    let repo = ClickHouseRepository::new()?;

    // Printing logs to stderr to avoid cluttering stdout (channel for MCP)
    eprintln!("🦀 Initializing Arbitrage Intel (Rust 2024)...");

    if let Err(e) = repo.init_tables().await {
        eprintln!("❌ ClickHouse Init Error: {}", e);
        return Err(e);
    }

    // Loading data (Seeding)
    // We pass a clone of the repository to the collector
    let collector = DataCollector::new(repo.clone());
    eprintln!("🚀 Seeding mock data to ClickHouse...");

    if let Err(e) = collector.seed_mock_data(100).await {
        eprintln!("⚠️ Seeding failed (maybe data already exists): {}", e);
    } else {
        eprintln!("✅ Successfully seeded 100 entries.");
    }

    // Initialization of business logic and MCP server
    let analyzer = ArbitrageAnalyzer::new(repo);
    let mcp_server = McpServer::new(analyzer);

    eprintln!("📡 MCP Server is listening on stdin...");

    // Starting the MCP server
    // This method will block the thread and wait for commands from the AI (Claude/Gemini)
    if let Err(e) = mcp_server.run().await {
        eprintln!("❌ MCP Server runtime error: {}", e);
        return Err(e);
    }

    Ok(())
}
