use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

use solution_exercise_0::{format_report, is_streaming, sample_logs, Protocol, Summary};

fn main() {
    let started = Instant::now();
    let logs = sample_logs();
    let summary = Arc::new(Mutex::new(Summary::default()));
    let (tx, rx) = mpsc::channel();
    let worker_count = 2;
    let mut buckets = vec![Vec::new(); worker_count];

    for (idx, log) in logs.into_iter().enumerate() {
        buckets[idx % worker_count].push(log);
    }

    let mut handles = Vec::with_capacity(worker_count);
    for (worker_id, bucket) in buckets.into_iter().enumerate() {
        let summary = Arc::clone(&summary);
        let tx = tx.clone();

        let handle = thread::spawn(move || {
            for log in bucket {
                {
                    let mut summary = summary
                        .lock()
                        .expect("summary mutex should not be poisoned");
                    summary.total += 1;
                    summary.total_latency_ms += log.latency_ms;

                    match log.protocol {
                        Protocol::Rest => summary.rest += 1,
                        Protocol::Grpc => summary.grpc += 1,
                    }

                    if is_streaming(log.interaction) {
                        summary.streaming += 1;
                    }

                    if log.success {
                        summary.success += 1;
                    } else {
                        summary.failed += 1;
                    }
                }

                tx.send(format!("worker-{worker_id}:{}", log.id))
                    .expect("main receiver should still exist");
            }
        });
        handles.push(handle);
    }

    drop(tx);

    for handle in handles {
        handle.join().expect("worker thread should not panic");
    }

    let mut worker_messages: Vec<String> = rx.into_iter().collect();
    worker_messages.sort();

    let summary = summary
        .lock()
        .expect("summary mutex should not be poisoned")
        .clone();
    let elapsed_ms = started.elapsed().as_millis();

    println!(
        "{}",
        format_report(&summary, worker_messages.len(), elapsed_ms)
    );
}
