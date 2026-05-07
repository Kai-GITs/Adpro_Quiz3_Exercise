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
        let valid = !request.payment_id.trim().is_empty()
            && !request.user_id.trim().is_empty()
            && request.amount > 0;

        if !valid {
            return PaymentReply {
                payment_id: request.payment_id,
                accepted: false,
                message: "invalid payment".to_string(),
            };
        }

        let event = TransactionEvent {
            transaction_id: format!("TX-{}", request.payment_id),
            user_id: request.user_id,
            amount: request.amount,
            status: "SETTLED".to_string(),
        };

        self.transactions
            .lock()
            .expect("transactions mutex should not be poisoned")
            .push(event);

        PaymentReply {
            payment_id: request.payment_id,
            accepted: true,
            message: "payment accepted".to_string(),
        }
    }

    pub fn transaction_events(&self, query: TransactionQuery) -> Vec<TransactionEvent> {
        let limit = query.limit as usize;
        let events = self
            .transactions
            .lock()
            .expect("transactions mutex should not be poisoned");

        let filtered = events
            .iter()
            .filter(|event| event.user_id == query.user_id)
            .cloned();

        if limit == 0 {
            filtered.collect()
        } else {
            filtered.take(limit).collect()
        }
    }

    pub fn chat_responses(&self, messages: Vec<ChatMessage>) -> Vec<ChatMessage> {
        messages
            .into_iter()
            .map(|message| self.chat_response(message))
            .collect()
    }

    fn chat_response(&self, message: ChatMessage) -> ChatMessage {
        let body = if message.body.trim().is_empty() {
            format!("NACK {}: empty message", message.sender)
        } else {
            format!("ACK {}: {}", message.sender, message.body)
        };

        ChatMessage {
            sender: "server".to_string(),
            body,
        }
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
        let mut responses = Vec::new();

        while let Some(message) = inbound.next().await {
            responses.push(Ok(self.chat_response(message?)));
        }

        Ok(Response::new(Box::pin(tokio_stream::iter(responses))))
    }
}
