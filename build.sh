#!/bin/bash

# 🎛️ ntrck Build Script
# Builds the WASM module and sets up the project

set -e

echo "🎛️ Building ntrck..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Build WASM module
echo "🔨 Building WASM module..."
cd wasm
wasm-pack build --target web
cd ..

# Create pkg symlink for development
if [ ! -L "static/pkg" ]; then
    echo "🔗 Creating pkg symlink..."
    ln -sf ../wasm/pkg static/pkg
fi

echo "✅ Build complete!"
echo ""
echo "🚀 To run the project:"
echo "   Frontend only: python3 -m http.server 8080"
echo "   With backend:  cargo run --bin server"
echo ""
echo "📁 Project structure:"
echo "   wasm/     - Rust/WASM audio engine"
echo "   static/   - Frontend (HTML/CSS/JS)"
echo "   server/   - Backend API (optional)"
echo "   samples/  - Audio samples and presets" 