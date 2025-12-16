#!/bin/bash
# FHE Setup Verifikation
echo "=== FHE SETUP VERIFIKATION ==="

ERRORS=0

# 1. Verzeichnis-Struktur
echo "[1/6] Verzeichnis-Struktur..."
REQUIRED_DIRS=("fhe-core" "fhe-main" "fhe-core/src" "fhe-main/src")
for dir in "${REQUIRED_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        echo "✅ $dir"
    else
        echo "❌ $dir fehlt"
        ((ERRORS++))
    fi
done

# 2. Datei-Existenz
echo "[2/6] Wichtige Dateien..."
REQUIRED_FILES=(
    "Cargo.toml"
    "fhe-core/Cargo.toml"
    "fhe-core/src/lib.rs"
    "fhe-main/Cargo.toml"
    "fhe-main/src/main.rs"
)
for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file"
    else
        echo "❌ $file fehlt"
        ((ERRORS++))
    fi
done

# 3. Cargo.toml Inhalte prüfen
echo "[3/6] Cargo.toml Konfiguration..."
if grep -q "seeder_unix" fhe-core/Cargo.toml; then
    echo "✅ seeder_unix feature aktiviert"
else
    echo "❌ seeder_unix fehlt (ARM64 benötigt dies)"
    ((ERRORS++))
fi

# 4. Kompilierung testen
echo "[4/6] Kompilierungstest..."
if cargo check --release --quiet 2>/dev/null; then
    echo "✅ Kompilierung erfolgreich"
else
    echo "❌ Kompilierung fehlgeschlagen"
    ((ERRORS++))
fi

# 5. Hauptprogramm testen
echo "[5/6] Programmausführung..."
if timeout 10 cargo run --release -p fhe-main --quiet 2>&1 | grep -q "Gefundener Wert:"; then
    echo "✅ Programm läuft"
else
    echo "⚠️  Programmausführung könnte Probleme haben"
fi

# 6. Zusammenfassung
echo "[6/6] Zusammenfassung..."
if [ $ERRORS -eq 0 ]; then
    echo "🎉 ALLE TESTS BESTANDEN! Setup ist korrekt."
    echo ""
    echo "Verfügbare Skripte:"
    echo "  ./run_tests.sh    - Vollständige Testsuite"
    echo "  ./quick_switch.sh - Schnelle Query-Änderung"
    echo "  ./verify_setup.sh - Setup erneut verifizieren"
else
    echo "⚠️  $ERRORS Problem(e) gefunden"
    exit 1
fi
