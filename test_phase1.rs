use fhe_core::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🔬 Phase 1 Test: Erweiterte FHE-Operationen");
    let (ck, sk) = setup_fhe();
    
    // Test 1: Summe
    println!("\\n[Test 1] Encrypted Sum");
    let values = vec![1, 2, 3, 4];
    let enc_values: Vec<_> = values.iter().map(|&x| ck.encrypt(x)).collect();
    let enc_sum = encrypted_sum(&sk, &enc_values);
    let dec_sum = ck.decrypt(&enc_sum);
    println!("  Werte: {:?}", values);
    println!("  Ergebnis: {} (Erwartet: 10)", dec_sum);
    assert_eq!(dec_sum, 10, "❌ Summe fehlgeschlagen");
    println!("  ✅ Summe korrekt");
    
    // Test 2: Durchschnitt
    println!("\\n[Test 2] Encrypted Average");
    let enc_avg = encrypted_average(&sk, &enc_values);
    let dec_avg = ck.decrypt(&enc_avg);
    println!("  Durchschnitt: {} (Erwartet: 2)", dec_avg);
    assert_eq!(dec_avg, 2, "❌ Durchschnitt fehlgeschlagen");
    println!("  ✅ Durchschnitt korrekt");
    
    // Test 3: Größer-als Vergleich
    println!("\\n[Test 3] Encrypted Greater-Than");
    let a = ck.encrypt(5);
    let b = ck.encrypt(3);
    let c = ck.encrypt(3);
    let d = ck.encrypt(7);
    
    let gt1 = encrypted_greater_than(&sk, &a, &b); // 5 > 3 = 1
    let gt2 = encrypted_greater_than(&sk, &b, &c); // 3 > 3 = 0
    let gt3 = encrypted_greater_than(&sk, &b, &d); // 3 > 7 = 0
    
    println!("  5 > 3: {} (Erwartet: 1)", ck.decrypt(&gt1));
    println!("  3 > 3: {} (Erwartet: 0)", ck.decrypt(&gt2));
    println!("  3 > 7: {} (Erwartet: 0)", ck.decrypt(&gt3));
    
    assert_eq!(ck.decrypt(&gt1), 1, "❌ 5 > 3 fehlgeschlagen");
    assert_eq!(ck.decrypt(&gt2), 0, "❌ 3 > 3 fehlgeschlagen");
    assert_eq!(ck.decrypt(&gt3), 0, "❌ 3 > 7 fehlgeschlagen");
    println!("  ✅ Alle Vergleiche korrekt");
    
    // Test 4: Maximum
    println!("\\n[Test 4] Encrypted Maximum");
    let max1 = encrypted_max(&sk, &a, &b); // max(5,3) = 5
    let max2 = encrypted_max(&sk, &b, &d); // max(3,7) = 7
    
    println!("  max(5,3): {} (Erwartet: 5)", ck.decrypt(&max1));
    println!("  max(3,7): {} (Erwartet: 7)", ck.decrypt(&max2));
    
    assert_eq!(ck.decrypt(&max1), 5, "❌ max(5,3) fehlgeschlagen");
    assert_eq!(ck.decrypt(&max2), 7, "❌ max(3,7) fehlgeschlagen");
    println!("  ✅ Maximum korrekt");
    
    println!("\\n🎉 ALLE TESTS BESTANDEN! Phase 1 implementiert.");
    Ok(())
}
