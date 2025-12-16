# 🛡️ Easy-Eva-Studio v7.5 (Unified Crypto Engine PoC)

> **Status:** Architectural Preview | **Compliance:** NIST FIPS 203 Ready | **Tech:** C++17 / JNI / OpenFHE

Ein High-Performance Proof-of-Concept (PoC) für eine **Unified Crypto Engine** auf Android. Dieses Repository demonstriert, wie komplexe **Fully Homomorphic Encryption (FHE)** und moderner **Post-Quantum Key Exchange (PQC)** über eine einzige, speichersichere JNI-Architektur auf mobilen Endgeräten realisiert werden können.

### ⚡ Performance Benchmarks (Android / Termux)

Messungen auf Standard-Hardware (Snapdragon, No Root). Werte basieren auf nativer C++ Ausführung via JNI-Wrapper.

| Operation | Algorithmus | Zeit (avg) |
| :--- | :--- | :--- |
| **FHE Inferenz** | CKKS (Radix-4 NTT) | ~28.0 ms |
| **PQC KeyGen** | ML-KEM-1024 (Kyber Level 5) | **~0.80 ms** |
| **PQC Encaps** | ML-KEM-1024 | **~1.33 ms** |
| **PQC Decaps** | ML-KEM-1024 | **~0.70 ms** |
| **Total Handshake** | Hybrid (ECC + PQC) | **< 3.0 ms** |

### 🏗️ Architektur: Die "Unified Engine"

Das System trennt strikt zwischen **Data-in-Use** (Berechnung) und **Data-in-Transit** (Transport). Beide Engines teilen sich denselben C++ Runtime-Kern.

```mermaid
graph TD
    A[Android App / Java Layer] <-->|JNI Bridge| B(Unified C++ Wrapper);
    B <--> C{Crypto Core};
    C -->|Data-in-Use| D[FHE Engine / OpenFHE];
    C -->|Data-in-Transit| E[Hybrid PQC Engine];
    E --> F[ML-KEM-1024];
    E --> G[ECDH P-384];

### 🛡️ Unified Crypto Engine (Neu in v7.5)
Dieses Repository enthält jetzt eine hybride Schnittstelle (`simple_fhe_wrapper.cpp`), die zwei Welten verbindet:
1.  **High-Performance FHE:** Homomorphe Verschlüsselung via OpenFHE (CKKS/RNS) für Berechnungen auf verschlüsselten Daten.
2.  **Post-Quantum Security:** Vorbereitung für hybride Key-Encapsulation (ML-KEM-1024 + ECDH) gemäß NIST FIPS 203 Standards.

Ziel: Eine einheitliche JNI-Brücke für Android, die sowohl *Data-in-Use* (FHE) als auch *Data-in-Transit* (PQC) schützt.

*(Ende des Textes)*
---

**4. Speichern & Schließen:**
* Drücke `STRG` + `o`, dann `Enter`.
* Drücke `STRG` + `x`.

**5. Hochladen (Push):**
```bash
git add README.md
git commit -m "Docs: Refactor README with Architecture & Benchmarks"
git push

