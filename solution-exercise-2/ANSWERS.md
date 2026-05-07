# Jawaban Essay - Exercise 2

## 1. Mengapa gRPC Cocok untuk Microservice

gRPC cocok untuk komunikasi antar microservice karena interface didefinisikan eksplisit lewat protobuf. Service, RPC method, dan message schema menjadi kontrak yang bisa di-generate ke banyak bahasa. Ini mengurangi ambiguitas dibanding REST yang sering bergantung pada dokumentasi endpoint dan JSON manual.

Untuk kasus yang butuh streaming, gRPC juga lebih natural karena mendukung unary, server streaming, client streaming, dan bidirectional streaming. REST umumnya request-response unary, sehingga streaming biasanya perlu mekanisme tambahan seperti polling, SSE, atau WebSocket.

## 2. Peran Protobuf

Protobuf mendefinisikan message dan service contract. File `.proto` diproses menjadi type Rust oleh `prost` dan service trait oleh `tonic-build`.

Protobuf sering lebih efisien daripada JSON karena formatnya binary, field direpresentasikan dengan tag number, dan tidak perlu mengirim nama field sebagai text setiap kali. JSON lebih mudah dibaca manusia, tetapi biasanya lebih besar dan butuh parsing text.

## 3. HTTP/2, Multiplexing, dan Streaming

gRPC berjalan di atas HTTP/2. HTTP/2 mendukung multiplexing, yaitu beberapa stream request-response bisa berjalan di satu koneksi yang sama tanpa harus saling menunggu seperti pola lama yang lebih blocking. Ini membantu service menangani banyak RPC secara efisien.

Streaming di gRPC memanfaatkan kemampuan HTTP/2 stream. Karena itu server bisa mengirim banyak response untuk satu request, client bisa mengirim banyak request message, atau keduanya bisa saling mengirim message dalam bidirectional streaming.

## 4. Tonic, Tokio, dan Async/Await

Tonic memakai async Rust agar I/O network tidak memblokir thread saat menunggu data dari socket. `tokio` menyediakan runtime untuk menjalankan future, scheduler task, timer, dan async I/O. Dengan async/await, satu thread runtime bisa menangani banyak koneksi yang sedang menunggu I/O, sehingga throughput lebih baik untuk service network.

Async bukan berarti semua hal otomatis parallel CPU-bound. Jika pekerjaannya CPU-heavy, kita tetap perlu strategi lain seperti task blocking pool, worker pool, atau optimasi algoritma.

## 5. Performance Testing vs Profiling

Performance testing melihat performa dari luar: response time, throughput, error rate, dan behavior saat load tertentu. Tool seperti JMeter cocok untuk mengirim banyak request dan mengukur gejala.

Profiling melihat dari dalam program: function mana yang paling mahal, CPU time, allocation, lock contention, atau memory usage. Jika `WatchTransactions` lambat, mulai dari performance test untuk mereproduksi load, lalu profiling untuk mencari bottleneck. Kandidat masalahnya bisa filtering vector terlalu besar, lock terlalu lama, serialization cost, atau stream yang menunggu consumer lambat.
