### 🛡️ Unified Crypto Engine (Neu in v7.5)
Dieses Repository enthält jetzt eine hybride Schnittstelle (`simple_fhe_wrapper.cpp`), die zwei Welten verbindet:
1.  **High-Performance FHE:** Homomorphe Verschlüsselung via OpenFHE (CKKS/RNS) für Berechnungen auf verschlüsselten Daten.
2.  **Post-Quantum Security:** Vorbereitung für hybride Key-Encapsulation (ML-KEM-1024 + ECDH) gemäß NIST FIPS 203 Standards.

Ziel: Eine einheitliche JNI-Brücke für Android, die sowohl *Data-in-Use* (FHE) als auch *Data-in-Transit* (PQC) schützt.

