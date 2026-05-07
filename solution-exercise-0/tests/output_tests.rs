use std::process::Command;

#[test]
fn binary_prints_expected_summary() {
    let output = Command::new(env!("CARGO_BIN_EXE_solution-exercise-0"))
        .output()
        .expect("binary should run");

    assert!(
        output.status.success(),
        "binary failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    for expected in [
        "SERVICE_TRAFFIC_REPORT",
        "TOTAL=8",
        "REST=3",
        "GRPC=5",
        "STREAMING=3",
        "SUCCESS=6",
        "FAILED=2",
        "AVG_LATENCY_MS=50",
        "WORKER_MESSAGES=8",
        "PROFILE_ELAPSED_MS=",
    ] {
        assert!(
            stdout.contains(expected),
            "missing line: {expected}\n{stdout}"
        );
    }
}
