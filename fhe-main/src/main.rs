use fhe_core::*;
use clap::{Parser, Subcommand};
use rayon::prelude::*;
use std::path::Path;

#[derive(Parser)]
struct Cli { #[command(subcommand)] command: Commands }

#[derive(Subcommand)]
enum Commands {
    Add { id: u64, value: u64 },
    Search { id: u64 },
    Delete { id: u64 },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let ck = load_keys("client_vault.key")?;
    let sk = ServerKey::new(&ck);
    let db_path = "vault.db";

    let (mut ids, mut vals) = if Path::new(db_path).exists() {
        load_vault(db_path)?
    } else { (Vec::new(), Vec::new()) };

    match &cli.command {
        Commands::Add { id, value } => {
            ids.push(encrypt_8bit(&ck, *id));
            vals.push(encrypt_8bit(&ck, *value));
            save_vault(&(ids, vals), db_path)?;
            println!("✅ Patient {} gespeichert.", id);
        }
        Commands::Search { id } => {
            println!("[*] Paralleler Scan nach ID {}...", id);
            let q = encrypt_8bit(&ck, *id);
            let res = ids.par_iter().zip(vals.par_iter())
                .map(|(id_t, val_t)| {
                    let mut m = compare_8bit(&sk, &q, id_t);
                    let (mut v_l, mut v_m) = (val_t.0.clone(), val_t.1.clone());
                    (sk.smart_mul_lsb(&mut m, &mut v_l), sk.smart_mul_lsb(&mut m, &mut v_m))
                })
                .reduce(|| (sk.create_trivial(0), sk.create_trivial(0)), |mut acc, mut next| {
                    (sk.smart_add(&mut acc.0, &mut next.0), sk.smart_add(&mut acc.1, &mut next.1))
                });
            println!("\n=== ERGEBNIS ===\nID {}: {}", id, decrypt_8bit(&ck, &res.0, &res.1));
        }
        Commands::Delete { id } => {
            println!("[*] Lösche ID {} (Sicherer Scan-&-Filter)...", id);
            let q = encrypt_8bit(&ck, *id);
            
            // Wir behalten nur Einträge, die homomorph NICHT übereinstimmen
            let mut new_ids = Vec::new();
            let mut new_vals = Vec::new();
            
            for (id_t, val_t) in ids.into_iter().zip(vals.into_iter()) {
                let check = is_not_match(&sk, &q, &id_t);
                if ck.decrypt(&check) > 0 {
                    new_ids.push(id_t);
                    new_vals.push(val_t);
                }
            }
            save_vault(&(new_ids, new_vals), db_path)?;
            println!("✅ Löschvorgang für ID {} abgeschlossen.", id);
        }
    }
    Ok(())
}
