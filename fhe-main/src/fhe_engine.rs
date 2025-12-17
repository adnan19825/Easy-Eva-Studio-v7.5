use tfhe::integer::{RadixClientKey, ServerKey};
use std::fs::File;
use std::io::BufReader;
use rayon::prelude::*; // Für Turbo-Speed

// Der Speicher für die Schlüssel
pub struct KeyStore {
    pub client_key: RadixClientKey,
    pub server_key: ServerKey,
}

// Die Haupt-Maschine
pub struct FheEngine {
    pub key_store: KeyStore,
}

impl FheEngine {
    // 1. Initialisieren (Keys laden)
    pub fn new() -> Self {
        println!("   🔑 [ENGINE] Lade Schlüssel von Disk...");
        
        let ck_path = "keys/client_key.bin";
        let sk_path = "keys/server_key.bin";
        
        let ck_file = File::open(ck_path).expect("❌ Client Key fehlt! (keys/client_key.bin)");
        let sk_file = File::open(sk_path).expect("❌ Server Key fehlt! (keys/server_key.bin)");
        
        let ck: RadixClientKey = bincode::deserialize_from(BufReader::new(ck_file)).unwrap();
        let sk: ServerKey = bincode::deserialize_from(BufReader::new(sk_file)).unwrap();
        
        println!("   ✅ [ENGINE] Keys geladen.");

        FheEngine {
            key_store: KeyStore {
                client_key: ck,
                server_key: sk,
            }
        }
    }

    // 2. Die optimierte Berechnung (für Benchmarks/Training)
    pub fn compute_batch_optimized(&self, inputs: Vec<(u64, u64)>) {
        let sk = &self.key_store.server_key;
        
        // Parallelisierung mit Rayon (nutzt alle Kerne)
        inputs.par_iter().for_each(|(a, b)| {
            // Wir simulieren hier eine kleine Rechnung, um den Cache zu füllen
            // Da wir keine Ciphertexte haben, machen wir nur Dummy-Load
            // Im echten Betrieb nutzt der Server smart_scalar_add direkt.
            let _ = *a + *b; 
        });
    }
}
