# EXERCISE 2 - Mini gRPC Payment Service

Exercise ini lebih sulit daripada simulasi quiz biasa. Tujuannya adalah memperkuat Rust gRPC, async, streaming, dan profiling reasoning.

## Konteks

Kamu membangun service internal untuk microservice payment. Service ini memakai gRPC karena butuh kontrak interface yang jelas lewat protobuf dan streaming untuk transaction history/chat.

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

## Kegunaan `.proto`

File `proto/quiz_service.proto` adalah kontrak utama gRPC. Di sana didefinisikan:

- service apa yang tersedia, yaitu `QuizService`;
- RPC method apa yang bisa dipanggil, yaitu `SubmitPayment`, `WatchTransactions`, dan `Chat`;
- bentuk request dan response message, seperti `PaymentRequest`, `PaymentReply`, `TransactionEvent`, dan `ChatMessage`;
- apakah komunikasi unary atau streaming.

Saat `cargo build` atau `cargo test`, `build.rs` menjalankan `tonic-build`. Tool ini memakai `protoc` untuk membaca `.proto`, lalu generate Rust type dan trait gRPC server. Jadi `.proto` bukan file dekorasi; tanpa `.proto`, client dan server tidak punya kontrak typed yang sama.

## Programming Task

Lengkapi `src/lib.rs`.

Versi soal yang lebih formal tersedia di [SOAL.md](SOAL.md).

Requirement:

1. `SubmitPayment` adalah unary RPC.
   - Request valid jika `payment_id` tidak kosong, `user_id` tidak kosong, dan `amount > 0`.
   - Request valid menghasilkan `PaymentReply.accepted = true`.
   - Request valid juga menambahkan satu `TransactionEvent` baru dengan status `SETTLED`.
   - Request invalid menghasilkan `PaymentReply.accepted = false`.
2. `WatchTransactions` adalah server streaming RPC.
   - Filter event berdasarkan `user_id`.
   - Jika `limit == 0`, kirim semua event user tersebut.
   - Jika `limit > 0`, kirim maksimal `limit` event.
3. `Chat` adalah bidirectional streaming RPC.
   - Untuk setiap pesan masuk, server mengirim balasan.
   - Body kosong dibalas dengan `NACK <sender>: empty message`.
   - Body tidak kosong dibalas dengan `ACK <sender>: <body>`.
4. Gunakan generated code dari `proto/quiz_service.proto`.
5. Pertahankan test helper yang sudah disediakan karena test memeriksa behavior logic secara langsung.

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

1. Mengapa gRPC cocok untuk komunikasi antar microservice dibanding REST pada kasus yang butuh streaming dan kontrak schema kuat?
2. Jelaskan peran protobuf dalam gRPC. Mengapa protobuf sering lebih efisien daripada JSON?
3. Jelaskan hubungan HTTP/2, multiplexing, dan streaming dalam gRPC.
4. Dalam konteks Tonic, mengapa `tokio` dan `async/await` penting untuk performa service?
5. Bedakan performance testing dan profiling. Jika endpoint `WatchTransactions` terasa lambat, langkah investigasi apa yang akan kamu lakukan?
