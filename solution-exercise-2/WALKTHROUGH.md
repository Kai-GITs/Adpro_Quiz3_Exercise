# Walkthrough Solusi - Exercise 2

Dokumen ini menjelaskan cara menyelesaikan Mini gRPC Payment Service secara bertahap.

## 1. Pahami Peran `.proto`

File `proto/quiz_service.proto` mendefinisikan kontrak:

```proto
service QuizService {
  rpc SubmitPayment (PaymentRequest) returns (PaymentReply);
  rpc WatchTransactions (TransactionQuery) returns (stream TransactionEvent);
  rpc Chat (stream ChatMessage) returns (stream ChatMessage);
}
```

Maknanya:

- `SubmitPayment`: unary, satu payment request dan satu payment reply.
- `WatchTransactions`: server streaming, client minta history lalu server mengirim banyak event.
- `Chat`: bidirectional streaming, client dan server sama-sama mengirim sequence message.

Saat `cargo test`, `build.rs` memanggil `tonic-build`. `tonic-build` memakai `protoc` untuk generate Rust struct dan trait. Karena itu dependency `protoc` wajib tersedia.

## 2. Kenapa Tests Memanggil Logic Helper?

Real gRPC server bisa dites lewat network, tetapi untuk quiz 50 menit itu terlalu banyak setup. Project ini tetap punya server Tonic, tetapi tests memanggil function helper:

```rust
submit_payment_logic
transaction_events
chat_responses
```

Keuntungannya:

- feedback dari `cargo test` cepat;
- logic tetap sama dengan yang dipakai RPC handler;
- peserta tetap belajar `.proto` dan Tonic structure tanpa harus membuat client manual.

## 3. Shared State Transaction

Service menyimpan transaction event:

```rust
transactions: Arc<Mutex<Vec<TransactionEvent>>>
```

Alasannya:

- Tonic service bisa di-clone dan dipakai oleh banyak request.
- `Arc` memberi shared ownership.
- `Mutex` melindungi vector saat ada payment baru yang menambah event.

Untuk production, desain bisa memakai database async atau `tokio::sync::Mutex`, tetapi untuk latihan materi concurrency, `Arc<Mutex<_>>` sengaja dibuat eksplisit.

## 4. Implementasi `submit_payment_logic`

Validasi dulu:

```rust
let valid = !request.payment_id.trim().is_empty()
    && !request.user_id.trim().is_empty()
    && request.amount > 0;
```

Jika invalid, return tanpa menyentuh transaction list:

```rust
if !valid {
    return PaymentReply {
        payment_id: request.payment_id,
        accepted: false,
        message: "invalid payment".to_string(),
    };
}
```

Jika valid, buat event:

```rust
let event = TransactionEvent {
    transaction_id: format!("TX-{}", request.payment_id),
    user_id: request.user_id,
    amount: request.amount,
    status: "SETTLED".to_string(),
};
```

Lalu lock sebentar hanya untuk push:

```rust
self.transactions.lock().unwrap().push(event);
```

Terakhir return accepted reply.

## 5. Implementasi `transaction_events`

Ambil limit:

```rust
let limit = query.limit as usize;
```

Lock transaction list:

```rust
let events = self.transactions.lock().unwrap();
```

Filter dan clone:

```rust
let filtered = events
    .iter()
    .filter(|event| event.user_id == query.user_id)
    .cloned();
```

Kenapa `cloned()`? Karena function harus return `Vec<TransactionEvent>`, bukan reference ke isi `Mutex`. Reference ke data di dalam lock tidak boleh keluar setelah lock guard drop.

Limit rule:

```rust
if limit == 0 {
    filtered.collect()
} else {
    filtered.take(limit).collect()
}
```

## 6. Implementasi `chat_responses`

Untuk setiap inbound message, buat response dari server:

```rust
messages
    .into_iter()
    .map(|message| self.chat_response(message))
    .collect()
```

Helper:

```rust
fn chat_response(&self, message: ChatMessage) -> ChatMessage {
    let body = if message.body.trim().is_empty() {
        format!("NACK {}: empty message", message.sender)
    } else {
        format!("ACK {}: {}", message.sender, message.body)
    };

    ChatMessage {
        sender: "server".to_string(),
        body,
    }
}
```

`trim().is_empty()` membuat `"   "` dianggap empty message.

## 7. Hubungan Logic Helper dengan RPC Handler

Unary handler:

```rust
async fn submit_payment(&self, request: Request<PaymentRequest>) -> Result<Response<PaymentReply>, Status> {
    let reply = self.submit_payment_logic(request.into_inner());
    Ok(Response::new(reply))
}
```

Server streaming handler:

```rust
let stream = tokio_stream::iter(
    self.transaction_events(request.into_inner())
        .into_iter()
        .map(Ok),
);
```

