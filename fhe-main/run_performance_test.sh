#!/bin/bash
echo "🏃 FHE PERFORMANCE TEST"
echo "======================"

cd ~/fhe-workspace/fhe-main

if [ ! -f "target/release/fhe-main" ]; then
    echo "❌ Binary nicht gefunden. Bitte zuerst bauen: cargo build --release"
    exit 1
fi

echo ""
echo "1️⃣  TEST SINGLE OPERATIONS:"
echo "--------------------------"
echo "Adding single entries..."
for i in {1..3}; do
    echo "  Entry $i..."
    ./target/release/fhe-main add $i $((i * 100)) --benchmark 2>&1 | grep -E "(Added|PERFORMANCE)"
done

echo ""
echo "2️⃣  TEST BATCH OPERATIONS:"
echo "-------------------------"
echo "Adding batch of 5 entries..."
./target/release/fhe-main add-batch "10:1000,20:2000,30:3000,40:4000,50:5000" --benchmark

echo ""
echo "3️⃣  RUN BENCHMARK:"
echo "-----------------"
./target/release/fhe-main benchmark --iterations 20

echo ""
echo "4️⃣  CHECK STATS:"
echo "---------------"
./target/release/fhe-main stats

echo ""
echo "🎉 PERFORMANCE TEST COMPLETE!"
echo ""
echo "📋 EMPFEHLUNGEN BASED ON RESULTS:"
echo "1. Für wenige Einträge (<10): Einzel-Operationen"
echo "2. Für viele Einträge (>10): Batch-Operationen"
echo "3. Performance-Monitoring immer mit --benchmark"
