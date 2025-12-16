#!/bin/bash
# FHE Test Suite - Automatische Verifikation
set -e  # Bei Fehler abbrechen

echo "=== FHE TEST SUITE v1.0 ==="
echo "Plattform: $(uname -m) - $(uname -s)"
echo "Datum: $(date)"
echo ""

cd ~/fhe-workspace

# 1. Kompilierung testen
echo "[1/5] Kompilierung testen..."
if cargo check --release -p fhe-main; then
    echo "✅ Kompilierung erfolgreich"
else
    echo "❌ Kompilierung fehlgeschlagen"
    exit 1
fi

# 2. Schlüssel-Verwaltung
echo "[2/5] Schlüssel-Verwaltung testen..."
if [ -f "client_vault.key" ]; then
    echo "🔑 Vorhandenen Schlüssel verwenden"
    KEY_ACTION="load"
else
    echo "🔑 Neue Schlüssel generieren"
    KEY_ACTION="generate"
fi

# 3. Testfälle definieren
declare -A test_cases=(
    ["non_match"]="5"
    ["match"]="7"
    ["edge_0"]="0"
    ["edge_15"]="15"  # Max für 4-bit
)

# 4. Jeden Testfall ausführen
echo "[3/5] Testfälle ausführen..."
for test_name in "${!test_cases[@]}"; do
    query_value="${test_cases[$test_name]}"
    
    echo "--- Test: $test_name (Query=$query_value) ---"
    
    # Query-Wert in main.rs ersetzen
    sed -i "s/let query = ck.encrypt([0-9]\+);/let query = ck.encrypt($query_value);/" fhe-main/src/main.rs
    
    # Programm ausführen und Resultat extrahieren
    if output=$(cargo run --release -p fhe-main --quiet 2>&1); then
        # Ergebnis extrahieren
        result=$(echo "$output" | grep "Gefundener Wert:" | awk '{print $3}')
        enc_time=$(echo "$output" | grep "Encryption time:" | awk '{print $3}')
        fhe_time=$(echo "$output" | grep "FHE operation time:" | awk '{print $4}')
        
        # Erwartetes Ergebnis berechnen
        if [ "$query_value" = "7" ]; then
            expected="13"
        else
            expected="0"
        fi
        
        # Verifikation
        if [ "$result" = "$expected" ]; then
            echo "✅ PASS: Result=$result (Expected=$expected) | Enc:${enc_time:-N/A} FHE:${fhe_time:-N/A}"
            echo "$test_name,pass,$result,$expected,$enc_time,$fhe_time" >> test_results.csv
        else
            echo "❌ FAIL: Result=$result (Expected=$expected)"
            echo "$test_name,fail,$result,$expected,$enc_time,$fhe_time" >> test_results.csv
        fi
    else
        echo "❌ EXECUTION FAILED"
        echo "$test_name,error,,,," >> test_results.csv
    fi
    
    sleep 0.5  # Kurze Pause zwischen Tests
done

# 5. Zusammenfassung
echo ""
echo "[4/5] Zusammenfassung..."
if [ -f "test_results.csv" ]; then
    echo "📊 Testergebnisse:"
    echo "Testname, Status, Result, Expected, EncTime, FHETime"
    cat test_results.csv
    
    passes=$(grep -c ",pass," test_results.csv 2>/dev/null || echo "0")
    total=$(wc -l < test_results.csv 2>/dev/null || echo "0")
    
    echo ""
    echo "📈 Statistik: $passes/$total Tests bestanden"
    
    if [ "$passes" -eq "$total" ] && [ "$total" -gt 0 ]; then
        echo "🎉 ALLE TESTS BESTANDEN!"
    else
        echo "⚠️  Einige Tests fehlgeschlagen"
    fi
fi

echo ""
echo "[5/5] Aufräumen..."
# Zurück zu Standard-Query (5 für non-match)
sed -i "s/let query = ck.encrypt([0-9]\+);/let query = ck.encrypt(5);/" fhe-main/src/main.rs

echo "✅ Testsuite abgeschlossen!"
