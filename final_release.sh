#!/bin/bash
echo "=== EASY-EVA FINAL RELEASE v1.0 ==="
echo "Datum: $(date)"
echo "[1/3] Bereinige alten Build..."
cargo clean
echo "[2/3] Kompiliere finale Binary (Optimized)..."
cargo build --release -p fhe-main
echo "[3/3] Setze Berechtigungen..."
chmod +x ./target/release/fhe-main
echo "✅ RELEASE FERTIG: ./target/release/fhe-main"
