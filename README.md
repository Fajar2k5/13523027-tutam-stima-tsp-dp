# TSP Solver dengan DP menggunakan bahasa Rust

**Deskripsi**  
Program ini mengimplementasikan solusi Traveling Salesman Problem (TSP) menggunakan algoritma Held–Karp (bit‐mask DP) di Rust. Tersedia dua mode penggunaan:

1. **Input File Mode**: Membaca file teks (`.txt`) berisi ukuran `n` dan matriks ketetanggaan, lalu mencetak hasil (biaya minimal dan rute).  
2. **Interactive Mode**: Terminal UI:
   - Memasukkan nilai `n` (jumlah simpul).  
   - Mengedit baris‐baris matriks ketetanggaan.  
   - Menekan tombol **Hitung** untuk menjalankan algoritma DP.  
   - Melihat hasil (Cost & Tour).

---

## Cara Kerja Secara Singkat

1. **Input**  
   - Jika dijalankan dengan _satu_ argumen nama file (misal `./tsp matrix.txt`), program akan otomatis:
     1. Membaca baris pertama (`n`),  
     2. Membaca `n×n` angka berikutnya sebagai matriks,  
     3. Memanggil fungsi `solve_tsp(matrix)` untuk mencari rute terpendek,  
     4. Mencetak:
        ```
        Cost: <biaya_minimum>
        Tour: 0 <...> 0
        ```
     5. Kemudian selesai (keluar).  
   - Jika **tanpa argumen**, jadilah mode UI:
     1. Program akan menampilkan prompt:
        ```
        Enter number of vertices (n):
        ```  
        User harus mengetik `n` (bilangan positif) dan tekan Enter.  
     2. Muncul UI dengan **n baris** yang awalnya berisi `0 0 0 ... 0`.  
     3. Gunakan **panah Atas/Bawah** untuk memilih baris, ketik angka dan spasi untuk mengedit baris ke-i.  
     4. Setelah selesai mengisi semua baris matriks, navigasikan ke tombol `[ Hitung ]` dengan menekan enter, lalu tekan lagi **Enter**.  
     5. Hasil DP langsung muncul di panel bawah:  
        ```
        Shortest Tour Cost: <biaya_minimum>
        Tour Sequence: [0, v1, v2, …, 0]
        ```  
     6. Tekan **Esc** (pada mode editor) atau **Ctrl+C** di mana saja untuk keluar dari program.  
     7. Tekan **Esc** saat tombol `[ Hitung ]` ter-highlight untuk kembali ke mode edit matriks (apabila ingin mengubah input).

2. **Algoritma Held–Karp (Bit‐Mask DP)**  
   - Menggunakan bitmask `mask` dengan panjang `n` bit untuk melacak subset simpul yang sudah dikunjungi.  
   - `dp[mask][j]` menyimpan biaya minimum untuk memulai di simpul 0, mengunjungi semua simpul dalam `mask`, dan berakhir di `j`.  
   - Transisi:  
     ```
     dp[mask][j] = min{i ∈ mask{j}} { dp[mask_without_j][i] + dist[i][j] }
     ```  
   - Setelah menghitung semua subset (`0 ≤ mask < 2^n` dengan bit 0 selalu set), ditambahkan biaya kembali ke 0:  
     ```
     best_cost = min{j ≠ 0} { dp[(1<<n)−1][j] + dist[j][0] }
     ```   

---

## Menjalankan Program Tanpa Kompilasi
1. Mode input interaktif:
   ```bash
   ./tsp
   ```
3. Mode input file:
   ```bash
   ./tsp {nama_file_txt}
   ```

## Instalasi & Kompilasi

1. **Pastikan Rust & Cargo terpasang**  
   ```bash
   rustc --version    # minimal v1.56.0
   cargo --version
2. Jalankan command untuk build dan run
   ```bash
   cargo build --release && cargo run
   ```
   Hasil executable berada di ./target/release

## Test Cases

Untuk menjalankan keenam _test cases_, jalankan ./test_all.sh
1. test1.txt
   ```txt
   1
   0
   ```
2. test2.txt
   ```txt
   2
   0 5
   3 0
   ```
3. test3.txt
   ```txt
   4
   0 10 15 20
   10 0 35 25
   15 35 0 30
   20 25 30 0
   ```
4. test4.txt
   ```txt
   5
   0  2  9  10 7
   1  0  6  4  3
   15 7  0  8  12
   6  3  12 0  5
   10 4  8  9  0
   ```
5. test5.txt
   ```txt
   6
   0  1000000000 2000000000 100 300000000 7
   1000000000 0  500000000 20 400000000 8
   2000000000 500000000 0  30 100000000 9
   100 20 30 0 60 10
   300000000 400000000 100000000 60 0 11
   7  8  9  10 11 0
   ```
6. test6.txt
   ```txt
   8
   0  3  1  5  8  2  7  4
   3  0  6  7  9  3  10 5
   1  6  0  4  2  5  8  6
   5  7  4  0  3  2  9  7
   8  9  2  3  0  1  6  4
   2  3  5  2  1  0  4  3
   7  10 8  9  6  4  0  5
   4  5  6  7  4  3  5  0
   ```
