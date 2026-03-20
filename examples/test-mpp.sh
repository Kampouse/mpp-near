#!/bin/bash
# Test MPP-NEAR Server
# Demonstrates full MPP flow: 402 → Pay → Retry → 200

PORT=${1:-3457}
BASE="http://localhost:$PORT"

echo "========================================"
echo "MPP-NEAR Server Test"
echo "========================================"
echo ""

# 1. Health check
echo "[1] Health Check"
curl -s "$BASE/health" | jq .
echo ""

# 2. Discovery endpoint
echo "[2] MPP Discovery (/.well-known/payment)"
curl -s "$BASE/.well-known/payment" | jq .
echo ""

# 3. Pricing
echo "[3] Pricing"
curl -s "$BASE/pricing" | jq .
echo ""

# 4. Request protected endpoint (should get 402)
echo "[4] Request Protected Endpoint (expect 402)"
RESPONSE=$(curl -s -i "$BASE/api/v1/search")
echo "$RESPONSE" | head -30
echo ""

# Extract challenge from response
CHALLENGE_ID=$(echo "$RESPONSE" | grep -o 'challenge_id":"[^"]*' | cut -d'"' -f3)
AMOUNT=$(echo "$RESPONSE" | grep -o '"amount":"[^"]*' | head -1 | cut -d'"' -f4)
TOKEN=$(echo "$RESPONSE" | grep -o '"token":"[^"]*' | head -1 | cut -d'"' -f4)
RECIPIENT=$(echo "$RESPONSE" | grep -o '"recipient":"[^"]*' | cut -d'"' -f4)

echo "Challenge ID: $CHALLENGE_ID"
echo "Amount: $AMOUNT $TOKEN"
echo "Recipient: $RECIPIENT"
echo ""

# 5. Retry with mock credential (for testing)
echo "[5] Retry with Mock Credential"
curl -s -H "Authorization: Payment challenge_id=\"$CHALLENGE_ID\", intent_hash=\"mock_hash_123\", account_id=\"test\"" \
  "$BASE/api/v1/search" | jq .
echo ""

# 6. Test another endpoint
echo "[6] Test /api/v1/generate"
RESPONSE=$(curl -s -i "$BASE/api/v1/generate")
CHALLENGE_ID=$(echo "$RESPONSE" | grep -o 'challenge_id":"[^"]*' | cut -d'"' -f3)

curl -s -H "Authorization: Payment challenge_id=\"$CHALLENGE_ID\", intent_hash=\"mock_hash_456\", account_id=\"test\"" \
  "$BASE/api/v1/generate" | jq .
echo ""

echo "========================================"
echo "Test Complete"
echo ""
echo "To test with real payments:"
echo "  node mpp-near-client.js http://localhost:$PORT/api/v1/search"
echo "========================================"
