# Rangkuman Quiz Adpro: Concurrency, Profiling, dan gRPC

Target quiz dari `DESCRIPTION.txt`: satu programming problem Rust dalam waktu sekitar 50 menit, plus beberapa essay singkat. Materi utama: concurrency, profiling, dan gRPC. Dari pola quiz lama, soal biasanya tidak hanya meminta definisi, tetapi meminta kamu membaca situasi, memilih trade-off, memperbaiki code, lalu menjelaskan alasan teknisnya.

## Strategi 50 Menit

Alokasi yang realistis:

1. 5 menit: baca requirement, cari API yang harus dipertahankan, tandai fungsi yang `todo!()` atau blank.
2. 25-30 menit: selesaikan programming task sampai `cargo test` memberi progress.
3. 5-10 menit: rapikan edge case utama, terutama `workers == 0`, invalid input, clone/ownership, dan `join`.
4. 10 menit terakhir: jawab essay dengan format "definisi singkat, hubungkan ke project, sebut trade-off".

Saat stuck di Rust compiler, baca error dari atas. Error ownership biasanya menunjuk pada tiga hal: value moved, borrowed value does not live long enough, atau mutable borrow conflict. Jangan melawan borrow checker dengan random clone. Tentukan dulu siapa owner data dan apakah data perlu shared ownership atau message passing.

## 1. Rust Ownership dan Borrowing

Ownership adalah aturan Rust untuk memastikan memory safety tanpa garbage collector. Intinya:

- Setiap value punya satu owner.
- Saat owner keluar scope, value di-drop.
- Heap data seperti `String` dan `Vec<T>` akan move saat assignment atau dipassing by value.
- Tipe sederhana seperti `i32`, `bool`, `char` biasanya `Copy`, jadi tetap bisa dipakai setelah dipassing.

Contoh move:

```rust
fn takes_ownership(s: String) {
    println!("{s}");
}

fn main() {
    let name = String::from("Alya");
    takes_ownership(name);
    // println!("{name}"); // compile error: name sudah moved
}
```

Solusi jika hanya ingin membaca adalah borrowing:

```rust
fn len(s: &String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("Alya");
    let n = len(&name);
    println!("{name} has length {n}");
}
```

Mutable borrow:

```rust
fn add_suffix(s: &mut String) {
    s.push_str("-paid");
}

fn main() {
    let mut id = String::from("TX-001");
    add_suffix(&mut id);
}
```

Rule penting:

- Banyak immutable reference boleh: `&x`, `&x`, `&x`.
- Hanya satu mutable reference aktif: `&mut x`.
- Tidak boleh punya immutable reference aktif dan mutable reference aktif ke data yang sama pada waktu yang sama.
- Reference tidak boleh lebih lama hidup daripada data yang direferensikan.

Intuisi: immutable borrow itu seperti banyak orang membaca dokumen. Mutable borrow itu seperti satu orang sedang mengedit dokumen. Tidak boleh ada pembaca saat editor sedang mengedit karena hasil bacaan bisa tidak konsisten.

## 2. Thread dan Multi-threading

Process adalah program yang sedang berjalan dengan memory space sendiri. Thread adalah unit eksekusi di dalam process. Beberapa thread dalam process yang sama bisa berbagi memory.

Concurrency berarti beberapa pekerjaan disusun agar bisa berjalan overlap. Parallelism berarti beberapa pekerjaan benar-benar berjalan pada waktu yang sama, biasanya di core berbeda. Program concurrent belum tentu parallel, tetapi program parallel hampir pasti concurrent.

Rust membuat thread dengan `thread::spawn`:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("from worker");
    });

    println!("from main");
    handle.join().unwrap();
}
```

Jika thread memakai variable dari luar, biasanya perlu `move`:

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("{data:?}");
    });

    handle.join().unwrap();
}
```

Mengapa `move`? Karena thread bisa hidup lebih lama daripada function yang membuatnya. Rust tidak mau thread menyimpan reference ke variable lokal yang mungkin sudah di-drop. Dengan `move`, ownership atau handle data dipindahkan ke closure.

## 3. Shared Memory Model

Shared memory model berarti beberapa thread mengakses state yang sama. Masalah klasiknya:

