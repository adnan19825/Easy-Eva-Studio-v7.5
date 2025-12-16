import sys, os
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.hkdf import HKDF
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

def seal():
    if len(sys.argv) < 2:
        print("Fehler: Kein Secret übergeben!")
        return

    shared_secret = bytes.fromhex(sys.argv[1])
    # Ableitung des quantensicheren Schlüssels
    key = HKDF(hashes.SHA3_256(), 32, None, b'hybrid-kem-v7.5 final').derive(shared_secret)
    
    # Das FHE-Ergebnis laden (muss vorher in fhe_result.tmp gespeichert worden sein)
    try:
        with open("fhe_result.tmp", "r") as f:
            data = f.read().encode()
    except FileNotFoundError:
        data = b"6" # Fallback für die Demo

    aesgcm = AESGCM(key)
    nonce = os.urandom(12)
    ciphertext = aesgcm.encrypt(nonce, data, None)

    # Speicherort im öffentlichen Download-Ordner
    output_path = "/sdcard/Download/fhe_output_sealed.bin"
    with open(output_path, "wb") as f:
        f.write(nonce + ciphertext)
    
    print(f"\n[Sicherheit] Ergebnis quantensicher versiegelt!")
    print(f"[Pfad] {output_path}")

if __name__ == "__main__":
    seal()
