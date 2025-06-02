 
#!/usr/bin/env bash

# Direktori tempat binary dan test-file berada:
BINARY="./target/release/tsp"
TESTDIR="./tests"

# Periksa apakah binary ada
if [[ ! -x "$BINARY" ]]; then
  echo "Error: binary $BINARY tidak ditemukan atau tidak executable."
  echo "Pastikan Anda sudah menjalankan: cargo build --release"
  exit 1
fi

# Loop lewat semua file *.txt di TESTDIR
for f in "$TESTDIR"/*.txt; do
  # Tampilkan nama file sebagai header
  echo "=============================="
  echo " Running test file: $f"
  echo "------------------------------"

  # Jalankan binary dalam mode batch dengan nama file sebagai argumen
  "$BINARY" "$f"

  echo    # baris kosong untuk pemisah
done
