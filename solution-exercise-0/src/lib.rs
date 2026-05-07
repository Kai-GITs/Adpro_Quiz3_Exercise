#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Rest,
    Grpc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interaction {
    Unary,
    ServerStreaming,
    BidirectionalStreaming,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestLog {
    pub id: &'static str,
    pub service: &'static str,
    pub protocol: Protocol,
    pub interaction: Interaction,
    pub latency_ms: u32,
    pub success: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Summary {
    pub total: u32,
    pub rest: u32,
    pub grpc: u32,
    pub streaming: u32,
    pub success: u32,
    pub failed: u32,
    pub total_latency_ms: u32,
}

pub fn sample_logs() -> Vec<RequestLog> {
    vec![
        RequestLog {
            id: "REQ-001",
            service: "catalog",
            protocol: Protocol::Rest,
            interaction: Interaction::Unary,
            latency_ms: 20,
            success: true,
        },
        RequestLog {
            id: "REQ-002",
            service: "order",
            protocol: Protocol::Rest,
            interaction: Interaction::Unary,
            latency_ms: 55,
            success: true,
        },
        RequestLog {
            id: "REQ-003",
            service: "logging",
            protocol: Protocol::Rest,
            interaction: Interaction::Unary,
            latency_ms: 35,
            success: false,
        },
        RequestLog {
            id: "REQ-004",
            service: "payment",
            protocol: Protocol::Grpc,
            interaction: Interaction::Unary,
            latency_ms: 40,
            success: true,
        },
        RequestLog {
            id: "REQ-005",
            service: "transaction-history",
            protocol: Protocol::Grpc,
            interaction: Interaction::ServerStreaming,
            latency_ms: 70,
            success: true,
        },
        RequestLog {
            id: "REQ-006",
            service: "chat",
            protocol: Protocol::Grpc,
            interaction: Interaction::BidirectionalStreaming,
            latency_ms: 85,
            success: true,
        },
        RequestLog {
            id: "REQ-007",
            service: "recommendation",
            protocol: Protocol::Grpc,
            interaction: Interaction::ServerStreaming,
            latency_ms: 60,
            success: false,
        },
        RequestLog {
            id: "REQ-008",
            service: "payment",
            protocol: Protocol::Grpc,
            interaction: Interaction::Unary,
            latency_ms: 35,
            success: true,
        },
    ]
}

pub fn is_streaming(interaction: Interaction) -> bool {
    !matches!(interaction, Interaction::Unary)
}

pub fn average_latency_ms(summary: &Summary) -> u32 {
    if summary.total == 0 {
        0
    } else {
        summary.total_latency_ms / summary.total
    }
}

pub fn format_report(summary: &Summary, worker_messages: usize, elapsed_ms: u128) -> String {
    format!(
        "SERVICE_TRAFFIC_REPORT\n\
         TOTAL={}\n\
         REST={}\n\
         GRPC={}\n\
         STREAMING={}\n\
         SUCCESS={}\n\
         FAILED={}\n\
         AVG_LATENCY_MS={}\n\
         WORKER_MESSAGES={}\n\
         PROFILE_ELAPSED_MS={}",
        summary.total,
        summary.rest,
        summary.grpc,
        summary.streaming,
        summary.success,
        summary.failed,
        average_latency_ms(summary),
        worker_messages,
        elapsed_ms
    )
}