- Critical section: bagian code yang mengakses shared data.
- Race condition: hasil bergantung pada urutan thread yang tidak deterministik.
- Lost update: dua update terjadi, tetapi salah satu tertimpa.
- Synchronization: mekanisme untuk mengatur akses, misalnya `Mutex`.
- Deadlock: thread saling menunggu lock sehingga tidak ada yang lanjut.

Contoh counter shared yang benar:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter = {}", *counter.lock().unwrap());
}
```

Peran `Arc<Mutex<T>>`:

- `Mutex<T>` memberi mutual exclusion agar hanya satu thread mengakses data pada satu waktu.
- `Arc<T>` memberi shared ownership yang thread-safe melalui atomic reference counting.
- `Arc::clone(&x)` tidak meng-copy data besar. Ia menambah reference count dan memberi handle baru ke data yang sama.
- `Rc<T>` bukan untuk multi-thread karena reference count-nya tidak atomic.

Kesalahan umum:

```rust
let counter = Mutex::new(0);
for _ in 0..10 {
    thread::spawn(move || {
        *counter.lock().unwrap() += 1;
    });
}
```

Ini salah karena `counter` moved ke thread pertama. Iterasi berikutnya tidak punya owner lagi. Gunakan `Arc`.

Tips deadlock:

- Jangan lock lebih lama dari perlu.
- Jangan melakukan operasi lambat saat lock masih dipegang.
- Jika harus lock dua resource, selalu lock dengan urutan yang konsisten.
- Hindari nested lock jika bisa.

## 4. Message Passing Model

Message passing berarti thread tidak berbagi state utama secara langsung, tetapi mengirim pesan melalui channel. Rust menyediakan `std::sync::mpsc`.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("done")).unwrap();
    });

    let message = rx.recv().unwrap();
    println!("{message}");
}
```

`mpsc` berarti multiple producer, single consumer. Banyak sender bisa mengirim pesan ke satu receiver.

Intuisi:

- Shared memory: "kita semua edit satu spreadsheet yang sama, harus pakai lock".
- Message passing: "worker kirim form hasil kerja ke admin, admin yang mengumpulkan".

Dalam quiz, kombinasi yang sering muncul adalah shared memory untuk state utama dan message passing untuk hasil. Contoh: inventory dilindungi `Arc<Mutex<_>>`, receipt dikirim lewat channel.

## 5. Pola Programming Rust yang Sering Keluar

