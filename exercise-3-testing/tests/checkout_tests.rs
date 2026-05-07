use exercise_3_testing::CartItem;

fn item(sku: &str, unit_price: u32, qty: u32) -> CartItem {
    CartItem {
        sku: sku.to_string(),
        unit_price,
        qty,
    }
}

#[test]
fn replace_this_placeholder_with_checkout_tests() {
    let _items = vec![item("book", 40_000, 2)];

    todo!("write at least 6 focused tests based on SOAL.md")
}
