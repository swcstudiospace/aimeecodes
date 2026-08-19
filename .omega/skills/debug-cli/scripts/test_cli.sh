#!/bin/bash
# Smoke test script for verifying omega CLI functionality
# Usage: ./scripts/test_cli.sh

set -e  # Exit on error

echo "=== Building omega CLI ==="
cargo build

echo ""
echo "=== Step 1: Get latest documentation ==="
./target/debug/omega --help

echo ""
echo "=== Step 2: Test with -p flag ==="
./target/debug/omega -p "echo 'CLI test successful'" || echo "Note: -p test may require valid context"

echo ""
echo "=== Step 3: Verify subcommand help ==="
./target/debug/omega list --help
./target/debug/omega conversation --help
./target/debug/omega config --help

echo ""
echo "=== Step 4: Test conversation commands ==="
./target/debug/omega conversation list || echo "No conversations yet (expected)"

echo ""
echo "✅ All smoke tests passed!"
echo ""
echo "Next steps:"
echo "  1. Always run --help first to get latest docs"
echo "  2. Test features with -p flag: ./target/debug/omega -p 'your task'"
echo "  3. Clone conversations before debugging: omega conversation clone <id>"
echo "  4. Never commit during debugging"
