# SOAL - Exercise 3

## Judul

Testing a Small Checkout Project

## Durasi Simulasi

30-40 menit.

## Latar Belakang

Kamu diberi library checkout sederhana untuk toko online. Implementasi sudah tersedia di `src/lib.rs`. Tugasmu bukan menulis ulang business logic, tetapi membuat test yang membuktikan behavior penting sudah benar.

Latihan ini dibuat karena pada quiz project-based, test sering menjadi cara tercepat memahami requirement dan memastikan solusi tidak merusak behavior.

## File yang Boleh Diubah

Fokus ubah:

- `tests/checkout_tests.rs`

Jangan ubah `src/lib.rs` kecuali kamu menemukan bug yang benar-benar terbukti dari test.

## Public API

Library menyediakan:

```rust
pub struct CartItem {
    pub sku: String,
    pub unit_price: u32,
    pub qty: u32,
}

pub struct CheckoutSummary {
    pub subtotal: u32,
    pub discount: u32,
    pub shipping_fee: u32,
    pub total: u32,
}

pub fn calculate_subtotal(items: &[CartItem]) -> u32
pub fn discount_amount(subtotal: u32, discount_percent: u32) -> u32
pub fn shipping_fee(after_discount: u32, is_member: bool) -> u32
pub fn checkout(items: &[CartItem], discount_percent: u32, is_member: bool) -> CheckoutSummary
```

## Business Rules

- `subtotal` adalah jumlah `unit_price * qty` untuk semua item.
- Item dengan `qty == 0` tidak menambah subtotal.
- `discount_percent` dihitung dari subtotal.
- `discount_percent > 100` diperlakukan sebagai 100.
- `shipping_fee` gratis jika user adalah member.
- `shipping_fee` gratis jika total setelah discount minimal `100_000`.
- Selain itu, `shipping_fee` adalah `15_000`.
- `total = subtotal - discount + shipping_fee`.

## Programming Task

Ganti placeholder test di `tests/checkout_tests.rs`.

Minimal buat 6 test:

1. subtotal menjumlahkan beberapa item;
2. item quantity nol tidak menambah subtotal;
3. discount normal dihitung benar;
4. discount lebih dari 100% di-cap menjadi 100%;
5. shipping gratis untuk member;
6. `checkout` menghasilkan `CheckoutSummary` lengkap yang benar.

## Essay Questions

1. Jelaskan perbedaan unit test dan integration test dalam Rust. Test di folder `tests/` termasuk yang mana?
2. Jelaskan pola Arrange-Act-Assert dan berikan contoh dari salah satu test yang kamu buat.
3. Mengapa edge case seperti `qty == 0` dan `discount_percent > 100` penting dites?

## Rubrik Mandiri

- 40%: test mencakup business rules utama.
- 25%: test punya nama jelas dan satu fokus behavior per test.
- 20%: assertion memeriksa output penting, bukan detail internal.
- 15%: essay mengaitkan konsep testing dengan project.

## Hints Bertahap

Hint 1: Buat helper function kecil di test untuk membuat `CartItem`.

Hint 2: Jangan hanya test happy path. Ambil minimal dua edge case.

Hint 3: Test yang baik akan tetap berguna walaupun implementasi internal berubah.
