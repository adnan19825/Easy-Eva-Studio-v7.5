#!/bin/bash
# Schnelle Query-Änderung mit Verifikation

if [ -z "$1" ]; then
    echo "Usage: $0 <query_value>"
    echo "Beispiele:"
    echo "  $0 5    # Non-match test"
    echo "  $0 7    # Match test"
    echo "  $0 0    # Edge case 0"
    echo "  $0 15   # Edge case 15 (max 4-bit)"
    exit 1
fi

QUERY="$1"
MAIN_RS="$HOME/fhe-workspace/fhe-main/src/main.rs"

echo "Ändere Query zu: $QUERY"

# Backup erstellen
cp "$MAIN_RS" "$MAIN_RS.backup.$(date +%s)"

# Query ersetzen mit Verifikation
if sed -i "s/let query = ck.encrypt([0-9]\+);/let query = ck.encrypt($QUERY);/" "$MAIN_RS"; then
    echo "✅ Datei aktualisiert"
    
    # Verifikation: Prüfe ob Änderung erfolgreich war
    if grep -q "let query = ck.encrypt($QUERY);" "$MAIN_RS"; then
        echo "✅ Verifikation: Query korrekt gesetzt"
        
        # Zeige Kontext
        echo ""
        echo "📄 Kontext in main.rs:"
        grep -n -B2 -A2 "let query = ck.encrypt" "$MAIN_RS"
        
        # Optional: Direkt ausführen
        echo ""
        read -p "Sofort ausführen? (j/n): " -n 1 -r
        echo ""
        if [[ $REPLY =~ ^[Jj]$ ]]; then
            cd ~/fhe-workspace
            cargo run --release -p fhe-main
        fi
    else
        echo "❌ Verifikation fehlgeschlagen - Änderung nicht gefunden"
        # Backup wiederherstellen
        mv "$MAIN_RS.backup" "$MAIN_RS" 2>/dev/null || echo "Backup nicht gefunden"
    fi
else
    echo "❌ Fehler beim Ändern der Datei"
    exit 1
fi
