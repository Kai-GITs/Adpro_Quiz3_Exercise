use solution_exercise_3_testing::{
    calculate_subtotal, checkout, discount_amount, shipping_fee, CartItem, REGULAR_SHIPPING_FEE,
};

fn item(sku: &str, unit_price: u32, qty: u32) -> CartItem {
    CartItem {
        sku: sku.to_string(),
        unit_price,
        qty,
    }
}

#[test]
fn subtotal_sums_multiple_items() {
    let items = vec![item("book", 40_000, 2), item("pen", 5_000, 3)];

    let subtotal = calculate_subtotal(&items);

    assert_eq!(subtotal, 95_000);
}

#[test]
fn zero_quantity_item_does_not_increase_subtotal() {
    let items = vec![item("book", 40_000, 2), item("bag", 200_000, 0)];

    let subtotal = calculate_subtotal(&items);

    assert_eq!(subtotal, 80_000);
}

#[test]
fn discount_amount_uses_percentage_of_subtotal() {
    let discount = discount_amount(95_000, 10);

    assert_eq!(discount, 9_500);
}

#[test]
fn discount_more_than_100_percent_is_capped() {
    let discount = discount_amount(80_000, 150);

    assert_eq!(discount, 80_000);
}

#[test]
fn shipping_is_free_for_member() {
    let fee = shipping_fee(10_000, true);

    assert_eq!(fee, 0);
}

#[test]
fn shipping_is_free_when_after_discount_reaches_threshold() {
    let fee = shipping_fee(100_000, false);

    assert_eq!(fee, 0);
}

#[test]
fn shipping_is_regular_for_non_member_below_threshold() {
    let fee = shipping_fee(99_999, false);

    assert_eq!(fee, REGULAR_SHIPPING_FEE);
}

#[test]
fn checkout_returns_complete_summary() {
    let items = vec![item("book", 40_000, 2), item("pen", 5_000, 3)];

    let summary = checkout(&items, 10, false);

    assert_eq!(summary.subtotal, 95_000);
    assert_eq!(summary.discount, 9_500);
    assert_eq!(summary.shipping_fee, 15_000);
    assert_eq!(summary.total, 100_500);
}
