# Walkthrough Solusi - Exercise 1

Dokumen ini menjelaskan cara menyelesaikan programming task dari nol. Tujuannya bukan menghafal final code, tetapi memahami urutan keputusan yang aman untuk Rust concurrency.

## 1. Baca Tests Dulu

Tests memberi tahu behavior yang wajib:

- valid request mengurangi inventory;
- `qty == 0` ditolak sebagai `RejectedInvalidQuantity`;
- SKU tidak ada atau stock kurang ditolak sebagai `RejectedOutOfStock`;
- banyak thread tidak boleh oversell;
- `workers == 0` tetap harus jalan sebagai 1 worker;
- empty request tidak mengubah inventory.

Kesimpulan: masalah utama bukan hanya loop request, tetapi menjaga inventory bersama agar update atomic secara logika.

## 2. Tentukan Shared State

Inventory harus dibaca dan diubah oleh banyak worker. Bentuk yang dipakai:

```rust
let inventory = Arc::new(Mutex::new(initial_inventory));
```

Alasannya:

- `Mutex` menjaga critical section saat cek stock dan kurangi stock.
- `Arc` membuat banyak worker punya handle ke inventory yang sama.
- `Arc::clone(&inventory)` hanya clone handle, bukan clone seluruh `HashMap`.

## 3. Tentukan Jalur Hasil

Requirement meminta `mpsc::channel`, jadi receipt tidak disimpan ke shared vector. Worker mengirim hasil:

```rust
let (tx, rx) = mpsc::channel();
```

Setiap worker menerima clone sender:

```rust
let tx = tx.clone();
```

Main thread nanti membaca dari `rx`.

## 4. Bagi Request ke Worker

Worker count harus minimal 1:

```rust
let worker_count = workers.max(1);
```

Lalu request dibagi round-robin:

```rust
let mut partitions = vec![Vec::new(); worker_count];
for (idx, request) in requests.into_iter().enumerate() {
    partitions[idx % worker_count].push(request);
}
```

Round-robin cukup untuk latihan karena semua request diproses tepat sekali dan distribusinya sederhana.

## 5. Spawn Worker dengan `move`

Setiap worker butuh ownership atas bucket request, sender, dan handle inventory:

```rust
for (worker_id, partition) in partitions.into_iter().enumerate() {
    let inventory = Arc::clone(&inventory);
    let tx = tx.clone();

    let handle = thread::spawn(move || {
        for request in partition {
            let receipt = process_one(worker_id, request, &inventory);
            tx.send(receipt).unwrap();
        }
    });
}
```

Keyword `move` penting karena thread bisa hidup lebih lama daripada scope function. Dengan `move`, closure membawa ownership data yang dibutuhkan.

## 6. Implementasi `process_one`

Urutan paling aman:

1. Tangani `qty == 0` sebelum lock inventory.
2. Lock inventory.
3. Ambil stock untuk SKU.
4. Jika cukup, kurangi stock dan return `Fulfilled`.
5. Jika tidak cukup, return `RejectedOutOfStock`.

Bagian check dan decrement harus berada dalam lock yang sama:

```rust
let mut inventory = inventory.lock().unwrap();
let stock = inventory.entry(request.sku.clone()).or_insert(0);

if *stock >= request.qty {
    *stock -= request.qty;
    // fulfilled
} else {
    // rejected
}
```

Jika check dan decrement dipisah tanpa lock yang sama, dua thread bisa sama-sama melihat stock cukup dan menyebabkan oversell.

## 7. Tutup Sender dan Join

Main thread masih memegang `tx` original. Jika tidak di-drop, `rx.into_iter()` bisa menunggu sender yang tidak pernah dipakai.

```rust
drop(tx);
```

Lalu tunggu semua worker:

```rust
for handle in handles {
    handle.join().unwrap();
}
```

## 8. Bangun Report

Kumpulkan receipts:

```rust
let mut receipts: Vec<Receipt> = rx.into_iter().collect();
receipts.sort_by(|a, b| a.request_id.cmp(&b.request_id));
```

