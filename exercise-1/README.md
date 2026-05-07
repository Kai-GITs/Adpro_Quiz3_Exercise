# EXERCISE 1 - Concurrent Order Batch Processor

Bahasa pengerjaan bebas, tetapi jawaban essay disarankan memakai Bahasa Indonesia dengan technical term English.

## Konteks

Kamu diminta memperbaiki batch processor sederhana untuk toko online. Setiap `Request` berisi permintaan pembelian SKU tertentu. Program harus memproses banyak request secara concurrent, menjaga stok inventory tetap konsisten, dan mengirim hasil pemrosesan kembali ke main thread.

Waktu simulasi quiz: 50 menit.

## Mengapa Project Ini Tidak Memiliki `.proto`?

Exercise 1 sengaja tidak memakai `.proto` karena fokusnya adalah Module 06: Rust ownership, borrowing, multi-threading, shared memory model, `Arc`, `Mutex`, dan `mpsc`. File `.proto` dipakai untuk mendefinisikan kontrak gRPC service dan message pada project networking/gRPC. Karena Exercise 1 bukan RPC service dan tidak punya client-server boundary, menambahkan `.proto` di sini justru membuat latihan tidak fokus.

Materi `.proto` ada di Exercise 2, karena Exercise 2 memang mensimulasikan gRPC service.

## Programming Task

Lengkapi fungsi `process_batch` di `src/lib.rs`.

Versi soal yang lebih formal tersedia di [SOAL.md](SOAL.md).

Requirement:

1. Gunakan `Arc<Mutex<HashMap<String, u32>>>` untuk shared inventory.
2. Gunakan `thread::spawn` untuk membuat worker thread.
3. Gunakan `move` closure dengan benar agar data yang dibutuhkan worker berpindah secara aman.
4. Gunakan `mpsc::channel` untuk mengirim `Receipt` dari worker thread ke main thread.
5. Gunakan `join` agar main thread menunggu seluruh worker selesai.
6. Jika `workers == 0`, perlakukan sebagai 1 worker.
7. Request dengan `qty == 0` harus ditolak sebagai `RejectedInvalidQuantity`.
8. Request dengan stock tidak cukup harus ditolak sebagai `RejectedOutOfStock`.
9. Request valid dengan stock cukup harus mengurangi inventory dan menghasilkan `Fulfilled`.

## Cara Mengecek

```bash
cargo test
```

Project starter ini harus compile, tetapi test akan gagal sampai `todo!()` diisi.

Untuk memastikan starter hanya compile tanpa menjalankan failing tests:

```bash
cargo test --no-run
```

## Essay Questions

Jawab di file Markdown terpisah ketika latihan.

1. Jelaskan perbedaan shared memory model dan message passing model. Dalam project ini, bagian mana yang memakai shared memory dan bagian mana yang memakai message passing?
2. Mengapa `Arc` diperlukan ketika `Mutex<HashMap<...>>` ingin digunakan oleh banyak thread? Mengapa `Rc` bukan pilihan yang tepat?
3. Apa hubungan antara race condition, lost update, critical section, dan `Mutex` pada kasus inventory ini?
4. Mengapa closure pada `thread::spawn` biasanya membutuhkan keyword `move`? Jelaskan memakai konsep ownership dan lifetime.
5. Sebutkan satu skenario deadlock yang mungkin terjadi jika project ini dikembangkan menjadi lebih kompleks, misalnya ada dua shared resource.
