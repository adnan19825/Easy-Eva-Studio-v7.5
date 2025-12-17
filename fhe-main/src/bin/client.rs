
use fhe_core::EncryptedPayload;
use tfhe::integer::RadixClientKey;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================================================");
    println!("   📱 EASY-EVA v10.3 FINAL CLIENT (Integer)       ");
    println!("==================================================");

    // 1. INPUT
    let input_value: u64 = 1; 
    println!("🔒 User Input: {}", input_value);

    // 2. KEYS LADEN
    let ck_file = File::open("keys/client_key.bin")
        .expect("❌ Key-Datei fehlt! (keys/client_key.bin)");
    
    let client_key: RadixClientKey = bincode::deserialize_from(BufReader::new(ck_file))?;

    // 3. VERSCHLÜSSELN
    println!("🎲 Verschlüssele...");
    let encrypted_data = client_key.encrypt(input_value);
    let payload = EncryptedPayload { data: encrypted_data };
    let bytes = bincode::serialize(&payload)?;

    // 4. SENDEN
    println!("📡 Sende an Server...");
    let start_net = Instant::now();
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(&bytes).await?;
    
    // WICHTIG: Verbindung schließen, damit Server aufhört zu lesen
    stream.shutdown().await?; 

    // 5. EMPFANGEN
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    if buffer.is_empty() {
        panic!("❌ FEHLER: Keine Daten vom Server empfangen!");
    }

    // 6. ENTSCHLÜSSELN
    let response: EncryptedPayload = bincode::deserialize(&buffer)?;
    
    // 👇 HIER WAR DER FEHLER! DIESE ZEILE HAT GEFEHLT:
    let decrypted_result: u64 = client_key.decrypt(&response.data);

    let duration = start_net.elapsed().as_micros() as f64 / 1000.0;

    println!("--------------------------------------------------");
    println!("✨ ERGEBNIS: {}", decrypted_result);
    println!("--------------------------------------------------");

    if decrypted_result == 3 {
        println!("✅ TEST BESTANDEN (1 + 2 = 3). SYSTEM LÄUFT!");
    } else {
        println!("❌ FEHLER: Ergebnis {} ist falsch.", decrypted_result);
    }

    Ok(())
}

