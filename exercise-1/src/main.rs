use exercise_1::{process_batch, sample_inventory, sample_requests};

fn main() {
    let report = process_batch(sample_inventory(), sample_requests(), 3);
    println!("{report:#?}");
}
