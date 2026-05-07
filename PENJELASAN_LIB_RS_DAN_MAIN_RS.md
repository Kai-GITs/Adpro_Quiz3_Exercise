# Penjelasan `src/lib.rs` dan `src/main.rs`

Dalam project Rust berbasis Cargo, dua file yang paling sering muncul adalah `src/lib.rs` dan `src/main.rs`.

## Apa Itu `src/main.rs`?

`src/main.rs` adalah entry point untuk binary application. Jika project punya `src/main.rs`, Cargo akan membuat executable program.

Function utama di file ini adalah:

```rust
fn main() {
    println!("Program mulai dari sini");
}
```

Gunakan `main.rs` untuk:

- membaca input;
- menjalankan alur program;
- memanggil function dari library;
- mencetak output;
- menjalankan server;
- melakukan orchestration.

Dalam exercise project-based, `main.rs` sering menjadi tempat peserta menghubungkan semua komponen.

## Apa Itu `src/lib.rs`?

`src/lib.rs` adalah root file untuk library crate. Jika project punya `src/lib.rs`, Cargo akan membuat library yang bisa dipakai oleh:

- `src/main.rs` dalam project yang sama;
- integration tests di folder `tests/`;
- project Rust lain jika crate dipublish atau dijadikan dependency.

Gunakan `lib.rs` untuk:

- mendefinisikan struct, enum, dan function reusable;
- menyimpan business logic;
- membuat public API;
- memisahkan logic dari entry point;
- membuat logic mudah dites.

Contoh:

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Keyword `pub` penting. Tanpa `pub`, function hanya private di library dan tidak bisa dipakai dari `main.rs` atau integration tests.

## Bagaimana `main.rs` Menggunakan `lib.rs`?

Nama library crate berasal dari `name` di `Cargo.toml`. Jika `Cargo.toml` berisi:

```toml
[package]
name = "exercise-0"
```

maka di Rust code nama crate dipakai dengan underscore:

```rust
use exercise_0::add;
```

Rust mengubah hyphen `-` menjadi underscore `_` untuk import path.

Contoh lengkap:

```rust
// Cargo.toml
[package]
name = "mini-project"
```

```rust
// src/lib.rs
pub fn greeting(name: &str) -> String {
    format!("Hello, {name}")
}
```

```rust
// src/main.rs
use mini_project::greeting;

fn main() {
    let message = greeting("Alya");
    println!("{message}");
}
```

Karena package name `mini-project`, import path-nya menjadi `mini_project`.

## Bagaimana Tests Menggunakan `lib.rs`?

Integration tests di folder `tests/` juga memakai library crate seperti external user.

```rust
// tests/greeting_tests.rs
use mini_project::greeting;

#[test]
fn greeting_contains_name() {
    assert_eq!(greeting("Alya"), "Hello, Alya");
}
```

Itulah alasan logic sebaiknya diletakkan di `lib.rs`: agar bisa dipanggil oleh `main.rs` dan tests.

## Pola yang Dipakai di Exercise Ini

Sebagian exercise memakai pola:

- `src/lib.rs`: tipe data, helper, dan logic reusable.
- `src/main.rs`: tempat peserta menghubungkan helper dan menjalankan program.
- `tests/`: memverifikasi output atau behavior melalui public API.

Contoh Exercise 0:

```rust
// src/main.rs
use exercise_0::{format_report, sample_logs, Summary};
```

Artinya `main.rs` mengambil function dan type public dari `lib.rs`.

Contoh Exercise 3:

```rust
// src/main.rs
use exercise_3_testing::{checkout, CartItem};
```

Artinya binary memakai checkout logic yang didefinisikan di library.

## Kenapa Tidak Semua Code Ditulis di `main.rs`?

Bisa saja semua code ditulis di `main.rs`, tetapi itu kurang baik untuk project yang perlu dites.

Jika semua logic ada di `main.rs`:

- integration test lebih sulit memanggil function langsung;
- business logic bercampur dengan printing/input/output;
- code lebih susah direuse;
- debugging lebih berantakan.

Pola yang lebih sehat:

- simpan logic di `lib.rs`;
- panggil logic dari `main.rs`;
- test logic lewat `tests/`.

## Checklist Saat Menghubungkan `lib.rs` dan `main.rs`

- Pastikan function/type di `lib.rs` diberi `pub` jika mau dipakai di luar.
- Cek package name di `Cargo.toml`.
- Ganti hyphen `-` menjadi underscore `_` saat import.
- Pakai `use crate_name::NamaType` atau `use crate_name::nama_function`.
- Jalankan `cargo test` untuk memastikan library dan binary sama-sama compile.

## Common Error

Jika muncul error seperti:

```text
unresolved import `exercise_0`
```

cek:

- apakah package name benar;
- apakah hyphen sudah diganti underscore;
- apakah file `src/lib.rs` ada;
- apakah item yang di-import diberi `pub`.

Jika muncul error:

```text
function `checkout` is private
```

berarti function ada, tetapi belum diberi `pub` di `lib.rs`.
