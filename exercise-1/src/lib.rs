use std::collections::HashMap;

/// Order request yang harus diproses oleh worker.
///
/// `id` harus muncul lagi di `Receipt.request_id`, sedangkan `sku` dan `qty`
/// dipakai untuk mengecek dan mengurangi inventory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub id: String,
    pub customer: String,
    pub sku: String,
    pub qty: u32,
}

/// Status akhir untuk satu request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiptStatus {
    Fulfilled,
    RejectedOutOfStock,
    RejectedInvalidQuantity,
}

/// Hasil pemrosesan satu request.
///
/// `worker_id` dipakai test untuk memastikan hasil memang berasal dari worker
/// tertentu, tetapi urutan receipt tidak boleh diasumsikan karena concurrency
/// bersifat nondeterministic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    pub request_id: String,
    pub sku: String,
    pub accepted_qty: u32,
    pub status: ReceiptStatus,
    pub worker_id: usize,
}

/// Ringkasan akhir batch processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchReport {
    pub receipts: Vec<Receipt>,
    pub remaining_inventory: HashMap<String, u32>,
    pub fulfilled_count: usize,
    pub rejected_count: usize,
}

pub fn sample_inventory() -> HashMap<String, u32> {
    HashMap::from([
        ("book".to_string(), 5),
        ("pen".to_string(), 3),
        ("sticker".to_string(), 10),
    ])
}

pub fn sample_requests() -> Vec<Request> {
    vec![
        Request {
            id: "R-001".to_string(),
            customer: "Alya".to_string(),
            sku: "book".to_string(),
            qty: 2,
        },
        Request {
            id: "R-002".to_string(),
            customer: "Bima".to_string(),
            sku: "pen".to_string(),
            qty: 1,
        },
        Request {
            id: "R-003".to_string(),
            customer: "Citra".to_string(),
            sku: "book".to_string(),
            qty: 3,
        },
        Request {
            id: "R-004".to_string(),
            customer: "Dewa".to_string(),
            sku: "pen".to_string(),
            qty: 0,
        },
        Request {
            id: "R-005".to_string(),
            customer: "Eka".to_string(),
            sku: "laptop".to_string(),
            qty: 1,
        },
    ]
}

pub fn process_batch(
    initial_inventory: HashMap<String, u32>,
    requests: Vec<Request>,
    workers: usize,
) -> BatchReport {
    // TODO guide:
    // 1. Normalize worker count with `workers.max(1)`.
    // 2. Wrap `initial_inventory` with `Arc<Mutex<_>>`.
    // 3. Split requests into worker buckets so each request is processed once.
    // 4. Create an `mpsc::channel` for worker -> main thread receipts.
    // 5. Spawn worker threads with `thread::spawn(move || { ... })`.
    // 6. Inside each worker, lock inventory only while checking/updating stock.
    // 7. Send `Receipt` through the channel.
    // 8. Drop the original sender, join all handles, collect receipts.
    // 9. Build `BatchReport` from receipts and remaining inventory.
    let _ = (initial_inventory, requests, workers);
    todo!("implement concurrent processing with Arc<Mutex<_>>, thread::spawn, mpsc, and join")
}
