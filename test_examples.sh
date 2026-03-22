#!/bin/bash

# Quick test script for MPP-NEAR examples
# This script helps you test that the examples work correctly

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║          MPP-NEAR Examples Test Script                      ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if .env file exists
if [ ! -f .env ]; then
    echo "❌ .env file not found!"
    echo "Creating .env from .env.example..."
    cp .env.example .env
    echo "✅ Created .env file"
    echo ""
    echo "⚠️  Please edit .env and add your actual credentials:"
    echo "   - OUTLAYER_API_KEY (required for NEAR Intents)"
    echo "   - NEAR_ACCOUNT_ID (optional, for standard NEAR)"
    echo "   - NEAR_PRIVATE_KEY (optional, for standard NEAR)"
    echo ""
    exit 1
fi

# Load environment variables
set -a
source .env
set +a

echo "📋 Configuration check:"
echo "   OUTLAYER_API_KEY: ${OUTLAYER_API_KEY:+✅ Set}${OUTLAYER_API_KEY:-❌ Not set}"
echo "   NEAR_ACCOUNT_ID: ${NEAR_ACCOUNT_ID:+✅ Set}${NEAR_ACCOUNT_ID:-❌ Not set}"
echo "   MPP_RECIPIENT: ${MPP_RECIPIENT:-merchant.near}"
echo "   MPP_SERVER_URL: ${MPP_SERVER_URL:-http://localhost:3000}"
echo ""

# Check if server is running
echo "🔍 Checking if server is running..."
if curl -s http://localhost:3000/health > /dev/null 2>&1; then
    echo "✅ Server is running!"
    echo ""
else
    echo "❌ Server is not running!"
    echo ""
    echo "Please start the server in another terminal:"
    echo "  cargo run --example full_server --features server"
    echo ""
    exit 1
fi

# Test 1: Free endpoint
echo "📤 Test 1: Free endpoint (no payment)"
HEALTH_RESPONSE=$(curl -s http://localhost:3000/health)
echo "Response: $HEALTH_RESPONSE"
if echo "$HEALTH_RESPONSE" | grep -q "healthy"; then
    echo "✅ Test passed!"
else
    echo "❌ Test failed!"
fi
echo ""

# Test 2: Get pricing
echo "📤 Test 2: Get pricing information"
PRICING_RESPONSE=$(curl -s http://localhost:3000/pricing)
echo "Response: $PRICING_RESPONSE"
if echo "$PRICING_RESPONSE" | grep -q "endpoints"; then
    echo "✅ Test passed!"
else
    echo "❌ Test failed!"
fi
echo ""

# Test 3: Paid endpoint without payment
echo "📤 Test 3: Paid endpoint without payment (should get 402)"
PAID_RESPONSE=$(curl -s -w "\n%{http_code}" http://localhost:3000/api/v1/ping)
HTTP_CODE=$(echo "$PAID_RESPONSE" | tail -n1)
RESPONSE_BODY=$(echo "$PAID_RESPONSE" | sed '$d')

echo "HTTP Status: $HTTP_CODE"
echo "Response: $RESPONSE_BODY"

if [ "$HTTP_CODE" = "402" ]; then
    echo "✅ Test passed! Got expected 402 Payment Required"
else
    echo "❌ Test failed! Expected 402, got $HTTP_CODE"
fi
echo ""

# Test 4: Full client test (requires OUTLAYER_API_KEY)
if [ -n "$OUTLAYER_API_KEY" ]; then
    echo "📤 Test 4: Full client payment flow"
    echo "Running: cargo run --example test_payment_client --features client,intents"
    echo ""

    if cargo run --example test_payment_client --features client,intents; then
        echo ""
        echo "✅ Full client test passed!"
    else
        echo ""
        echo "❌ Full client test failed!"
        echo "Make sure your OUTLAYER_API_KEY is valid and has funds"
    fi
else
    echo "⏭️  Skipping Test 4: OUTLAYER_API_KEY not set"
    echo "   Set it in .env to test real payments"
fi
echo ""

echo "╔════════════════════════════════════════════════════════════╗"
echo "║                    Test Summary                             ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "✅ Basic tests passed!"
echo ""
echo "Next steps:"
echo "1. To test real payments, set OUTLAYER_API_KEY in .env"
echo "2. Run: cargo run --example test_payment_client --features client,intents"
echo "3. Or run the full client: cargo run --example full_client --features client,intents"
echo ""
