# Jawaban Essay - Exercise 0

## 1. Shared Memory dan Message Passing

Shared memory model muncul pada `Arc<Mutex<Summary>>`. Semua worker punya handle ke summary yang sama, lalu mengunci `Mutex` saat update count. Tanpa `Mutex`, update count bisa race dan menghasilkan lost update.

Message passing model muncul pada `mpsc::channel`. Worker mengirim progress message ke main thread setiap selesai memproses satu log. Main thread tidak perlu membaca state internal worker; ia cukup menerima message.

## 2. Instant sebagai Profiling Sederhana

`Instant` bukan profiler lengkap karena hanya memberi durasi total suatu blok kerja. Ia tidak memberi tahu function mana yang paling mahal, berapa allocation, atau apakah ada lock contention.

Namun `Instant` tetap berguna untuk profiling sederhana karena cepat dipakai dan cukup untuk membandingkan perubahan kasar. Misalnya, setelah mengubah cara memproses log, kita bisa melihat apakah durasi total naik atau turun.

## 3. gRPC Streaming vs REST Unary

`WatchTransactions` cocok untuk server streaming karena satu request bisa menghasilkan banyak transaction event. Jika memakai REST unary biasa, server biasanya harus mengirim semua data sekaligus atau client melakukan polling.

`Chat` cocok untuk bidirectional streaming karena client dan server sama-sama perlu mengirim message secara berkelanjutan. REST unary kurang natural untuk percakapan dua arah karena model dasarnya request-response tunggal.

## 4. Output Deterministic dalam Program Concurrent

Concurrency membuat urutan eksekusi worker tidak deterministik. Jika output bergantung pada urutan worker, test bisa kadang pass dan kadang fail. Karena itu report harus berbasis aggregate count yang deterministic, bukan urutan pemrosesan thread. Jika progress message dicetak untuk debug, message sebaiknya disort sebelum dicetak.
