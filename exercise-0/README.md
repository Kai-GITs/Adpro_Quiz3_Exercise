# EXERCISE 0 - Service Traffic Mini Analyzer

Exercise ini adalah jembatan sebelum Exercise 1 dan Exercise 2. Programnya lebih sederhana dan hanya meminta kamu mengisi `src/main.rs`, tetapi tetap menyentuh materi lengkap: Rust ownership, concurrency, shared memory, message passing, profiling sederhana, REST vs gRPC, dan streaming.

Versi soal formal tersedia di [SOAL.md](SOAL.md).

## Cara Mengecek

```powershell
cargo test
```

Starter project ini compile, tetapi test gagal karena `main` masih `todo!()`.

Untuk compile-check tanpa menjalankan binary test:

```powershell
cargo test --no-run
```

## Target Belajar

- Menggunakan data dari `sample_logs()`.
- Memakai `thread::spawn`, `Arc<Mutex<_>>`, dan `mpsc`.
- Mengukur waktu kerja dengan `Instant`.
- Mengklasifikasikan request REST/gRPC dan unary/streaming.
- Menghasilkan output report deterministic.

## Catatan `lib.rs` dan `main.rs`

Di project ini, `src/lib.rs` berisi type dan helper yang sudah disediakan, seperti `RequestLog`, `Summary`, `sample_logs`, dan `format_report`.

Tugas kamu hanya mengisi `src/main.rs`, lalu menghubungkan helper dari library dengan:

```rust
use exercise_0::{format_report, sample_logs, Summary};
```

Package name di `Cargo.toml` adalah `exercise-0`, tetapi import Rust memakai `exercise_0` karena hyphen berubah menjadi underscore.
