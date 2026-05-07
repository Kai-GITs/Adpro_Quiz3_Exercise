# EXERCISE 3 - Testing a Small Checkout Project

Exercise ini fokus pada kemampuan membuat test untuk suatu project. Soalnya sengaja sederhana agar kamu berlatih membaca requirement, menentukan edge case, lalu menulis test yang jelas.

Versi soal formal tersedia di [SOAL.md](SOAL.md).

## Cara Mengecek

```powershell
cargo test
```

Starter project ini compile, tetapi test awal gagal karena ada `todo!()` di `tests/checkout_tests.rs`. Tugas kamu adalah mengganti placeholder itu dengan test yang benar.

## Target Belajar

- Membaca public API di `src/lib.rs`.
- Menulis integration test di folder `tests/`.
- Memakai pola Arrange-Act-Assert.
- Memilih edge case penting tanpa membuat test terlalu rumit.
- Memastikan test memeriksa behavior, bukan detail implementasi internal.

## Catatan `lib.rs` dan `main.rs`

`src/lib.rs` berisi checkout logic yang harus dites. `src/main.rs` hanya contoh kecil cara memakai library tersebut sebagai binary program.

Tests di folder `tests/` memakai public API dari `lib.rs` dengan:

```rust
use exercise_3_testing::CartItem;
```

Package name di `Cargo.toml` adalah `exercise-3-testing`, tetapi import path Rust menjadi `exercise_3_testing`.
