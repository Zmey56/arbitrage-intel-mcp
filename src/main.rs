mod domain;
mod infrastructure;
mod usecase;

use dotenvy::dotenv;
use infrastructure::clickhouse::ClickHouseRepository;
use infrastructure::mcp::McpServer;
use usecase::analyzer::ArbitrageAnalyzer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let repo = ClickHouseRepository::new()?;
    repo.init_tables().await?;

    let analyzer = ArbitrageAnalyzer::new(repo);
    let mcp_server = McpServer::new(analyzer);

    // important: The MCP server communicates via stdout,
    // therefore, all application logs must go to stderr!
    eprintln!("🦀 Arbitrage Intel MCP Server started on Rust...");

    mcp_server.run().await?;

    Ok(())
}
