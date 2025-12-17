use fhe_main::fhe_engine::FheEngine;
use tfhe::integer::ServerKey;
use fhe_core::EncryptedPayload;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================================================");
    println!("   🚀 EASY-EVA v10.2 STABLE SERVER                ");
    println!("==================================================");

    // 1. ENGINE LADEN
    let engine = FheEngine::new();
    let shared_engine = Arc::new(engine);

    // 2. SERVER STARTEN
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("✅ SERVER ONLINE (Port 8080). Warte auf Daten...");
    println!("--------------------------------------------------");

    loop {
        let (socket, _) = listener.accept().await?;
        let engine_clone = shared_engine.clone();
        tokio::spawn(async move {
            handle_client(socket, engine_clone).await.ok();
        });
    }
}

async fn handle_client(mut socket: TcpStream, engine: Arc<FheEngine>) -> std::io::Result<()> {
    // WICHTIG: read_to_end statt read
    let mut buffer = Vec::new();
    socket.read_to_end(&mut buffer).await?;
    
    if buffer.is_empty() { return Ok(()); }

    // Ab hier deserialize (Buffer ist jetzt ein Vec, kein Slice mehr nötig)
let mut payload: EncryptedPayload = match bincode::deserialize(&buffer) {
        Ok(p) => p,
        Err(e) => {
            println!("❌ Fehler beim Entpacken: {}", e);
            return Ok(());
        }
    };
    
    // ... Rest bleibt gleich (ab println!("⚙️ Rechne..."))
    
    // FIX 1: 'mut' hinzufügen -> Wir müssen die Payload verändern dürfen
    let mut payload: EncryptedPayload = match bincode::deserialize(&buffer) {
        Ok(p) => p,
        Err(_) => return Ok(()),
    };

    println!("⚙️  Rechne: Input + 2 ...");
    let sk = &engine.key_store.server_key;
    
    // FIX 2: '&mut payload.data' -> TFHE braucht Schreibzugriff für Smart Operations
    // Wir nutzen 'scalar_add_parallelized' für Integer (schneller & parallel)
// Wichtig: &payload.data (ohne mut), da wir einen NEUEN Ciphertext erzeugen
let result_ct = sk.scalar_add_parallelized(&payload.data, 2);

    let response = EncryptedPayload { data: result_ct };
    let response_bytes = bincode::serialize(&response).unwrap();
    socket.write_all(&response_bytes).await?;
    Ok(())
}
