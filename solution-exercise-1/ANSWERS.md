# Jawaban Essay - Exercise 1

## 1. Shared Memory Model vs Message Passing Model

Shared memory model berarti beberapa thread mengakses data yang sama di memory. Karena data yang sama bisa dibaca atau ditulis oleh beberapa thread, bagian yang mengubah data harus dilindungi synchronization primitive seperti `Mutex`. Dalam project ini, inventory adalah shared memory karena semua worker mengakses `Arc<Mutex<HashMap<String, u32>>>`.

Message passing model berarti thread tidak berbagi state utama secara langsung, tetapi mengirim pesan melalui channel. Dalam project ini, worker mengirim `Receipt` ke main thread lewat `mpsc::channel`. Jadi inventory memakai shared memory, sedangkan hasil pemrosesan memakai message passing.

## 2. Mengapa Perlu Arc

`thread::spawn` membutuhkan data yang dipakai thread memiliki lifetime cukup panjang dan ownership yang jelas. `Mutex<HashMap<...>>` sendiri hanya punya satu owner. Agar banyak thread bisa memiliki handle ke data yang sama, kita bungkus dengan `Arc`, yaitu atomic reference counting yang thread-safe.

`Rc` tidak tepat karena reference count milik `Rc` tidak atomic dan tidak aman dipakai lintas thread. Rust mencegah penggunaan `Rc` di thread karena `Rc` tidak mengimplementasikan `Send` dan `Sync` untuk kebutuhan tersebut.

## 3. Race Condition, Lost Update, Critical Section, Mutex

Race condition terjadi ketika hasil program bergantung pada urutan eksekusi thread yang tidak deterministik. Lost update bisa terjadi jika dua thread membaca stok yang sama lalu sama-sama menulis hasil pengurangan, sehingga salah satu update hilang. Critical section pada project ini adalah bagian saat worker mengecek stok dan mengurangi stok. `Mutex` membuat hanya satu thread yang masuk critical section tersebut pada satu waktu.

## 4. Mengapa thread::spawn Membutuhkan move

Closure pada `thread::spawn` bisa hidup lebih lama daripada function yang membuatnya. Jika closure hanya meminjam variable lokal, reference itu bisa menjadi tidak valid saat thread masih berjalan. Keyword `move` memindahkan ownership data yang dibutuhkan ke dalam closure, sehingga thread punya data sendiri atau handle sendiri seperti `Arc` clone.

## 5. Contoh Deadlock

Misalnya nanti ada dua shared resource: `inventory` dan `payment_log`. Worker A mengunci `inventory` lalu menunggu `payment_log`, sementara Worker B mengunci `payment_log` lalu menunggu `inventory`. Keduanya saling menunggu dan tidak ada yang bisa lanjut. Cara mengurangi risiko ini adalah menetapkan urutan lock yang konsisten dan menjaga durasi lock sesingkat mungkin.
