#!/bin/bash
cd ~/fhe-workspace

cat > debug.rs << 'RUSTCODE'
use fhe_core::*;

fn main() {
    let (ck, sk) = setup_fhe();
    
    println!("=== DEBUGAUSGABE ===");
    
    let vals = [1u64, 2, 3, 4];
    let mut enc_vals = Vec::new();
    
    for &v in &vals {
        let enc = ck.encrypt(v);
        let dec = ck.decrypt(&enc);
        println!("Plain: {} → Encrypted → Decrypted: {}", v, dec);
        enc_vals.push(enc);
    }
    
    println!("\n=== PAARWEISE ADDITION ===");
    
    // 1 + 2
    let sum1 = sk.unchecked_add(&enc_vals[0], &enc_vals[1]);
    println!("1 + 2 = {}", ck.decrypt(&sum1));
    
    // (1+2) + 3  
    let sum2 = sk.unchecked_add(&sum1, &enc_vals[2]);
    println!("(1+2) + 3 = {}", ck.decrypt(&sum2));
    
    // ((1+2)+3) + 4
    let sum3 = sk.unchecked_add(&sum2, &enc_vals[3]);
    println!("((1+2)+3) + 4 = {}", ck.decrypt(&sum3));
    
    println!("\n=== ENCRYPTED_SUM FUNKTION ===");
    let total = encrypted_sum(&sk, &enc_vals);
    println!("encrypted_sum Ergebnis: {}", ck.decrypt(&total));
}
RUSTCODE

rustc --edition=2021 -L ./target/release/deps --extern fhe_core=./target/release/libfhe_core.rlib debug.rs
./debug
rm -f debug debug.rs
