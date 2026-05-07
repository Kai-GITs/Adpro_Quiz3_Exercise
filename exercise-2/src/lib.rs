use std::pin::Pin;
use std::sync::{Arc, Mutex};

use futures_core::Stream;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status};

pub mod quizprep {
    tonic::include_proto!("quizprep");
}

pub use quizprep::{ChatMessage, PaymentReply, PaymentRequest, TransactionEvent, TransactionQuery};

use quizprep::quiz_service_server::QuizService;

type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + 'static>>;

#[derive(Debug, Clone, Default)]
pub struct QuizApp {
    // Shared state untuk service. Walaupun Tonic handler berjalan async,
    // exercise ini sengaja memakai Mutex sederhana agar hubungan dengan
    // materi shared memory model tetap terlihat.
    transactions: Arc<Mutex<Vec<TransactionEvent>>>,
}

impl QuizApp {
    pub fn new(transactions: Vec<TransactionEvent>) -> Self {
        Self {
            transactions: Arc::new(Mutex::new(transactions)),
        }
    }

    pub fn with_seed_data() -> Self {
        Self::new(vec![
            TransactionEvent {
                transaction_id: "TX-001".to_string(),
                user_id: "u1".to_string(),
                amount: 10_000,
                status: "SETTLED".to_string(),
            },
            TransactionEvent {
                transaction_id: "TX-002".to_string(),
                user_id: "u2".to_string(),
                amount: 5_000,
                status: "SETTLED".to_string(),
            },
            TransactionEvent {
                transaction_id: "TX-003".to_string(),
                user_id: "u1".to_string(),
                amount: 7_000,
                status: "REFUNDED".to_string(),
            },
        ])
    }

    pub fn submit_payment_logic(&self, request: PaymentRequest) -> PaymentReply {
        // TODO guide:
        // 1. Validate payment_id, user_id, and amount.
        // 2. Return rejected reply for invalid request.
        // 3. For valid request, lock transactions briefly and push SETTLED event.
        // 4. Return accepted reply.
        let _ = &self.transactions;
        let _ = request;
        todo!("validate payment and record a SETTLED transaction when accepted")
    }

    pub fn transaction_events(&self, query: TransactionQuery) -> Vec<TransactionEvent> {
        // TODO guide:
        // 1. Lock transactions and iterate over existing events.
        // 2. Filter by query.user_id.
        // 3. If limit == 0 collect all; otherwise take only `limit` events.
        let _ = &self.transactions;
        let _ = query;
        todo!("filter transaction events by user_id and limit")
    }

    pub fn chat_responses(&self, messages: Vec<ChatMessage>) -> Vec<ChatMessage> {
        // TODO guide:
        // Map every inbound message to a server response.
        // Empty body -> NACK, non-empty body -> ACK.
        let _ = messages;
        todo!("produce ACK/NACK response for every inbound chat message")
    }
}

#[tonic::async_trait]
impl QuizService for QuizApp {
    type WatchTransactionsStream = ResponseStream<TransactionEvent>;
    type ChatStream = ResponseStream<ChatMessage>;

    async fn submit_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentReply>, Status> {
        let reply = self.submit_payment_logic(request.into_inner());
        Ok(Response::new(reply))
    }

    async fn watch_transactions(
        &self,
        request: Request<TransactionQuery>,
    ) -> Result<Response<Self::WatchTransactionsStream>, Status> {
        let stream = tokio_stream::iter(
            self.transaction_events(request.into_inner())
                .into_iter()
                .map(Ok),
        );
        Ok(Response::new(Box::pin(stream)))
    }

    async fn chat(
        &self,
        request: Request<tonic::Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        let mut inbound = request.into_inner();
        let mut messages = Vec::new();

        while let Some(message) = inbound.next().await {
            messages.push(message?);
        }

        let stream = tokio_stream::iter(self.chat_responses(messages).into_iter().map(Ok));
        Ok(Response::new(Box::pin(stream)))
    }
}
