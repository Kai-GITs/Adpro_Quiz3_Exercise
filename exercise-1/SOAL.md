# SOAL - Exercise 1

## Judul

Concurrent Order Batch Processor

## Durasi Simulasi

50 menit.

## Latar Belakang

Sebuah toko online menerima batch order request. Setiap request meminta sejumlah item berdasarkan `sku`. Karena jumlah request bisa banyak, sistem ingin memproses request secara concurrent menggunakan beberapa worker thread.

Masalahnya, semua worker membaca dan mengubah inventory yang sama. Jika update inventory tidak dijaga, sistem bisa oversell, mengalami lost update, atau menghasilkan report yang tidak konsisten.

## File yang Boleh Diubah

Dalam simulasi quiz, fokus ubah:

- `src/lib.rs`

Jangan ubah tests kecuali quiz memang meminta.

## Programming Problem

Lengkapi:

```rust
pub fn process_batch(
    initial_inventory: HashMap<String, u32>,
    requests: Vec<Request>,
    workers: usize,
) -> BatchReport
```

Target behavior:

- Semua request diproses tepat satu kali.
- Request valid dan stock cukup menghasilkan `ReceiptStatus::Fulfilled`.
- Request `qty == 0` menghasilkan `ReceiptStatus::RejectedInvalidQuantity`.
- Request untuk SKU tidak ada atau stock tidak cukup menghasilkan `ReceiptStatus::RejectedOutOfStock`.
- Inventory akhir tidak boleh negatif dan tidak boleh oversell.
- `workers == 0` diperlakukan sebagai 1 worker.
- `fulfilled_count` dan `rejected_count` harus konsisten dengan isi `receipts`.

Constraint teknis:

- Wajib memakai `Arc<Mutex<HashMap<String, u32>>>`.
- Wajib memakai `thread::spawn`.
- Wajib memakai `move` closure.
- Wajib memakai `mpsc::channel` untuk mengirim `Receipt` dari worker ke main thread.
- Wajib memakai `join`.

## Acceptance Tests

Jalankan:

```powershell
cargo test
```

Test menilai:

- request valid, invalid, dan out-of-stock;
- `workers == 0`;
- banyak thread yang memperebutkan SKU yang sama;
- empty request list.

## Essay Questions

Jawab singkat tetapi bernalar. Format ideal: definisi, kaitkan ke code, lalu sebut konsekuensi/trade-off.

1. Jelaskan perbedaan shared memory model dan message passing model. Dalam project ini, bagian mana yang memakai shared memory dan bagian mana yang memakai message passing?
2. Mengapa `Arc` diperlukan ketika `Mutex<HashMap<...>>` ingin digunakan oleh banyak thread? Mengapa `Rc` bukan pilihan yang tepat?
3. Apa hubungan antara race condition, lost update, critical section, dan `Mutex` pada kasus inventory ini?
4. Mengapa closure pada `thread::spawn` biasanya membutuhkan keyword `move`? Jelaskan memakai konsep ownership dan lifetime.
5. Sebutkan satu skenario deadlock yang mungkin terjadi jika project ini dikembangkan menjadi lebih kompleks, misalnya ada dua shared resource.

## Rubrik Mandiri

- 40%: concurrency structure benar (`Arc`, `Mutex`, `spawn`, `move`, `join`).
- 25%: business logic benar untuk fulfilled, invalid quantity, dan out-of-stock.
- 15%: report benar dan tidak tergantung urutan nondeterministic.
- 20%: essay menjelaskan konsep dan mengaitkan ke project.

## Hints Bertahap

Hint 1: Jangan share `Vec<Receipt>` dengan `Mutex` jika requirement meminta `mpsc`; worker cukup `send` receipt.

Hint 2: Lock inventory hanya saat mengecek dan mengurangi stock. Jangan pegang lock saat melakukan pekerjaan lain.

Hint 3: Setelah semua `tx.clone()` masuk worker, `drop(tx)` di main thread membantu receiver berhenti ketika semua worker selesai.

Hint 4: Jika urutan receipt berbeda antar run, itu normal. Sort receipt berdasarkan `request_id` sebelum membangun report jika ingin hasil deterministic.