Pattern memproses data dengan worker:

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn run(items: Vec<u32>, workers: usize) -> Vec<u32> {
    let workers = workers.max(1);
    let shared_total = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();
    let mut buckets = vec![Vec::new(); workers];

    for (i, item) in items.into_iter().enumerate() {
        buckets[i % workers].push(item);
    }

    let mut handles = vec![];
    for (worker_id, bucket) in buckets.into_iter().enumerate() {
        let shared_total = Arc::clone(&shared_total);
        let tx = tx.clone();

        handles.push(thread::spawn(move || {
            for item in bucket {
                {
                    let mut total = shared_total.lock().unwrap();
                    *total += item;
                }
                tx.send(worker_id as u32).unwrap();
            }
        }));
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    rx.into_iter().collect()
}
```

Hal yang harus dicek sebelum submit:

- Semua thread di-`join`.
- Sender utama di-`drop(tx)` agar receiver tidak menunggu selamanya.
- Lock tidak dipegang saat mengirim channel jika tidak perlu.
- `workers == 0` ditangani.
- Hasil test yang urutannya nondeterministic jangan diasumsikan urut kecuali kamu sort.

## 6. Profiling dan Performance

Performance testing dan profiling berbeda.

Performance testing:

- Melihat sistem dari luar.
- Mengukur response time, latency, throughput, error rate, dan behavior saat load tinggi.
- Tool contoh: JMeter.
- Cocok untuk menjawab "apakah sistem kuat menangani 1000 request per menit?"

Profiling:

- Melihat program dari dalam.
- Mencari function paling mahal, CPU hotspot, memory allocation, lock contention, atau I/O bottleneck.
- Cocok untuk menjawab "bagian code mana yang membuat request lambat?"

Kalimat quiz yang aman: performance testing menemukan gejala dan batas performa, profiling mencari penyebab teknis di level code/runtime.

## 7. JVM Ringkas untuk Materi Profiling

Walaupun programming quiz memakai Rust, modul profiling banyak memakai Java/JVM. Pahami konsepnya untuk essay atau multiple choice.

JVM membuat Java portable dan mengatur runtime:

- Class Loader Subsystem: memuat `.class` ke memory.
- Runtime Data Area: area memory saat program berjalan.
- Execution Engine: menjalankan bytecode.
- Interpreter: membaca bytecode dan menjalankan instruksi satu per satu.
- JIT Compiler: mengubah hot code menjadi native machine code saat runtime.
- Garbage Collector: membersihkan object yang tidak lagi reachable.
- JNI/Native Method Libraries: jembatan ke native code.

Runtime Data Area yang penting:

- Method Area: metadata class, method, static data.
- Heap: object dan array, shared antar thread, target GC.
- Stack: frame method per thread, local variable, call stack.
- PC Register: alamat instruksi yang sedang dieksekusi per thread.
- Native Method Stack: untuk native method.

JIT Compiler:

- Mengidentifikasi hot code.
- Compile bytecode menjadi native machine code.
- Melakukan optimization seperti inlining dan loop optimization.
- Trade-off: ada overhead kompilasi di awal, tetapi code yang sering jalan menjadi lebih cepat.

Garbage Collector:

- Mengelola memory otomatis.
- Mencari object yang tidak lagi reachable.
- Mengurangi manual memory management.
- Bisa menyebabkan pause atau overhead jika allocation tinggi.

## 8. JMeter dan Performance Testing

JMeter biasa dipakai untuk simulasi load. Konsep yang perlu dikenal:

- Test Plan: root dari skenario test.
- Thread Group: jumlah virtual user, ramp-up, loop.
- Sampler: request yang dikirim, misalnya HTTP Request.
- Listener: melihat hasil, misalnya Summary Report.
- Assertion: memastikan response benar, bukan hanya cepat.

Metric penting:

- Response time: waktu dari request sampai response.
- Throughput: jumlah request yang berhasil per satuan waktu.
- Error rate: persentase request gagal.
- Latency: waktu tunggu sebelum response mulai diterima.

Common trap: response cepat tetapi salah tetap gagal secara kualitas. Performance test harus tetap punya assertion.

## 9. Logging

Logging membantu observability. Logging yang baik menjawab "apa yang terjadi" tanpa harus attach debugger.

Best practice:

- Log event penting, bukan semua hal.
- Hindari log data sensitif seperti password, token, kartu, atau personal data yang tidak perlu.
- Gunakan level dengan benar: error, warn, info, debug, trace.
- Sertakan context seperti request id, user id, transaction id.
- Jangan membuat logging terlalu berat di hot path.

Logging bukan profiling. Log bisa membantu investigasi, tetapi profiler tetap diperlukan untuk melihat CPU/memory hotspot secara objektif.

## 10. Premature Optimization

Premature optimization adalah mengoptimasi sebelum ada bukti bottleneck. Ini berbahaya karena bisa membuat code lebih kompleks tanpa manfaat nyata.

Urutan yang sehat:

1. Pastikan program benar.
2. Ukur performa.
3. Cari bottleneck dengan profiling.
4. Optimasi bagian yang terbukti mahal.
5. Ukur lagi.

Dalam essay, jangan menjawab "langsung ubah algoritma" jika belum ada data. Jawaban yang lebih kuat adalah "reproduce dengan performance test, profile, baru optimasi".

## 11. REST API

REST adalah architectural style untuk API berbasis resource. REST populer karena sederhana, memakai HTTP yang umum, mudah dites, dan biasanya memakai JSON.

REST architectural constraints:

- Uniform Interface: resource diakses dengan pendekatan konsisten.
- Client-Server: client dan server dipisah.
- Stateless: setiap request berdiri sendiri, server tidak bergantung pada history request sebelumnya.
- Cacheable: response bisa ditandai cacheable jika cocok.
- Layered System: client tidak harus tahu apakah terhubung langsung ke server akhir atau lewat layer lain.
- Code on Demand: optional, server bisa mengirim executable code ke client.

HTTP methods:

| Method | Makna umum | Contoh |
| --- | --- | --- |
| GET | Ambil resource | `GET /orders/1` |
| POST | Buat resource/action baru | `POST /orders` |
| PUT | Replace resource secara penuh | `PUT /orders/1` |
| PATCH | Update sebagian resource | `PATCH /orders/1` |
| DELETE | Hapus resource | `DELETE /orders/1` |

Stateless trap: stateless bukan berarti server tidak punya database. Stateless berarti setiap request membawa informasi cukup untuk diproses tanpa bergantung pada session/history request sebelumnya.

## 12. JSON, XML, SOAP, REST

JSON:

- Ringan dan populer di web.
- Mudah dibaca manusia.
- Cocok untuk REST API modern.

XML:

- Tag-based.
- Lebih verbose.
- Banyak dipakai di sistem enterprise lama atau format dokumen.

SOAP:

- Protocol yang lebih standardized.
- Biasanya XML-based.
- Cocok untuk enterprise yang butuh contract dan standard ketat.

REST:

- Architectural style, bukan protocol tunggal.
- Biasanya HTTP + JSON.
- Lebih sederhana dan fleksibel daripada SOAP.

## 13. gRPC

gRPC adalah framework RPC modern. Client memanggil function pada server seolah-olah local method, tetapi komunikasi terjadi lewat network.

Komponen penting:

- `.proto`: definisi service dan message.
- Protobuf: format serialization binary dan schema.
- HTTP/2: transport modern untuk multiplexing dan streaming.
- Generated code: client stub dan server trait.
- Tonic: framework gRPC di Rust.
- Prost: Protobuf implementation di Rust.
- Tokio: async runtime untuk network I/O.

### Apa Itu `.proto`?

`.proto` adalah kontrak service dan data untuk gRPC. File ini menjawab pertanyaan:

- service apa yang tersedia;
- method apa yang bisa dipanggil;
- request dan response berisi field apa;
- komunikasi unary atau streaming.

Contoh di Exercise 2:

```proto
service QuizService {
  rpc SubmitPayment (PaymentRequest) returns (PaymentReply);
  rpc WatchTransactions (TransactionQuery) returns (stream TransactionEvent);
  rpc Chat (stream ChatMessage) returns (stream ChatMessage);
}
```

Di Rust, `tonic-build` membaca `.proto` saat build, memakai `protoc`, lalu generate Rust struct dan service trait. Karena itu `.proto` hanya relevan untuk project gRPC/network service. Exercise 1 tidak punya `.proto` karena Exercise 1 adalah concurrency project dalam satu process, bukan client-server RPC service.

Dependency untuk project gRPC:

```powershell
winget install --id Google.Protobuf --exact --accept-source-agreements --accept-package-agreements
protoc --version
```

Setelah install `protoc`, restart terminal atau IDE agar PATH terbaca.

Empat pola komunikasi gRPC:

1. Unary: satu request, satu response.
2. Server streaming: satu request, banyak response.
3. Client streaming: banyak request message, satu response.
4. Bidirectional streaming: client dan server sama-sama mengirim banyak message.

Contoh `.proto`:

```proto
syntax = "proto3";

package quizprep;

service PaymentService {
  rpc SubmitPayment (PaymentRequest) returns (PaymentReply);
  rpc WatchTransactions (TransactionQuery) returns (stream TransactionEvent);
}

message PaymentRequest {
  string payment_id = 1;
  string user_id = 2;
  uint32 amount = 3;
}

message PaymentReply {
  bool accepted = 1;
  string message = 2;
}
```

## 14. HTTP/2 dan Mengapa gRPC Cepat

Fitur HTTP/2 yang relevan:

- Multiplexing: banyak stream dalam satu koneksi.
- Streaming: data bisa dikirim bertahap.
- Header compression: mengurangi overhead header.
- Binary framing: komunikasi lebih efisien daripada text framing tradisional.

Protobuf juga membantu karena:

- Binary format lebih compact.
- Schema jelas.
- Generated type mengurangi parsing manual.
- Field memakai number tag, bukan nama field text berulang seperti JSON.

Tetapi gRPC bukan selalu lebih baik. REST masih unggul untuk public API sederhana, debugging manual via browser/curl, dan compatibility luas.

## 15. Tonic dan Tokio

Tonic adalah gRPC framework Rust. Tonic memakai async Rust dan berjalan di atas Tokio.

Intuisi async:

- Saat request menunggu network I/O, thread tidak harus idle.
- Runtime bisa menjalankan task lain.
- Cocok untuk service yang banyak menunggu socket, database, atau stream.

Common trap: async bukan otomatis lebih cepat untuk CPU-heavy work. Jika function menghitung sangat berat, async saja tidak menyelesaikan masalah CPU bottleneck. Gunakan profiling, optimasi algoritma, atau worker pool.

## 16. REST vs gRPC

Gunakan REST jika:

- API public untuk banyak client.
- Butuh mudah dites dengan browser, curl, Postman.
- Data sederhana.
- Streaming bukan kebutuhan utama.

Gunakan gRPC jika:

- Komunikasi antar microservice internal.
- Butuh schema contract kuat.
- Butuh streaming.
- Butuh efisiensi serialization.
- Banyak service multi-language yang bisa generate client/server code.

Trade-off penting:

- gRPC efisien, tetapi browser support tidak sesederhana REST.
- Protobuf kuat, tetapi tidak semudah JSON untuk dibaca manusia.
- REST fleksibel, tetapi contract bisa lebih longgar dan rawan mismatch jika dokumentasi buruk.

## 17. Template Jawaban Essay

Gunakan pola tiga bagian:

1. Definisi: jelaskan konsep dengan singkat.
2. Project link: sebut bagian code atau scenario.
3. Trade-off/consequence: jelaskan risiko atau alasan pilihan.

Contoh untuk shared memory vs message passing:

> Shared memory model membuat beberapa thread mengakses state yang sama, sehingga perlu synchronization seperti `Mutex` untuk mencegah race condition dan lost update. Message passing membuat thread berkomunikasi lewat channel, sehingga ownership data berpindah sebagai pesan dan shared state bisa dikurangi. Pada project inventory, `Arc<Mutex<HashMap<_>>>` adalah shared memory, sedangkan `mpsc::channel` untuk mengirim `Receipt` adalah message passing.

Contoh untuk profiling:

> Performance testing mengukur gejala dari luar seperti response time dan throughput, sedangkan profiling mencari penyebab dari dalam seperti CPU hotspot, allocation, atau lock contention. Jika `WatchTransactions` lambat, saya akan reproduce dengan load test, pastikan assertion response benar, lalu profile service untuk melihat apakah bottleneck ada di filtering data, lock, serialization protobuf, atau network streaming.

Contoh untuk gRPC:

> gRPC cocok untuk microservice karena service contract ditulis di `.proto`, lalu client dan server code bisa di-generate. Dibanding REST JSON, protobuf lebih compact dan HTTP/2 mendukung streaming/multiplexing. Trade-off-nya, REST lebih mudah dipakai untuk public API dan debugging manual, sedangkan gRPC lebih kuat untuk internal service-to-service communication.

## 18. Common Quiz Traps

- "Stateless" bukan berarti server tidak menyimpan data sama sekali.
- "Asynchronous programming" bukan protocol. WebSocket adalah protocol; async adalah programming model.
- gRPC streaming bukan karena protobuf didesain khusus untuk chunk, tetapi karena gRPC/HTTP/2 mendukung streaming.
- `Arc` bukan pengganti `Mutex`. `Arc` membagi ownership; `Mutex` melindungi mutation.
- `Mutex` bukan membuat program parallel lebih cepat; ia membuat akses shared data aman.
- `Rc` bukan untuk multi-thread.
- `move` closure bukan selalu copy data; ia memindahkan ownership atau handle.
- Performance testing bukan profiling.
- Profiling tanpa workload realistis bisa menyesatkan.
- Optimization sebelum pengukuran sering membuat code rumit tanpa memperbaiki bottleneck.

## 19. Checklist Sebelum Quiz

Pastikan kamu bisa menjelaskan dan menulis:

- `thread::spawn(move || { ... })`
- `handle.join().unwrap()`
- `Arc::new(Mutex::new(value))`
- `Arc::clone(&shared)`
- `let mut guard = shared.lock().unwrap();`
- `mpsc::channel`, `tx.send`, `rx.recv` atau `rx.into_iter`
- perbedaan `String` move vs `i32` copy
- rule mutable borrow
- REST method dan stateless
- gRPC unary vs streaming
- protobuf sebagai schema dan serialization
- Tonic memakai Tokio async runtime
- performance testing vs profiling

Jika kamu bisa mengerjakan dua exercise di folder ini tanpa melihat solution, fondasi kamu sudah kuat untuk model quiz project-based.

## 20. `src/lib.rs` vs `src/main.rs`

Dalam project Rust Cargo, `src/main.rs` adalah entry point binary application. Program mulai dari function:

```rust
fn main() {
    println!("hello");
}
```

`main.rs` cocok untuk orchestration: ambil input, panggil function, print output, atau menjalankan server.

`src/lib.rs` adalah root library crate. File ini cocok untuk menyimpan struct, enum, helper, dan business logic yang reusable dan mudah dites.

Contoh:

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```rust
// src/main.rs
use my_project::add;

fn main() {
    println!("{}", add(2, 3));
}
```

Nama import berasal dari `name` di `Cargo.toml`. Jika package name memakai hyphen, import Rust memakai underscore.

```toml
[package]
name = "exercise-3-testing"
```

```rust
use exercise_3_testing::checkout;
```

Integration tests di folder `tests/` juga memakai public API dari `lib.rs`:

```rust
use exercise_3_testing::checkout;
```

Karena itu item yang mau dipakai dari `main.rs` atau `tests/` harus diberi `pub` di `lib.rs`.

Rule praktis:

- Taruh logic di `lib.rs`.
- Taruh alur menjalankan program di `main.rs`.
- Taruh test behavior di folder `tests/`.
- Gunakan `use crate_name::item_name` untuk menghubungkan `main.rs` ke `lib.rs`.

Penjelasan lebih lengkap ada di `PENJELASAN_LIB_RS_DAN_MAIN_RS.md`.

## 21. Cara Memakai Paket Latihan Ini

Urutan belajar yang disarankan:

1. Baca `RANGKUMAN_QUIZ_ADPRO.md` sampai bagian concurrency dan gRPC.
2. Kerjakan `exercise-0/SOAL.md` sebagai pemanasan yang hanya mengisi `main`.
3. Kerjakan `exercise-1/SOAL.md` tanpa membuka solution.
4. Jalankan `cargo test`; jika gagal, baca error dan perbaiki.
5. Setelah selesai atau benar-benar stuck, baca `solution-exercise-1/WALKTHROUGH.md`, bukan langsung copy code.
6. Ulangi pola yang sama untuk `exercise-2/SOAL.md`.
7. Kerjakan `exercise-3-testing/SOAL.md` untuk melatih cara membuat tests.
8. Terakhir baca `ANSWERS.md` masing-masing solution dan coba tulis ulang jawaban essay dengan kata-katamu sendiri.

Untuk simulasi 50 menit, jangan buka `WALKTHROUGH.md` sebelum timer habis. Untuk belajar konsep, walkthrough justru penting karena menjelaskan kenapa suatu design dipilih.

Exercise 0 adalah yang paling cocok untuk pemanasan karena:

- hanya `src/main.rs` yang perlu diisi;
- tidak ada dependency external;
- tetap memakai `Arc<Mutex<_>>`, `mpsc`, `thread::spawn`, `Instant`, dan klasifikasi REST/gRPC;
- output dites lewat binary sehingga terasa seperti project kecil yang utuh.

## 22. Testing Project di Rust

Testing adalah cara mengubah requirement menjadi executable specification. Pada quiz project-based, test membantu kamu memahami apa yang harus benar sebelum refactor atau implementasi.

Di Rust ada dua bentuk umum:

- Unit test: biasanya ditulis di file yang sama dengan code, di dalam module `#[cfg(test)]`.
- Integration test: ditulis di folder `tests/`, memakai crate seperti external user.

Pola yang mudah dipakai adalah Arrange-Act-Assert:

```rust
#[test]
fn discount_more_than_100_percent_is_capped() {
    let subtotal = 80_000; // Arrange

    let discount = discount_amount(subtotal, 150); // Act

    assert_eq!(discount, 80_000); // Assert
}
```

Tips memilih test:

- Mulai dari happy path utama.
- Tambahkan edge case dari requirement, misalnya input kosong, batas threshold, nilai nol, atau nilai terlalu besar.
- Satu test sebaiknya fokus pada satu behavior.
- Nama test harus menjelaskan behavior, bukan hanya `test_1`.
- Jangan hanya mengejar coverage angka; pastikan test benar-benar memeriksa rule penting.
