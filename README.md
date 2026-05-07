# Quiz Prep Output

Folder ini dibuat dari prompt awal: latihan quiz project-based untuk Concurrency, Profiling, dan gRPC, dengan Bahasa Indonesia dan technical term tetap English.

## Isi Folder

- `exercise-0/`: starter project pemanasan yang cover concurrency, profiling sederhana, REST/gRPC classification, dan hanya perlu isi `src/main.rs`.
- `exercise-0/SOAL.md`: versi soal formal untuk simulasi quiz pemanasan.
- `solution-exercise-0/`: solusi lengkap Exercise 0, walkthrough, dan jawaban essay.
- `exercise-1/`: starter project Rust untuk concurrency, shared memory model, `Arc<Mutex<_>>`, `thread::spawn`, `move`, `join`, dan `mpsc`.
- `exercise-1/SOAL.md`: versi soal formal untuk simulasi quiz.
- `solution-exercise-1/`: solusi lengkap Exercise 1, walkthrough, dan jawaban essay.
- `exercise-2/`: starter project Rust gRPC dengan Tonic, Tokio, Prost, dan `.proto`.
- `exercise-2/SOAL.md`: versi soal formal untuk simulasi quiz.
- `solution-exercise-2/`: solusi lengkap Exercise 2, walkthrough, dan jawaban essay.
- `exercise-3-testing/`: starter project sederhana untuk latihan membuat tests.
- `solution-exercise-3-testing/`: solusi tests lengkap, walkthrough, dan jawaban essay.
- `RANGKUMAN_QUIZ_ADPRO.md`: rangkuman besar materi, tips, trik, code, dan strategi quiz.
- `PENJELASAN_PROTO_DAN_STRUKTUR_PROJECT.md`: penjelasan khusus tentang `.proto`, mengapa Exercise 1 tidak punya `.proto`, dan bagaimana dependency dipakai.
- `PENJELASAN_LIB_RS_DAN_MAIN_RS.md`: penjelasan khusus tentang `src/lib.rs`, `src/main.rs`, dan cara menghubungkannya.

## Dependency Utama

Semua project membutuhkan Rust/Cargo.

Project gRPC (`exercise-2` dan `solution-exercise-2`) juga membutuhkan `protoc`. Jika belum tersedia:

```powershell
winget install --id Google.Protobuf --exact --accept-source-agreements --accept-package-agreements
```

Restart terminal/IDE setelah install, lalu cek:

```powershell
protoc --version
```

## Expected Test Behavior

Starter project:

```powershell
cargo test --no-run
```

harus compile. Full `cargo test` akan gagal karena `todo!()` memang bagian yang harus dikerjakan peserta.

Solution project:

```powershell
cargo test
```

harus pass.

## Branch Plan untuk GitHub

Repository kosong akan dipush dengan branch terpisah:

- `main`: semua materi lengkap.
- `exercise-0`: hanya Exercise 0.
- `solution-exercise-0`: hanya Solution Exercise 0.
- `exercise-1`: hanya Exercise 1.
- `solution-exercise-1`: hanya Solution Exercise 1.
- `exercise-2-hard`: hanya Exercise 2.
- `solution-exercise-2`: hanya Solution Exercise 2.
- `exercise-3-testing`: hanya Exercise 3.
- `solution-exercise-3-testing`: hanya Solution Exercise 3.
- `study-guide`: rangkuman dan penjelasan umum.
