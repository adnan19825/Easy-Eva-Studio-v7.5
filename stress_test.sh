#!/bin/bash
echo "🚀 Starte FHE-Stresstest: Generiere 20 Einträge..."
for i in {1..20}
do
   # Zufällige ID (0-255) und zufälliger Wert (0-255)
   ID=$((RANDOM % 256))
   VAL=$((RANDOM % 256))
   ./target/release/fhe-main add $ID $VAL > /dev/null
   echo -n "."
done
echo -e "\n✅ 20 Einträge generiert. Starte Zeitmessung für Suche..."
time ./target/release/fhe-main search 200
