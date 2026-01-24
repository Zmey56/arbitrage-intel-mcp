import mcp

// RegisterArbitrageTools defines the tools available to the AI model.
func (s *MCPServer) RegisterArbitrageTools() {
    s.server.RegisterTool("get_top_spreads", "Fetch current arbitrage opportunities by spread percentage",
        func(ctx context.Context, args struct {
            MinSpread float64 `json:"min_spread"`
            Network   string  `json:"network"`
        }) (interface{}, error) {
            // Here we call the UseCase that queries ClickHouse
            opportunities, err := s.analyzer.FindHighSpreads(ctx, args.MinSpread, args.Network)
            if err != nil {
                return nil, fmt.Errorf("failed to analyze spreads: %w", err)
            }
            return opportunities, nil
        },
    )
}
