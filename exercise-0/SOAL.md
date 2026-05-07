# SOAL - Exercise 0

## Judul

Service Traffic Mini Analyzer

## Durasi Simulasi

40-50 menit.

## Latar Belakang

Kamu diberi log traffic dari beberapa service internal. Sebagian request memakai REST, sebagian memakai gRPC. Sebagian gRPC memakai unary, sebagian memakai streaming. Tugasmu adalah membuat program kecil yang menganalisis log tersebut secara concurrent dan mencetak summary.

Exercise ini lebih sederhana dari Exercise 1 dan 2 karena kamu hanya perlu mengisi `src/main.rs`. Semua tipe data dan helper sudah tersedia di `src/lib.rs`.

## File yang Boleh Diubah

Dalam simulasi quiz, hanya ubah:

- `src/main.rs`

Jangan ubah `src/lib.rs` dan tests.

## Programming Problem

Lengkapi `main`.

Requirement:

1. Ambil data dari `sample_logs()`.
2. Gunakan `Instant::now()` untuk profiling waktu proses.
3. Gunakan minimal 2 worker thread dengan `thread::spawn`.
4. Gunakan `Arc<Mutex<Summary>>` sebagai shared summary.
5. Gunakan `mpsc::channel` agar worker mengirim progress message ke main thread.
6. Gunakan `join` untuk menunggu semua worker selesai.
7. Hitung:
   - total log;
   - jumlah REST;
   - jumlah gRPC;
   - jumlah streaming interaction;
   - jumlah success;
   - jumlah failed;
   - average latency.
8. Cetak report dengan `format_report`.

## Expected Output

Test tidak menuntut `PROFILE_ELAPSED_MS` bernilai tertentu, tetapi line lainnya harus benar.

```text
SERVICE_TRAFFIC_REPORT
TOTAL=8
REST=3
GRPC=5
STREAMING=3
SUCCESS=6
FAILED=2
AVG_LATENCY_MS=50
WORKER_MESSAGES=8
PROFILE_ELAPSED_MS=<angka>
```

## Essay Questions

1. Dalam program ini, bagian mana yang menunjukkan shared memory model dan bagian mana yang menunjukkan message passing model?
2. Mengapa `Instant` bukan profiler lengkap, tetapi tetap berguna untuk profiling sederhana?
3. Dari data sample, mengapa `WatchTransactions` atau `Chat` lebih cocok dimodelkan sebagai gRPC streaming dibanding REST unary biasa?
4. Mengapa output report perlu deterministic walaupun pemrosesan dilakukan concurrent?

## Rubrik Mandiri

- 30%: memakai concurrency primitive dengan benar.
- 25%: summary count benar.
- 15%: output sesuai format.
- 15%: memakai profiling sederhana dengan `Instant`.
- 15%: essay mengaitkan REST/gRPC/profiling/concurrency ke program.

## Hints Bertahap

Hint 1: Buat bucket log untuk worker, seperti round-robin berdasarkan index.

Hint 2: Lock `Summary` hanya saat update count.

Hint 3: Kirim progress message sederhana, misalnya `"worker-0:REQ-001"`.

Hint 4: Sort progress messages sebelum menghitung atau mencetak jika kamu ingin debug output stabil.
