# Penjelasan `.proto` dan Struktur Project

## Mengapa Exercise 1 Tidak Ada `.proto`?

Exercise 1 tidak memiliki `.proto` karena scope-nya adalah concurrency di dalam satu Rust process. Materi yang diuji:

- ownership dan borrowing;
- `thread::spawn` dan `move` closure;
- `Arc` untuk shared ownership lintas thread;
- `Mutex` untuk melindungi critical section;
- `mpsc` untuk message passing antar thread;
- `join` agar main thread menunggu worker selesai.

Tidak ada RPC boundary di Exercise 1. Tidak ada client yang memanggil server lewat network. Tidak ada message contract antar service. Karena itu `.proto` tidak relevan untuk Exercise 1.

Jika `.proto` dipaksakan masuk Exercise 1, latihan akan menjadi campuran yang tidak natural: peserta sedang diminta memperbaiki race condition dan ownership, tetapi tiba-tiba juga harus memikirkan gRPC schema. Untuk quiz 50 menit, ini justru mengurangi fokus.

## Apa Kegunaan `.proto`?

`.proto` adalah file Protocol Buffers yang mendefinisikan kontrak gRPC secara typed. Isinya biasanya mencakup:

- `service`: daftar RPC yang tersedia di server;
- `rpc`: method yang bisa dipanggil client;
- `message`: struktur data request dan response;
- `stream`: penanda bahwa request atau response dikirim sebagai sequence, bukan satu data saja.

Contoh:

```proto
service QuizService {
  rpc SubmitPayment (PaymentRequest) returns (PaymentReply);
  rpc WatchTransactions (TransactionQuery) returns (stream TransactionEvent);
  rpc Chat (stream ChatMessage) returns (stream ChatMessage);
}
```

Maknanya:

- `SubmitPayment` adalah unary RPC: satu request, satu response.
- `WatchTransactions` adalah server streaming RPC: satu request, banyak response.
- `Chat` adalah bidirectional streaming RPC: client dan server sama-sama mengirim stream message.

## Mengapa `.proto` Penting di gRPC?

Tanpa `.proto`, client dan server tidak punya kontrak formal. Dengan `.proto`:

- interface service jelas;
- tipe request/response jelas;
- code bisa di-generate untuk banyak bahasa;
- perubahan contract lebih mudah dilacak;
- serialization memakai Protobuf binary format, biasanya lebih compact daripada JSON;
- streaming behavior bisa terlihat langsung dari signature RPC.

Di Rust Tonic, `.proto` diproses oleh `tonic-build` saat build. `build.rs` menjalankan compiler protobuf (`protoc`) untuk generate Rust code. Code generated ini kemudian dipakai oleh:

- server trait yang harus diimplementasikan;
- request/response struct;
- client/server wrapper.

## Dependency: `protoc`

Project gRPC lengkap membutuhkan `protoc`. Di mesin ini `protoc` sebelumnya belum tersedia, jadi dependency system di-install dengan:

```powershell
winget install --id Google.Protobuf --exact --accept-source-agreements --accept-package-agreements
```

Setelah install, terminal/IDE perlu restart agar PATH baru terbaca. Cek:

```powershell
protoc --version
```

## Mapping Project ke Materi

Exercise 1:

- Module 06 Concurrency.
- Fokus: shared memory model, message passing, Rust ownership.
- Tidak memakai `.proto` karena bukan gRPC/network service.

Exercise 2:

- Module 08 High Level Networking dan sebagian Module 07 Profiling.
- Fokus: REST vs gRPC, Protobuf, HTTP/2, streaming, Tonic/Tokio, dan reasoning profiling.
- Memakai `.proto` karena gRPC service wajib punya kontrak service/message.

Rangkuman:

- Menjembatani seluruh materi: Concurrency, Profiling, REST, gRPC, dan strategi essay.