Bidirectional streaming handler membaca semua inbound message, lalu mengubah responses menjadi stream.

## 8. Kenapa Solusi Ini Sesuai Materi?

- gRPC contract ada di `.proto`.
- Protobuf message menjadi Rust type via generated code.
- Tonic memakai `async` handler dan `tokio` runtime.
- `WatchTransactions` menunjukkan server streaming.
- `Chat` menunjukkan bidirectional streaming.
- Shared state dilindungi `Mutex`, sehingga masih mengikat ke materi concurrency.
- Essay profiling bisa dikaitkan ke `WatchTransactions`: jika lambat, ukur dengan performance testing lalu profile filtering, lock contention, serialization, atau network stream.

## 9. Kesalahan yang Sering Terjadi

- Mengedit generated code, bukan `.proto` atau `src/lib.rs`.
- Lupa install `protoc`.
- Menganggap `.proto` hanya dokumentasi, padahal ia source contract untuk generated code.
- Return reference ke data di dalam `Mutex`.
- Menambah transaction event meskipun payment invalid.
- Menganggap async otomatis menyelesaikan CPU bottleneck.

## 10. Cara Debug Jika Build Gagal

Jika error menyebut `protoc` tidak ditemukan:

1. Jalankan `protoc --version`.
2. Jika command tidak ditemukan, install `Google.Protobuf`.
3. Restart terminal atau IDE agar PATH baru terbaca.

Jika error menyebut package proto tidak ditemukan:

- pastikan `package quizprep;` di `.proto` sama dengan `tonic::include_proto!("quizprep")`;
- pastikan `build.rs` menunjuk file `proto/quiz_service.proto`;
- jalankan `cargo clean` lalu `cargo test` jika generated cache terasa stale.

Jika type generated tidak ditemukan:

- cek nama message di `.proto`;
- cek hasil `pub use quizprep::{...}` di `src/lib.rs`;
- ingat bahwa Rust generated type mengikuti nama message Protobuf.

## 11. Cara Debug Jika Test Gagal

Jika valid payment tidak tercatat:

- cek apakah event dipush ke `transactions`;
- cek format `transaction_id` harus `TX-<payment_id>`;
- cek status harus `SETTLED`.

Jika invalid payment tetap menambah event:

- pastikan return invalid terjadi sebelum push event;
- validasi `amount > 0`, `payment_id` tidak kosong, dan `user_id` tidak kosong.

Jika transaction filter salah:

- cek filter menggunakan `event.user_id == query.user_id`;
- cek `limit == 0` berarti semua, bukan nol hasil;
- cek `limit > 0` memakai `.take(limit)`.

Jika chat test gagal:

- sender response harus `"server"`;
- empty body memakai `trim().is_empty()`;
- format string harus persis `ACK <sender>: <body>` atau `NACK <sender>: empty message`.

## 12. Mengapa Helper Logic Tetap Valid untuk gRPC

Handler gRPC di Tonic hanya wrapper:

- unwrap request dari `tonic::Request`;
- panggil logic helper;
- bungkus hasil ke `tonic::Response`.

Karena logic helper dipakai oleh handler, test helper tetap menguji behavior utama service. Ini strategi yang bagus untuk quiz karena menghindari setup network yang tidak perlu, tetapi tetap mempertahankan struktur gRPC asli.

## 13. Profiling Reasoning untuk Exercise Ini

Jika `WatchTransactions` lambat, jangan langsung optimasi. Urutannya:

1. Performance testing: ukur latency dan throughput dengan data yang cukup besar.
2. Profiling: cek apakah waktu habis di lock, filtering vector, serialization Protobuf, atau network stream.
3. Perbaiki bottleneck yang terbukti.

Kemungkinan improvement:

- index transaction by `user_id`;
- pindahkan data ke database;
- hindari lock terlalu lama;
- batasi streaming result;
- gunakan pagination atau backpressure strategy.

## 14. Variasi Latihan Setelah Pass

1. Tambah client streaming RPC untuk upload batch payment.
2. Tambah field `currency` di `.proto`, lalu update tests.
3. Tambah `Status::invalid_argument` untuk request invalid di handler, bukan reply biasa.
4. Buat test async yang menjalankan Tonic server lokal dan client sungguhan.

Variasi ini sengaja tidak masuk starter agar Exercise 2 tetap bisa dikerjakan sebagai latihan quiz, bukan project besar.

## 15. Checklist Sebelum Submit Quiz

- `protoc --version` jalan.
- `cargo test` pass.
- `.proto` tidak rusak dan package cocok dengan `include_proto!`.
- Tidak mengedit generated code.
- Invalid payment tidak membuat event.
- `limit == 0` berarti unlimited.
- Essay menyebut Protobuf, HTTP/2, streaming, Tonic, Tokio, dan trade-off REST/gRPC.
