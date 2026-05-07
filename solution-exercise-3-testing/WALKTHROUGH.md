# Walkthrough Solusi - Exercise 3

Tujuan exercise ini adalah belajar menulis test yang baik untuk project sederhana.

## 1. Baca Public API

Jangan mulai dari implementasi internal. Lihat dulu function public:

```rust
calculate_subtotal
discount_amount
shipping_fee
checkout
```

Karena test ada di folder `tests/`, test hanya memakai public API. Ini bagus karena test menjadi representasi cara user memakai crate.

Public API itu berasal dari `src/lib.rs`. Binary demo di `src/main.rs` juga memakai API yang sama:

```rust
use solution_exercise_3_testing::{checkout, CartItem};
```

Nama package `solution-exercise-3-testing` berubah menjadi import path `solution_exercise_3_testing`.

## 2. Ubah Requirement Menjadi Test Case

Business rules di `SOAL.md` bisa langsung diterjemahkan:

- subtotal menjumlahkan semua item;
- quantity nol tidak menambah subtotal;
- discount normal dihitung dari subtotal;
- discount lebih dari 100 di-cap;
- shipping gratis untuk member;
- checkout summary menghitung semua field.

Setiap bullet idealnya jadi satu test agar kegagalannya mudah dibaca.

## 3. Buat Helper Test

Helper membuat test lebih ringkas:

```rust
fn item(sku: &str, unit_price: u32, qty: u32) -> CartItem {
    CartItem {
        sku: sku.to_string(),
        unit_price,
        qty,
    }
}
```

Helper ini bukan production code. Ia hanya mengurangi noise di tests.

## 4. Pakai Arrange-Act-Assert

Contoh:

```rust
#[test]
fn discount_more_than_100_percent_is_capped() {
    let subtotal = 80_000; // Arrange

    let discount = discount_amount(subtotal, 150); // Act

    assert_eq!(discount, 80_000); // Assert
}
```

Test ini fokus pada satu behavior: cap discount.

## 5. Test Summary End-to-End

Selain function kecil, perlu test `checkout` karena function ini menggabungkan subtotal, discount, shipping, dan total.

Contoh perhitungan:

- item: `40_000 * 2 + 5_000 * 3 = 95_000`;
- discount 10% = `9_500`;
- after discount = `85_500`;
- bukan member dan belum mencapai `100_000`, shipping = `15_000`;
- total = `100_500`.

Test ini membuktikan fungsi gabungan mengikuti business rule.

## 6. Kenapa Tidak Membuat Terlalu Banyak Test?

Latihan ini sengaja sederhana. Test yang baik bukan berarti sebanyak mungkin, tetapi cukup untuk menutup behavior penting dan edge case. Jika test terlalu banyak tetapi duplikatif, maintenance menjadi lebih berat tanpa banyak manfaat.

## 7. Kesalahan yang Sering Terjadi

- Test hanya memanggil function tanpa assertion.
- Test menggabungkan terlalu banyak behavior sehingga sulit tahu penyebab gagal.
- Nama test terlalu umum seperti `test_checkout`.
- Tidak mengetes edge case.
- Mengubah implementation agar test yang lemah pass, bukan menulis test sesuai requirement.

## 8. Cara Memilih Test yang Baik

Mulai dari requirement, bukan dari code internal. Untuk setiap rule, tanyakan:

- input apa yang mewakili rule ini?
- output apa yang harus benar?
- edge case apa yang bisa membuat rule salah?

Contoh rule `discount_percent > 100 diperlakukan sebagai 100` langsung menjadi test:

```rust
let discount = discount_amount(80_000, 150);
assert_eq!(discount, 80_000);
```

Test ini kecil, jelas, dan jika gagal penyebabnya mudah dicari.

## 9. Unit Test atau Integration Test?

Karena file test berada di folder `tests/`, test ini adalah integration test. Ia memakai crate seperti user eksternal:

```rust
use solution_exercise_3_testing::checkout;
```

Keuntungan integration test:

- memastikan public API nyaman dipakai;
- tidak bergantung pada private helper;
- cocok untuk project-based quiz karena menilai behavior dari luar.

Unit test tetap berguna jika kamu ingin mengetes private helper atau logic kecil di module yang sama.

## 10. Cara Debug Jika Test Gagal

Jika subtotal salah:

- cek perkalian `unit_price * qty`;
- cek semua item dijumlahkan;
- cek item `qty == 0`.

Jika discount salah:

- cek integer division;
- cek cap `min(100)`;
- cek discount dihitung dari subtotal, bukan after shipping.

Jika shipping salah:

- cek member selalu gratis;
- cek threshold `>= 100_000`, bukan `> 100_000`;
- cek non-member di bawah threshold dikenai `15_000`.

Jika checkout summary salah:

- tulis manual perhitungan di komentar test;
- cek urutan: subtotal, discount, after discount, shipping, total.

## 11. Anti-pattern dalam Test

Hindari test seperti ini:

```rust
#[test]
fn test_checkout() {
    checkout(&[], 0, false);
}
```

Test ini tidak punya assertion, sehingga hampir tidak membuktikan apa pun.

Hindari juga assertion yang terlalu lemah:

```rust
assert!(summary.total > 0);
```

Lebih baik cek angka yang memang ditentukan requirement:

```rust
assert_eq!(summary.total, 100_500);
```

## 12. Variasi Latihan Setelah Pass

1. Tambah test empty cart.
2. Tambah test checkout member dengan discount.
3. Tambah test subtotal besar untuk memastikan tidak panic.
4. Tambah table-driven test untuk beberapa discount percentage.

Variasi ini membantu kamu berpikir seperti tester: bukan hanya membuat code pass, tetapi menjaga behavior tetap benar saat code berubah.

## 13. Checklist Sebelum Submit Quiz

- Semua test punya assertion.
- Nama test menjelaskan behavior.
- Ada happy path dan edge case.
- Test tidak bergantung pada private implementation.
- `cargo test` pass.
- Essay bisa menjelaskan Arrange-Act-Assert dengan contoh dari test sendiri.
