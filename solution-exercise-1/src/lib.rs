use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub id: String,
    pub customer: String,
    pub sku: String,
    pub qty: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiptStatus {
    Fulfilled,
    RejectedOutOfStock,
    RejectedInvalidQuantity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    pub request_id: String,
    pub sku: String,
    pub accepted_qty: u32,
    pub status: ReceiptStatus,
    pub worker_id: usize,
}

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
    let worker_count = workers.max(1);
    let inventory = Arc::new(Mutex::new(initial_inventory));
    let (tx, rx) = mpsc::channel();
    let mut partitions = vec![Vec::new(); worker_count];

    for (idx, request) in requests.into_iter().enumerate() {
        partitions[idx % worker_count].push(request);
    }

    let mut handles = Vec::with_capacity(worker_count);
    for (worker_id, partition) in partitions.into_iter().enumerate() {
        let inventory = Arc::clone(&inventory);
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            for request in partition {
                let receipt = process_one(worker_id, request, &inventory);
                tx.send(receipt).expect("main receiver should still exist");
            }
        });
        handles.push(handle);
    }

    drop(tx);

    for handle in handles {
        handle.join().expect("worker thread should not panic");
    }

    let mut receipts: Vec<Receipt> = rx.into_iter().collect();
    receipts.sort_by(|a, b| a.request_id.cmp(&b.request_id));

    let fulfilled_count = receipts
        .iter()
        .filter(|receipt| receipt.status == ReceiptStatus::Fulfilled)
        .count();
    let rejected_count = receipts.len() - fulfilled_count;

    let remaining_inventory = Arc::try_unwrap(inventory)
        .expect("all worker Arc handles should be dropped")
        .into_inner()
        .expect("inventory mutex should not be poisoned");

    BatchReport {
        receipts,
        remaining_inventory,
        fulfilled_count,
        rejected_count,
    }
}

fn process_one(
    worker_id: usize,
    request: Request,
    inventory: &Arc<Mutex<HashMap<String, u32>>>,
) -> Receipt {
    if request.qty == 0 {
        return Receipt {
            request_id: request.id,
            sku: request.sku,
            accepted_qty: 0,
            status: ReceiptStatus::RejectedInvalidQuantity,
            worker_id,
        };
    }

    let mut inventory = inventory
        .lock()
        .expect("inventory mutex should not be poisoned");
    let stock = inventory.entry(request.sku.clone()).or_insert(0);

    if *stock >= request.qty {
        *stock -= request.qty;
        Receipt {
            request_id: request.id,
            sku: request.sku,
            accepted_qty: request.qty,
            status: ReceiptStatus::Fulfilled,
            worker_id,
        }
    } else {
        Receipt {
            request_id: request.id,
            sku: request.sku,
            accepted_qty: 0,
            status: ReceiptStatus::RejectedOutOfStock,
            worker_id,
        }
    }
}
