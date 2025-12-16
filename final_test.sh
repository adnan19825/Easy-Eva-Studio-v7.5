#!/bin/bash
cd ~/fhe-workspace
echo "=== FINALER TEST ==="

# 1. Kompilieren
echo "1. Kompiliere fhe-core..."
cargo build --release -p fhe-core 2>&1 | grep -E "(Finished|error:|warning:)" || echo "Kompilierung läuft..."

# 2. Test erstellen und ausführen
echo "2. Erstelle und führe Test aus..."
cat > /tmp/test_fhe.rs << 'RUST'
use fhe_core::*;
fn main() {
    let (ck, sk) = setup_fhe();
    
    // Test Summe
    let vals = vec![ck.encrypt(1), ck.encrypt(2), ck.encrypt(3), ck.encrypt(4)];
    let sum = encrypted_sum(&sk, &vals);
    let sum_result = ck.decrypt(&sum);
    println!("📊 Summe [1,2,3,4]: {} {}", sum_result, if sum_result == 10 { "✅" } else { "❌" });
    
    // Test Vergleich
    let a = ck.encrypt(5);
    let b = ck.encrypt(3);
    let gt = encrypted_greater_than(&sk, &a, &b);
    let gt_result = ck.decrypt(&gt);
    println!("📊 5 > 3: {} {}", gt_result, if gt_result == 1 { "✅" } else { "❌" });
    
    if sum_result == 10 && gt_result == 1 {
        println!("\n🎉 PHASE 1 ERFOLGREICH ABGESCHLOSSEN!");
    }
}
RUST

rustc --edition=2021 -L ./target/release/deps --extern fhe_core=./target/release/libfhe_core.rlib /tmp/test_fhe.rs 2>/dev/null
if [ -f "/tmp/test_fhe" ]; then
    /tmp/test_fhe
    rm -f /tmp/test_fhe
fi
rm -f /tmp/test_fhe.rs 2>/dev/null
