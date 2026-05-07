use exercise_2::{ChatMessage, PaymentRequest, QuizApp, TransactionQuery};

#[test]
fn submit_payment_accepts_valid_payment_and_records_transaction() {
    let app = QuizApp::with_seed_data();

    let reply = app.submit_payment_logic(PaymentRequest {
        payment_id: "P-100".to_string(),
        user_id: "u1".to_string(),
        amount: 75_000,
    });

    assert!(reply.accepted);
    assert_eq!(reply.payment_id, "P-100");
    assert_eq!(reply.message, "payment accepted");

    let events = app.transaction_events(TransactionQuery {
        user_id: "u1".to_string(),
        limit: 0,
    });

    assert!(events.iter().any(|event| event.transaction_id == "TX-P-100"
        && event.amount == 75_000
        && event.status == "SETTLED"));
}

#[test]
fn submit_payment_rejects_invalid_payment() {
    let app = QuizApp::with_seed_data();

    let reply = app.submit_payment_logic(PaymentRequest {
        payment_id: "P-101".to_string(),
        user_id: "u1".to_string(),
        amount: 0,
    });

    assert!(!reply.accepted);
    assert_eq!(reply.message, "invalid payment");

    let events = app.transaction_events(TransactionQuery {
        user_id: "u1".to_string(),
        limit: 0,
    });
    assert!(!events
        .iter()
        .any(|event| event.transaction_id == "TX-P-101"));
}

#[test]
fn transaction_events_filter_by_user_and_respect_limit() {
    let app = QuizApp::with_seed_data();

    let all_u1 = app.transaction_events(TransactionQuery {
        user_id: "u1".to_string(),
        limit: 0,
    });
    assert_eq!(all_u1.len(), 2);
    assert!(all_u1.iter().all(|event| event.user_id == "u1"));

    let limited = app.transaction_events(TransactionQuery {
        user_id: "u1".to_string(),
        limit: 1,
    });
    assert_eq!(limited.len(), 1);
    assert_eq!(limited[0].transaction_id, "TX-001");
}

#[test]
fn chat_responses_ack_each_message_and_nack_empty_body() {
    let app = QuizApp::with_seed_data();
    let responses = app.chat_responses(vec![
        ChatMessage {
            sender: "client-a".to_string(),
            body: "hello".to_string(),
        },
        ChatMessage {
            sender: "client-b".to_string(),
            body: "".to_string(),
        },
    ]);

    assert_eq!(responses.len(), 2);
    assert_eq!(responses[0].sender, "server");
    assert_eq!(responses[0].body, "ACK client-a: hello");
    assert_eq!(responses[1].sender, "server");
    assert_eq!(responses[1].body, "NACK client-b: empty message");
}

#[test]
fn transaction_events_return_empty_vec_for_unknown_user() {
    let app = QuizApp::with_seed_data();

    let events = app.transaction_events(TransactionQuery {
        user_id: "missing-user".to_string(),
        limit: 0,
    });

    assert!(events.is_empty());
}
