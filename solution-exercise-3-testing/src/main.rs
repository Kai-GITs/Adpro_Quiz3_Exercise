use solution_exercise_3_testing::{checkout, CartItem};

fn main() {
    let items = vec![
        CartItem {
            sku: "book".to_string(),
            unit_price: 40_000,
            qty: 2,
        },
        CartItem {
            sku: "pen".to_string(),
            unit_price: 5_000,
            qty: 3,
        },
    ];

    let summary = checkout(&items, 10, false);
    println!("{summary:#?}");
}
