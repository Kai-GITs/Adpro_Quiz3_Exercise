# Jawaban Essay - Exercise 3

## 1. Unit Test vs Integration Test

Unit test biasanya berada dekat dengan code yang dites, misalnya di dalam module `src/lib.rs` dengan `#[cfg(test)]`. Unit test cocok untuk menguji bagian kecil secara langsung.

Integration test di Rust biasanya berada di folder `tests/`. Test di folder ini memperlakukan crate seperti external user, sehingga hanya bisa mengakses public API. `tests/checkout_tests.rs` termasuk integration test karena menguji behavior library dari sisi pengguna crate.

## 2. Arrange-Act-Assert

Arrange berarti menyiapkan input dan kondisi awal. Act berarti memanggil function yang sedang dites. Assert berarti memeriksa hasil.

Contoh: pada test `checkout_returns_complete_summary`, Arrange-nya adalah membuat beberapa `CartItem`, Act-nya memanggil `checkout(&items, 10, false)`, dan Assert-nya memeriksa `subtotal`, `discount`, `shipping_fee`, dan `total`.

## 3. Pentingnya Edge Case

Edge case penting karena bug sering muncul di batas aturan, bukan hanya happy path. `qty == 0` perlu dites agar item kosong tidak sengaja menambah subtotal. `discount_percent > 100` perlu dites agar total tidak menjadi aneh karena discount lebih besar dari subtotal. Test edge case membuat requirement lebih eksplisit dan mencegah regression.
