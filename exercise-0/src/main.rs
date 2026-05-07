use exercise_0::{format_report, sample_logs, Summary};

fn main() {
    // TODO guide:
    // 1. Start profiling with `Instant::now()`.
    // 2. Load logs with `sample_logs()`.
    // 3. Create `Arc<Mutex<Summary>>`.
    // 4. Create `mpsc::channel` for worker progress messages.
    // 5. Split logs into at least 2 worker buckets.
    // 6. Spawn worker threads with `move` closures.
    // 7. Update summary counts inside the mutex.
    // 8. Send one progress message per processed log.
    // 9. Join workers, collect messages, then print `format_report`.
    let _ = (sample_logs, format_report);
    let _summary = Summary::default();
    todo!("implement the mini analyzer in main only")
}
