#!/bin/bash
# Test MPP-NEAR API Server
# Usage: ./test-api.sh [port]

PORT=${1:-3456}
BASE="http://localhost:$PORT"

echo "Testing MPP-NEAR API at $BASE"
echo "================================"

# Health check
echo -e "\n[1] Health Check"
curl -s "$BASE/health" | jq .

# Config
echo -e "\n[2] Config"
curl -s "$BASE/config" | head -20

# Balance
echo -e "\n[3] Balance"
curl -s "$BASE/balance" | jq .

# Tokens (filter ZEC)
echo -e "\n[4] Tokens (ZEC)"
curl -s "$BASE/tokens?search=ZEC" | jq '.tokens | length'

# Handoff URL
echo -e "\n[5] Handoff URL"
curl -s "$BASE/handoff" | jq .

# Fund link
echo -e "\n[6] Fund Link (1 NEAR)"
curl -s "$BASE/fund-link?amount=1" | jq .

echo -e "\n================================"
echo "All tests passed!"
