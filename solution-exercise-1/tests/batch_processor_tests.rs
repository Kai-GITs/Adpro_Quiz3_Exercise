use std::collections::HashMap;

use solution_exercise_1::{process_batch, ReceiptStatus, Request};

fn req(id: &str, sku: &str, qty: u32) -> Request {
    Request {
        id: id.to_string(),
        customer: format!("customer-{id}"),
        sku: sku.to_string(),
        qty,
    }
}

#[test]
fn processes_valid_invalid_and_out_of_stock_requests() {
    let inventory = HashMap::from([
        ("book".to_string(), 5),
        ("pen".to_string(), 3),
        ("sticker".to_string(), 10),
    ]);
    let requests = vec![
        req("R-001", "book", 2),
        req("R-002", "pen", 1),
        req("R-003", "book", 3),
        req("R-004", "pen", 0),
        req("R-005", "laptop", 1),
    ];

    let mut report = process_batch(inventory, requests, 3);
    report
        .receipts
        .sort_by(|a, b| a.request_id.cmp(&b.request_id));

    assert_eq!(report.fulfilled_count, 3);
    assert_eq!(report.rejected_count, 2);
    assert_eq!(report.remaining_inventory.get("book"), Some(&0));
    assert_eq!(report.remaining_inventory.get("pen"), Some(&2));
    assert_eq!(report.remaining_inventory.get("sticker"), Some(&10));

    assert_eq!(report.receipts[0].status, ReceiptStatus::Fulfilled);
    assert_eq!(report.receipts[1].status, ReceiptStatus::Fulfilled);
    assert_eq!(report.receipts[2].status, ReceiptStatus::Fulfilled);
    assert_eq!(
        report.receipts[3].status,
        ReceiptStatus::RejectedInvalidQuantity
    );
    assert_eq!(report.receipts[4].status, ReceiptStatus::RejectedOutOfStock);
}

#[test]
fn treats_zero_workers_as_one_worker() {
    let inventory = HashMap::from([("book".to_string(), 2)]);
    let requests = vec![req("R-001", "book", 1), req("R-002", "book", 1)];

    let report = process_batch(inventory, requests, 0);

    assert_eq!(report.fulfilled_count, 2);
    assert_eq!(report.rejected_count, 0);
    assert_eq!(report.remaining_inventory.get("book"), Some(&0));
}

#[test]
fn does_not_oversell_when_many_threads_hit_same_sku() {
    let inventory = HashMap::from([("ticket".to_string(), 4)]);
    let requests = (0..20)
        .map(|i| req(&format!("R-{i:03}"), "ticket", 1))
        .collect();

    let report = process_batch(inventory, requests, 8);

    assert_eq!(report.fulfilled_count, 4);
    assert_eq!(report.rejected_count, 16);
    assert_eq!(report.remaining_inventory.get("ticket"), Some(&0));
}

#[test]
fn empty_requests_keep_inventory_unchanged() {
    let inventory = HashMap::from([("book".to_string(), 2), ("pen".to_string(), 8)]);

    let report = process_batch(inventory.clone(), vec![], 4);

    assert_eq!(report.fulfilled_count, 0);
    assert_eq!(report.rejected_count, 0);
    assert!(report.receipts.is_empty());
    assert_eq!(report.remaining_inventory, inventory);
}
