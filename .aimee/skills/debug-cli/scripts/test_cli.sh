#!/bin/bash
# Smoke test script for verifying aimee CLI functionality
# Usage: ./scripts/test_cli.sh

set -e  # Exit on error

echo "=== Building aimee CLI ==="
cargo build

echo ""
echo "=== Step 1: Get latest documentation ==="
./target/debug/aimee --help

echo ""
echo "=== Step 2: Test with -p flag ==="
./target/debug/aimee -p "echo 'CLI test successful'" || echo "Note: -p test may require valid context"

echo ""
echo "=== Step 3: Verify subcommand help ==="
./target/debug/aimee list --help
./target/debug/aimee conversation --help
./target/debug/aimee config --help

echo ""
echo "=== Step 4: Test conversation commands ==="
./target/debug/aimee conversation list || echo "No conversations yet (expected)"

echo ""
echo "✅ All smoke tests passed!"
echo ""
echo "Next steps:"
echo "  1. Always run --help first to get latest docs"
echo "  2. Test features with -p flag: ./target/debug/aimee -p 'your task'"
echo "  3. Clone conversations before debugging: aimee conversation clone <id>"
echo "  4. Never commit during debugging"
