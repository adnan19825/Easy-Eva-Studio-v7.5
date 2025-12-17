
use fhe_core::setup_fhe_u16; //
use std::fs::{File, create_dir_all};
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================================================");
    println!("   🔑 KEY GENERATOR (v0.7.4 Integer Radix)");
    println!("==================================================");

    // 1. Ordner sicherstellen
    create_dir_all("keys")?;

    // 2. Schlüssel generieren (nutzt deine Core-Funktion)
    println!("⚙️  Generiere Schlüssel (kann dauern)...");
    let (client_key, server_key) = setup_fhe_u16();

    // 3. Speichern
    println!("💾 Speichere Client Key...");
    let ck_file = File::create("keys/client_key.bin")?;
    bincode::serialize_into(BufWriter::new(ck_file), &client_key)?;

    println!("💾 Speichere Server Key...");
    let sk_file = File::create("keys/server_key.bin")?;
    bincode::serialize_into(BufWriter::new(sk_file), &server_key)?;

    println!("✅ FERTIG! Neue Schlüssel liegen in 'keys/'.");
    Ok(())
}

