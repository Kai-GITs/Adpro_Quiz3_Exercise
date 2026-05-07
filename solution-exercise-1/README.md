# SOLUTION - EXERCISE 1

Project ini berisi solusi lengkap untuk `EXERCISE 1 - Concurrent Order Batch Processor`.

Pelajari urutan berpikirnya di [WALKTHROUGH.md](WALKTHROUGH.md), lalu baca jawaban essay di [ANSWERS.md](ANSWERS.md).

Jalankan:

```bash
cargo test
```

Poin utama implementasi:

- `Arc` dipakai agar ownership inventory bisa dibagikan ke banyak worker secara thread-safe.
- `Mutex` melindungi critical section saat membaca dan mengurangi stok.
- `mpsc::channel` dipakai agar worker mengirim hasil ke main thread tanpa menulis langsung ke vector bersama.
- `join` memastikan semua worker selesai sebelum report dibuat.

Project ini tidak memakai `.proto` karena bukan gRPC service. Fokusnya adalah concurrency dengan shared memory dan message passing di dalam satu Rust process.
