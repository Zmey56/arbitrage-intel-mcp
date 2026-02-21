```markdown
# Arbitrage Intelligence MCP Server (Go) 🚀

[![Go Version](https://img.shields.io/badge/Go-1.25+-00ADD8?style=flat&logo=go)](https://go.dev/)
[![Protocol](https://img.shields.io/badge/MCP-Standard-orange)](https://modelcontextprotocol.io/)
[![Database](https://img.shields.io/badge/ClickHouse-2026_Ready-yellow?logo=clickhouse)](https://clickhouse.com/)

A high-performance **Model Context Protocol (MCP)** server built in Go. This tool bridges the gap between Large Language Models (Gemini, Claude, GPT-4o) and real-time cryptocurrency arbitrage data stored in **ClickHouse**.

## 🌟 Overview

Modern LLMs struggle with "hallucinations" and lack access to granular, real-time market data. This project implements a professional-grade MCP server that allows AI agents to:
- Query millions of tick-level records using optimized SQL.
- Identify arbitrage anomalies across DEX/CEX.
- Correlate market events with historical spreads.

## 🛠 Tech Stack

- **Language:** Go 1.25+ (utilizing modern concurrency patterns)
- **Protocol:** Model Context Protocol (MCP) SDK
- **Database:** ClickHouse (via `clickhouse-go/v2`)
- **Architecture:** Clean Architecture / Hexagonal
- **Environment:** Docker-ready

## 🏗 Architecture Principles

This project demonstrates **Clean Code** and **SOLID** principles:
- **Dependency Inversion:** The core business logic is independent of the database implementation.
- **Interface Segregation:** MCP handlers only interact with specific use-case interfaces.
- **Observability:** Structured logging and contextual error handling.

## 🚀 Getting Started

### Prerequisites
- Go 1.25 or higher
- ClickHouse instance (local or Cloud)
- MCP-compatible client (Claude Desktop, Gemini Pro, or custom UI)

### Installation
1. **Clone the repository:**
   ```bash
   git clone [https://github.com/your-username/arbitrage-intel-mcp.git](https://github.com/your-username/arbitrage-intel-mcp.git)
   cd arbitrage-intel-mcp

```

2. **Configure environment:**
```bash
cp .env.example .env
# Edit .env with your ClickHouse credentials

```


3. **Build & Run:**
```bash
make build
./bin/mcp-server

```



### Connecting to AI Clients

Add the following to your MCP config file (e.g., `mcp-config.json`):

```json
{
  "mcpServers": {
    "crypto-arbitrage": {
      "command": "/path/to/arbitrage-intel-mcp/bin/mcp-server",
      "env": {
        "CLICKHOUSE_HOST": "localhost",
        "CLICKHOUSE_PORT": "9000"
      }
    }
  }
}

```

## 🔍 Available Tools for AI

| Tool | Parameters | Description |
| --- | --- | --- |
| `get_top_spreads` | `limit: int` | Returns top arbitrage opportunities sorted by profit. |
| `analyze_anomaly` | `pair: string` | Deep dive into specific pair volatility in ClickHouse. |
| `get_volume_stats` | `time_range: string` | Aggregates trading volume for risk assessment. |

## 🧪 Testing

```bash
make test

```

## 📜 License

This project is licensed under the MIT License.

---

*Developed for educational purposes as a demonstration of high-load engineering and AI integration.*

