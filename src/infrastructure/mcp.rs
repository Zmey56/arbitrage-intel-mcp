use crate::usecase::analyzer::ArbitrageAnalyzer;
use anyhow::Result;
use serde_json::{Value, json};
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub struct McpServer {
    analyzer: ArbitrageAnalyzer,
}

impl McpServer {
    pub fn new(analyzer: ArbitrageAnalyzer) -> Self {
        Self { analyzer }
    }

    pub async fn run(&self) -> Result<()> {
        let mut lines = BufReader::new(io::stdin()).lines();

        while let Some(line) = lines.next_line().await? {
            let request: Value = serde_json::from_str(&line)?;

            // Processing of JSON-RPC MCP methods
            if request["method"] == "listTools" {
                self.send_response(request["id"].clone(), self.list_tools())
                    .await?;
            } else if request["method"] == "callTool" {
                let tool_name = request["params"]["name"].as_str().unwrap_or("");
                let response = self.handle_tool_call(tool_name).await?;
                self.send_response(request["id"].clone(), response).await?;
            }
        }
        Ok(())
    }

    fn list_tools(&self) -> Value {
        json!({
            "tools": [
                {
                    "name": "get_top_spreads",
                    "description": "Returns recent high-profit arbitrage opportunities from ClickHouse",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "limit": { "type": "number", "description": "Number of opportunities to return" }
                        }
                    }
                }
            ]
        })
    }

    async fn handle_tool_call(&self, name: &str) -> Result<Value> {
        match name {
            "get_top_spreads" => {
                let data = self.analyzer.get_analytics(10).await?;
                Ok(json!({ "content": [{ "type": "text", "text": format!("{:?}", data) }] }))
            }
            _ => Ok(
                json!({ "isError": true, "content": [{ "type": "text", "text": "Unknown tool" }] }),
            ),
        }
    }

    async fn send_response(&self, id: Value, result: Value) -> Result<()> {
        let response = json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result
        });
        println!("{}", serde_json::to_string(&response)?);
        Ok(())
    }
}
