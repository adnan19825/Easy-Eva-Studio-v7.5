pub use tfhe::shortint::prelude::*;
use tfhe::shortint::parameters::PARAM_MESSAGE_4_CARRY_1_KS_PBS;
use std::fs::File;
use std::io::{Write, Read};

pub fn setup_fhe() -> (ClientKey, ServerKey) {
    gen_keys(PARAM_MESSAGE_4_CARRY_1_KS_PBS)
}

pub fn save_keys(ck: &ClientKey, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let encoded = bincode::serialize(ck)?;
    File::create(path)?.write_all(&encoded)?;
    Ok(())
}

pub fn load_keys(path: &str) -> Result<ClientKey, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;
    Ok(bincode::deserialize(&buffer)?)
}

pub fn encrypt_8bit(ck: &ClientKey, val: u64) -> (Ciphertext, Ciphertext) {
    (ck.encrypt(val % 16), ck.encrypt((val / 16) % 16))
}

pub fn decrypt_8bit(ck: &ClientKey, lsb: &Ciphertext, msb: &Ciphertext) -> u64 {
    ck.decrypt(lsb) + (ck.decrypt(msb) * 16)
}

pub fn compare_8bit(sk: &ServerKey, q: &(Ciphertext, Ciphertext), db: &(Ciphertext, Ciphertext)) -> Ciphertext {
    let lut = sk.generate_lookup_table(|x| if x % 16 == 0 { 1 } else { 0 });
    let m_lsb = sk.apply_lookup_table(&mut sk.unchecked_sub(&q.0, &db.0), &lut);
    let m_msb = sk.apply_lookup_table(&mut sk.unchecked_sub(&q.1, &db.1), &lut);
    let mut m_l = m_lsb;
    let mut m_m = m_msb;
    sk.smart_mul_lsb(&mut m_l, &mut m_m)
}

pub fn is_not_match(sk: &ServerKey, q: &(Ciphertext, Ciphertext), db: &(Ciphertext, Ciphertext)) -> Ciphertext {
    let lut = sk.generate_lookup_table(|x| if x % 16 == 0 { 0 } else { 1 });
    let m_lsb = sk.apply_lookup_table(&mut sk.unchecked_sub(&q.0, &db.0), &lut);
    let m_msb = sk.apply_lookup_table(&mut sk.unchecked_sub(&q.1, &db.1), &lut);
    sk.unchecked_add(&m_lsb, &m_msb) 
}

pub fn save_vault(data: &(Vec<(Ciphertext, Ciphertext)>, Vec<(Ciphertext, Ciphertext)>), path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let encoded = bincode::serialize(data)?;
    File::create(path)?.write_all(&encoded)?;
    Ok(())
}

pub fn load_vault(path: &str) -> Result<(Vec<(Ciphertext, Ciphertext)>, Vec<(Ciphertext, Ciphertext)>), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;
    Ok(bincode::deserialize(&buffer)?)
}