Sort bukan bagian concurrency, tetapi membuat output deterministic. Ini penting karena urutan thread tidak dijamin.

Hitung count:

```rust
let fulfilled_count = receipts
    .iter()
    .filter(|receipt| receipt.status == ReceiptStatus::Fulfilled)
    .count();
let rejected_count = receipts.len() - fulfilled_count;
```

Ambil inventory akhir:

```rust
let remaining_inventory = Arc::try_unwrap(inventory)
    .unwrap()
    .into_inner()
    .unwrap();
```

`Arc::try_unwrap` berhasil karena semua worker sudah selesai dan clone `Arc` mereka sudah drop.

## 9. Kenapa Solusi Ini Aman?

- Tidak ada data race karena mutation inventory selalu lewat `Mutex`.
- Tidak oversell karena check stock dan decrement berada dalam critical section yang sama.
- Tidak ada receipt hilang karena setiap request menghasilkan tepat satu `send`.
- Main thread tidak selesai terlalu cepat karena semua worker di-`join`.
- Message passing dipakai untuk output sehingga tidak perlu shared mutable `Vec<Receipt>`.

## 10. Kesalahan yang Sering Terjadi

- Memakai `Mutex<HashMap<...>>` tanpa `Arc`, lalu value moved ke worker pertama.
- Memakai `Rc` untuk thread, yang akan ditolak compiler.
- Lupa `join`, sehingga report bisa dibuat sebelum worker selesai.
- Lupa `drop(tx)`, lalu receiver menunggu terus.
- Lock inventory terlalu lama atau nested lock tanpa alasan.

## 11. Cara Debug Jika Test Gagal

Mulai dari test yang paling sederhana.

Jika `treats_zero_workers_as_one_worker` gagal, cek:

- apakah `workers.max(1)` sudah dipakai;
- apakah pembagian bucket membuat request tetap diproses saat worker input nol.

Jika `does_not_oversell_when_many_threads_hit_same_sku` gagal, cek:

- apakah check stock dan decrement berada dalam satu lock;
- apakah kamu pernah membaca stock di luar lock lalu mengurangi di lock lain;
- apakah setiap request menghasilkan tepat satu receipt.

Jika count salah, cek:

- `fulfilled_count` dihitung dari status receipt;
- `rejected_count = receipts.len() - fulfilled_count`;
- invalid quantity tidak mengurangi inventory.

Jika program hang, cek:

- `drop(tx)` setelah semua worker dibuat;
- semua worker sender adalah clone dari `tx`;
- receiver tidak menunggu sender original yang masih hidup.

## 12. Mental Model Singkat

Bayangkan inventory adalah satu buku stok fisik di meja kasir. Banyak worker boleh menerima order, tetapi hanya satu worker yang boleh membuka dan mencoret buku stok pada satu waktu. Itulah peran `Mutex`.

Receipt tidak perlu ditulis ke buku stok. Receipt cukup dikirim ke main thread lewat channel. Itulah peran `mpsc`.

`Arc` adalah daftar pemegang kunci menuju buku stok yang sama. `Arc::clone` bukan menggandakan buku stok, hanya memberi pegangan tambahan.

## 13. Variasi Latihan Setelah Pass

Setelah solution pass, coba modifikasi sendiri:

1. Tambah field `reason: String` pada `Receipt`.
2. Tambah SKU baru pada sample inventory dan test.
3. Ubah pembagian request dari round-robin menjadi chunk per worker.
4. Tambah test bahwa `worker_id` selalu kurang dari jumlah worker.

Jangan lakukan variasi ini sebelum memahami solution dasar, karena variasi bisa mengaburkan konsep utama.

## 14. Checklist Sebelum Submit Quiz

- `cargo test` pass.
- Tidak ada `todo!()` atau `unimplemented!()`.
- Semua thread di-`join`.
- Semua sender original di-drop jika memakai `rx.into_iter()`.
- Tidak memakai `Rc` untuk thread.
- Tidak meng-clone seluruh inventory untuk setiap worker.
- Essay menyebut `Arc` untuk shared ownership dan `Mutex` untuk mutual exclusion.
