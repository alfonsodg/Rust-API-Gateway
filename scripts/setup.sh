#!/bin/bash

# Development setup script for Rust API Gateway

set -e

echo "🚀 Setting up Rust API Gateway development environment..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust found: $(rustc --version)"

# Install required tools
echo "📦 Installing development tools..."
cargo install cargo-watch cargo-audit

# Build the project
echo "🔨 Building project..."
cargo build

# Run tests
echo "🧪 Running tests..."
cargo test

# Run benchmarks
echo "📊 Running benchmarks..."
cargo bench

echo "✅ Development environment ready!"
echo ""
echo "🎯 Quick commands:"
echo "  cargo run                    # Run the gateway"
echo "  cargo test                   # Run tests"
echo "  cargo bench                  # Run benchmarks"
echo "  cargo watch -x run          # Auto-reload on changes"
echo "  docker-compose up            # Run with Docker"
echo ""
echo "📖 Check README.md for detailed usage instructions."
