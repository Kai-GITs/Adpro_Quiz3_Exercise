pub const FREE_SHIPPING_THRESHOLD: u32 = 100_000;
pub const REGULAR_SHIPPING_FEE: u32 = 15_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartItem {
    pub sku: String,
    pub unit_price: u32,
    pub qty: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckoutSummary {
    pub subtotal: u32,
    pub discount: u32,
    pub shipping_fee: u32,
    pub total: u32,
}

pub fn calculate_subtotal(items: &[CartItem]) -> u32 {
    items
        .iter()
        .map(|item| item.unit_price.saturating_mul(item.qty))
        .sum()
}

pub fn discount_amount(subtotal: u32, discount_percent: u32) -> u32 {
    let percent = discount_percent.min(100);
    subtotal.saturating_mul(percent) / 100
}

pub fn shipping_fee(after_discount: u32, is_member: bool) -> u32 {
    if is_member || after_discount >= FREE_SHIPPING_THRESHOLD {
        0
    } else {
        REGULAR_SHIPPING_FEE
    }
}

pub fn checkout(items: &[CartItem], discount_percent: u32, is_member: bool) -> CheckoutSummary {
    let subtotal = calculate_subtotal(items);
    let discount = discount_amount(subtotal, discount_percent);
    let after_discount = subtotal.saturating_sub(discount);
    let shipping_fee = shipping_fee(after_discount, is_member);
    let total = after_discount + shipping_fee;

    CheckoutSummary {
        subtotal,
        discount,
        shipping_fee,
        total,
    }
}
