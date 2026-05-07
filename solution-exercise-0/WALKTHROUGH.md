# Walkthrough Solusi - Exercise 0

Exercise 0 dibuat sebagai latihan pemanasan. Kamu hanya mengisi `src/main.rs`, tetapi konsep yang disentuh cukup lengkap: ownership, concurrency, shared memory, message passing, profiling sederhana, dan REST/gRPC classification.

## 1. Baca `src/lib.rs` Sebagai API yang Sudah Disediakan

Di quiz project-based, tidak semua file harus diubah. Pada Exercise 0, `src/lib.rs` sudah menyediakan:

- `RequestLog`: satu catatan traffic service;
- `Protocol`: `Rest` atau `Grpc`;
- `Interaction`: unary atau streaming;
- `Summary`: accumulator untuk report;
- `sample_logs()`: data input;
- `is_streaming()`: helper untuk menentukan interaction streaming;
- `format_report()`: formatter output yang dites.

Artinya, tugas utama di `main` adalah orchestration: mengambil data, membagi kerja, menjalankan worker, mengumpulkan hasil, lalu mencetak report.

`main.rs` mengakses library dengan:

```rust
use solution_exercise_0::{
    format_report, is_streaming, sample_logs, Protocol, Summary,
};
```

Nama package di `Cargo.toml` adalah `solution-exercise-0`, tetapi import path menjadi `solution_exercise_0`.

## 2. Mulai dari Profiling Timer

Requirement meminta profiling sederhana. Gunakan:

```rust
let started = Instant::now();
```

Timer ini dimulai sebelum kerja utama. Nanti setelah worker selesai:

```rust
let elapsed_ms = started.elapsed().as_millis();
```

`Instant` bukan profiler detail, tetapi cukup untuk melihat durasi total program.

## 3. Siapkan Shared Summary

Karena beberapa worker akan mengupdate count yang sama, summary perlu shared ownership dan lock:

```rust
let summary = Arc::new(Mutex::new(Summary::default()));
```

Maknanya:

- `Arc` membuat banyak thread bisa punya handle ke `Summary` yang sama.
- `Mutex` memastikan hanya satu thread yang update summary pada satu waktu.

Tanpa `Mutex`, update seperti `total += 1` bisa mengalami lost update.

## 4. Siapkan Message Passing

Worker perlu memberi tahu main thread bahwa satu log sudah diproses:

```rust
let (tx, rx) = mpsc::channel();
```

Setiap worker mendapat `tx.clone()`. Setelah semua worker dibuat, main thread memanggil `drop(tx)` supaya receiver tahu kapan semua sender selesai.

## 5. Bagi Logs ke Worker

Exercise ini memakai 2 worker agar sederhana:

```rust
let worker_count = 2;
let mut buckets = vec![Vec::new(); worker_count];

for (idx, log) in logs.into_iter().enumerate() {
    buckets[idx % worker_count].push(log);
}
```

Round-robin membuat setiap log masuk tepat satu bucket.

## 6. Spawn Worker

Setiap worker perlu membawa bucket log, clone summary, dan clone sender:

```rust
let summary = Arc::clone(&summary);
let tx = tx.clone();

thread::spawn(move || {
    // process bucket
});
```

`move` diperlukan karena thread harus memiliki data yang dipakai di dalam closure. Kalau hanya meminjam dari stack `main`, Rust tidak bisa menjamin data masih hidup saat thread berjalan.

## 7. Update Summary dalam Critical Section

Di dalam worker, setiap log memperbarui count:

```rust
let mut summary = summary.lock().unwrap();
summary.total += 1;
summary.total_latency_ms += log.latency_ms;
```

Lalu klasifikasi:

- `Protocol::Rest` menambah `rest`;
- `Protocol::Grpc` menambah `grpc`;
- interaction streaming menambah `streaming`;
- `success == true` menambah `success`, selain itu `failed`.

Lock hanya perlu dipegang saat update summary. Setelah itu worker bisa mengirim progress message.

## 8. Kirim Progress Message

Setelah update summary:

```rust
tx.send(format!("worker-{worker_id}:{}", log.id)).unwrap();
```

Message ini membuat exercise menyentuh message passing. Tests hanya memeriksa jumlah message, bukan urutannya, karena urutan thread tidak deterministic.

## 9. Join Worker dan Collect Messages

Setelah spawn semua worker:

```rust
drop(tx);

for handle in handles {
    handle.join().unwrap();
}

let messages: Vec<String> = rx.into_iter().collect();
```

`join` memastikan semua worker selesai sebelum report dicetak. `drop(tx)` memastikan iterator receiver berhenti setelah semua worker sender drop.

## 10. Print Report

Ambil snapshot summary:

```rust
let summary = summary.lock().unwrap().clone();
let elapsed_ms = started.elapsed().as_millis();
println!("{}", format_report(&summary, messages.len(), elapsed_ms));
```

Output aggregate deterministic:

```text
TOTAL=8
REST=3
GRPC=5
STREAMING=3
SUCCESS=6
FAILED=2
AVG_LATENCY_MS=50
WORKER_MESSAGES=8
```

Hanya `PROFILE_ELAPSED_MS` yang boleh berubah antar run.

## 11. Mapping ke Materi

- Module 06 Concurrency: `thread::spawn`, `move`, `Arc`, `Mutex`, `mpsc`, `join`.
- Module 07 Profiling: `Instant` sebagai measurement sederhana dan konsep aggregate metric.
- Module 08 Networking: REST vs gRPC, unary vs streaming, dan alasan gRPC cocok untuk streaming service.

## 12. Kesalahan yang Sering Terjadi

- Mengupdate `Summary` tanpa `Mutex`.
- Lupa `drop(tx)`, lalu receiver menunggu terus.
- Lupa `join`, sehingga report bisa dicetak sebelum worker selesai.
- Mengasumsikan urutan progress message selalu sama.
- Menghitung average latency sebelum semua worker selesai.
