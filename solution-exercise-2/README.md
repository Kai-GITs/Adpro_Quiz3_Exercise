# SOLUTION - EXERCISE 2

Project ini berisi solusi lengkap untuk `EXERCISE 2 - Mini gRPC Payment Service`.

Pelajari urutan berpikirnya di [WALKTHROUGH.md](WALKTHROUGH.md), lalu baca jawaban essay di [ANSWERS.md](ANSWERS.md).

## Dependency

Project ini membutuhkan Rust/Cargo dan `protoc`.

Jika `protoc` belum tersedia, install dengan:

```powershell
winget install --id Google.Protobuf --exact --accept-source-agreements --accept-package-agreements
```

Setelah install, restart terminal/IDE agar PATH baru terbaca. Cek dengan:

```powershell
protoc --version
```

Jalankan:

```bash
cargo test
```

Untuk menjalankan server lokal:

```bash
cargo run
```

Poin utama implementasi:

- Unary RPC cocok untuk request-response tunggal seperti `SubmitPayment`.
- Server streaming cocok untuk `WatchTransactions` karena server bisa mengirim beberapa `TransactionEvent` untuk satu request.
- Bidirectional streaming cocok untuk `Chat` karena client dan server sama-sama bisa mengirim sequence message.
- `tokio` menjalankan async task dan Tonic membangun gRPC service di atas async Rust.
- `.proto` adalah kontrak service dan message yang dipakai `tonic-build` untuk generate Rust code.
