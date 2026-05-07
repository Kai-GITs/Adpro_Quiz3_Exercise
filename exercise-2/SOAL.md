# SOAL - Exercise 2

## Judul

Mini gRPC Payment Service

## Durasi Simulasi

70-90 menit untuk latihan penuh. Jika ingin simulasi quiz 50 menit, fokus kerjakan `submit_payment_logic`, `transaction_events`, dan essay 1-3 terlebih dahulu.

## Latar Belakang

Sebuah sistem microservice payment memakai gRPC untuk komunikasi internal. Service ini punya tiga pola komunikasi:

- unary untuk submit payment;
- server streaming untuk transaction history;
- bidirectional streaming untuk chat acknowledgement.

Project ini melatih pemahaman gRPC, protobuf, Tonic, Tokio, dan reasoning profiling.

## Dependency

Pastikan `protoc` tersedia:

```powershell
protoc --version
```

Jika belum ada:

```powershell
winget install --id Google.Protobuf --exact --accept-source-agreements --accept-package-agreements
```

Restart terminal/IDE setelah install.

## File yang Penting

- `proto/quiz_service.proto`: kontrak gRPC.
- `build.rs`: menjalankan `tonic-build` untuk generate Rust code dari `.proto`.
- `src/lib.rs`: implementasi service logic.
- `tests/grpc_logic_tests.rs`: acceptance tests.

## Programming Problem

Lengkapi tiga function di `src/lib.rs`:

```rust
pub fn submit_payment_logic(&self, request: PaymentRequest) -> PaymentReply
pub fn transaction_events(&self, query: TransactionQuery) -> Vec<TransactionEvent>
pub fn chat_responses(&self, messages: Vec<ChatMessage>) -> Vec<ChatMessage>
```

Requirement detail:

1. `submit_payment_logic`
   - Valid jika `payment_id` tidak kosong, `user_id` tidak kosong, dan `amount > 0`.
   - Invalid request menghasilkan `accepted = false` dan `message = "invalid payment"`.
   - Valid request menghasilkan `accepted = true` dan `message = "payment accepted"`.
   - Valid request menambah `TransactionEvent` dengan `transaction_id = "TX-<payment_id>"`, `status = "SETTLED"`.
   - Invalid request tidak boleh menambah transaction event.

2. `transaction_events`
   - Filter berdasarkan `user_id`.
   - `limit == 0` berarti ambil semua event user tersebut.
   - `limit > 0` berarti ambil maksimal sejumlah limit.
   - Unknown user menghasilkan vector kosong.

3. `chat_responses`
   - Setiap inbound message menghasilkan satu response.
   - Sender response selalu `"server"`.
   - Empty body menghasilkan `NACK <sender>: empty message`.
   - Non-empty body menghasilkan `ACK <sender>: <body>`.

## Essay Questions

1. Mengapa gRPC cocok untuk komunikasi antar microservice dibanding REST pada kasus yang butuh streaming dan kontrak schema kuat?
2. Jelaskan peran protobuf dalam gRPC. Mengapa protobuf sering lebih efisien daripada JSON?
3. Jelaskan hubungan HTTP/2, multiplexing, dan streaming dalam gRPC.
4. Dalam konteks Tonic, mengapa `tokio` dan `async/await` penting untuk performa service?
5. Bedakan performance testing dan profiling. Jika endpoint `WatchTransactions` terasa lambat, langkah investigasi apa yang akan kamu lakukan?
6. Jelaskan mengapa `SubmitPayment` cocok sebagai unary RPC, sedangkan `WatchTransactions` cocok sebagai server streaming RPC.

## Rubrik Mandiri

- 25%: memahami `.proto`, generated code, dan gRPC communication pattern.
- 30%: service logic benar untuk payment, transaction filter, dan chat response.
- 15%: shared state aman dan lock dipakai singkat.
- 15%: tests pass.
- 15%: essay menjawab trade-off REST/gRPC/profiling secara konkret.

## Hints Bertahap

Hint 1: Jangan edit generated code. Edit `.proto` hanya jika soal meminta perubahan interface.

Hint 2: Tests memanggil logic helper secara langsung supaya kamu bisa debug tanpa menjalankan server.

Hint 3: `tonic::include_proto!("quizprep")` mengambil hasil generated code dari package `quizprep` di `.proto`.

Hint 4: Untuk transaction list, clone event sebelum keluar dari lock. Jangan return reference ke data di dalam `Mutex`.
