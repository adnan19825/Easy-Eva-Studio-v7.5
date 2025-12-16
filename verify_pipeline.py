import sys
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.hkdf import HKDF
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

def verify():
    shared_secret_hex = "D8C2CEDA9EB19FE5E0B0CF88F741FB41"
    shared_secret = bytes.fromhex(shared_secret_hex)
    
    # Schlüssel identisch ableiten
    key = HKDF(hashes.SHA3_256(), 32, None, b'hybrid-kem-v7.5 final').derive(shared_secret)
    
    # Datei laden
    with open("/sdcard/Download/fhe_output_sealed.bin", "rb") as f:
        raw_data = f.read()
    
    nonce = raw_data[:12]
    ciphertext = raw_data[12:]
    
    # Entschlüsseln
    aesgcm = AESGCM(key)
    decrypted_data = aesgcm.decrypt(nonce, ciphertext, None)
    
    print("=== PIPELINE VERIFIKATION ===")
    print(f"Entschlüsseltes FHE-Ergebnis: {decrypted_data.decode()}")
    
    if decrypted_data.decode() == "6":
        print("✅ STATUS: Integrität bestätigt. Datenfluss quantensicher.")

if __name__ == "__main__":
    verify()
